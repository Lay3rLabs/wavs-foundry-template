/**
 * Create a new operator.
 *
 * Usage:
 * ```
 * pnpm deploy:create-operator <index>
 * ```
 */

import fs from 'fs'
import path from 'path'
import readline from 'readline'

import chalk from 'chalk'
import { Command } from 'commander'
import { toHex } from 'viem'
import { english, generateMnemonic, mnemonicToAccount } from 'viem/accounts'

import { WAVS_DOCKER_IMAGE } from './constants'
import { initProgram } from './env'
import { execSilently } from './utils'

const program = new Command('create-operator')
  .description('Create a new operator')
  .argument('[index]', 'The operator index (default: 1)')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option('-i, --index <index>', 'The operator index (default: 1)', (i) =>
    parseInt(i)
  )
  .option('-f, --force', 'Override existing operator without confirming')

const main = async () => {
  const {
    envName,
    options: { index: _index, force },
  } = initProgram(program)

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

  console.log(chalk.blueBright(`🤖 Creating operator ${index}...`))

  if (!fs.existsSync('.docker')) {
    fs.mkdirSync('.docker')
  }

  const containerName = `wavs-${index}`
  const readableOperatorLocation = `infra/${containerName}`
  const operatorLocation = path.join(
    __dirname,
    `../${readableOperatorLocation}`
  )

  if (fs.existsSync(operatorLocation)) {
    if (force) {
      console.log(
        chalk.blueBright(
          `🔥 Destroying existing operator at ${readableOperatorLocation}...`
        )
      )

      // Kill the operator container if running, ignore errors
      await execSilently('docker', 'kill', containerName).catch(() => {})

      fs.rmSync(operatorLocation, { recursive: true })
    } else {
      throw new Error(
        `❌ Operator already exists at ${readableOperatorLocation}. Use -f (--force) to override.`
      )
    }
  }

  fs.mkdirSync(operatorLocation, { recursive: true })

  // If prod deployment, prompt for mnemonic
  let mnemonic
  if (envName === 'prod') {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    })

    mnemonic = await new Promise<string>((resolve) =>
      rl.question(
        'Enter operator mnemonic (leave blank to generate a new one): ',
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
    chalk.cyanBright(`💸 Operator wallet address: ${account.address}`)
  )

  // Update operator env file with new mnemonic and private key.
  fs.writeFileSync(
    path.join(operatorLocation, '.env'),
    fs
      .readFileSync(path.join(__dirname, './templates/operator.env'), 'utf8')
      .replace('INSERT_MNEMONIC_HERE', mnemonic)
      .replace('INSERT_PRIVATE_KEY_HERE', privateKey)
  )

  // Create startup script
  fs.writeFileSync(
    path.join(operatorLocation, 'start.sh'),
    fs
      .readFileSync(
        path.join(__dirname, './templates/operator.start.sh'),
        'utf8'
      )
      .replace('INSERT_IMAGE_HERE', WAVS_DOCKER_IMAGE)
      .replace('INSERT_INSTANCE_NAME_HERE', containerName)
  )

  // Copy WAVS config file if it exists
  const wavsTomlSrc = path.join(__dirname, '../wavs.toml')
  if (fs.existsSync(wavsTomlSrc)) {
    fs.copyFileSync(wavsTomlSrc, path.join(operatorLocation, 'wavs.toml'))
  }

  console.log(
    chalk.greenBright(
      `✅ Operator ${index} created successfully at ${readableOperatorLocation}`
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
