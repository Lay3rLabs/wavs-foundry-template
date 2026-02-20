/**
 * Create a new aggregator.
 *
 * Usage:
 * ```
 * pnpm deploy:create-aggregator <index>
 * ```
 */

import fs from 'fs'
import path from 'path'
import readline from 'readline'

import chalk from 'chalk'
import { Command } from 'commander'
import {
  Hex,
  createPublicClient,
  formatEther,
  http,
  parseEther,
  toHex,
} from 'viem'
import { english, generateMnemonic, mnemonicToAccount } from 'viem/accounts'

import { WAVS_DOCKER_IMAGE } from './constants'
import { initProgram } from './env'
import { execSilently, sleep } from './utils'

const program = new Command('create-aggregator')
  .description('Create a new aggregator')
  .argument('[index]', 'The aggregator index (default: 1)')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option(
    '-r, --rpc-url <rpcUrl>',
    'The RPC URL for the chain (default: $RPC_URL from .env)'
  )
  .option('-i, --index <index>', 'The aggregator index (default: 1)', (i) =>
    parseInt(i)
  )
  .option('-f, --force', 'Override existing aggregator without confirming')

const main = async () => {
  const {
    envName,
    options: { index: _index, rpcUrl, force },
  } = initProgram(program)

  const client = createPublicClient({
    transport: http(rpcUrl),
  })

  if (program.args[0] !== undefined && _index !== undefined) {
    throw new Error(
      '❌ Cannot provide two indexes. Please only provide via -i (--index) or the first positional argument.'
    )
  }

  const index = program.args[0]
    ? parseInt(program.args[0])
    : _index === undefined
      ? 1
      : _index
  if (typeof index !== 'number' || isNaN(index) || index < 0) {
    throw new Error('❌ Invalid index. Please provide a valid positive number.')
  }

  console.log(chalk.blueBright(`🤖 Creating aggregator ${index}...`))

  if (!fs.existsSync('.docker')) {
    fs.mkdirSync('.docker')
  }

  const containerName = `wavs-aggregator-${index}`
  const readableAggregatorLocation = `infra/aggregator-${index}`
  const aggregatorLocation = path.join(
    __dirname,
    `../${readableAggregatorLocation}`
  )

  if (fs.existsSync(aggregatorLocation)) {
    if (force) {
      console.log(
        chalk.blueBright(
          `🔥 Destroying existing aggregator at ${readableAggregatorLocation}...`
        )
      )

      // Kill the aggregator container if running, ignore errors
      await execSilently('docker', 'kill', containerName).catch(() => {})

      fs.rmSync(aggregatorLocation, { recursive: true })
    } else {
      throw new Error(
        `❌ Aggregator already exists at ${readableAggregatorLocation}. Use -f (--force) to override.`
      )
    }
  }

  fs.mkdirSync(aggregatorLocation, { recursive: true })

  // If prod deployment, prompt for mnemonic
  let mnemonic
  if (envName === 'prod') {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    })

    mnemonic = await new Promise<string>((resolve) =>
      rl.question(
        'Enter aggregator mnemonic (leave blank to generate a new one): ',
        resolve
      )
    )
  }

  if (!mnemonic) {
    console.log(chalk.blueBright('🔑 Generating new mnemonic...'))
    mnemonic = generateMnemonic(english)
  }

  const account = mnemonicToAccount(mnemonic)
  const privateKey = toHex(account.getHdKey().privateKey!)

  console.log(
    chalk.cyanBright(`💸 Aggregator wallet address: ${account.address}`)
  )

  // Update aggregator env file with new mnemonic and private key.
  fs.writeFileSync(
    path.join(aggregatorLocation, '.env'),
    fs
      .readFileSync(path.join(__dirname, './templates/aggregator.env'), 'utf8')
      .replace('INSERT_MNEMONIC_HERE', mnemonic)
      .replace('INSERT_PRIVATE_KEY_HERE', privateKey)
  )

  // Create startup script
  fs.writeFileSync(
    path.join(aggregatorLocation, 'start.sh'),
    fs
      .readFileSync(
        path.join(__dirname, './templates/aggregator.start.sh'),
        'utf8'
      )
      .replace('INSERT_IMAGE_HERE', WAVS_DOCKER_IMAGE)
      .replace('INSERT_INSTANCE_NAME_HERE', containerName)
  )

  // Copy WAVS config file if it exists
  const wavsTomlSrc = path.join(__dirname, '../wavs.toml')
  if (fs.existsSync(wavsTomlSrc)) {
    fs.copyFileSync(wavsTomlSrc, path.join(aggregatorLocation, 'wavs.toml'))
  }

  // Fund aggregator on dev
  if (envName === 'dev') {
    console.log(chalk.blueBright(`💸 Funding aggregator...`))

    await client.request({
      method: 'anvil_setBalance' as any,
      params: [account.address, ('0x' + parseEther('15').toString(16)) as Hex],
    })
    await sleep(1)

    const balance = await client.getBalance({
      address: account.address,
    })

    console.log(
      chalk.greenBright(`💰 Aggregator funded with ${formatEther(balance)} ETH`)
    )
  } else {
    console.log(chalk.yellowBright(`🔄 Waiting for aggregator to be funded...`))

    while (true) {
      const balance = await client.getBalance({
        address: account.address,
      })

      if (balance !== 0n) {
        console.log(
          chalk.greenBright(
            `💰 Aggregator funded with ${formatEther(balance)} ETH`
          )
        )
        break
      }

      await sleep(3)
    }
  }

  console.log(
    chalk.greenBright(
      `✅ Aggregator ${index} created successfully at ${readableAggregatorLocation}`
    )
  )
}

main().catch((err) => {
  console.error(chalk.redBright(err.message))
  process.exit(1)
})

process.on('SIGINT', () => {
  console.error(chalk.redBright('SIGINT received. Shutting down...'))
  process.exit(1)
})

process.on('SIGTERM', () => {
  console.error(chalk.redBright('SIGTERM received. Shutting down...'))
  process.exit(1)
})
