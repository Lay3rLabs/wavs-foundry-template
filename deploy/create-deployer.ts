/**
 * Create and fund a new deployer wallet.
 *
 * Usage:
 * ```
 * pnpm deploy:create-deployer
 * ```
 */

import fs from 'fs'

import chalk from 'chalk'
import { Command } from 'commander'
import { Hex, createPublicClient, formatEther, http, parseEther } from 'viem'
import { generatePrivateKey, privateKeyToAddress } from 'viem/accounts'

import { initProgram } from './env'
import { sleep } from './utils'

const program = new Command('create-deployer')
  .description('Create and fund a new deployer wallet')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option(
    '-r, --rpc-url <rpcUrl>',
    'The RPC URL for the chain (default: $RPC_URL from .env)'
  )

const main = async () => {
  const {
    envName,
    options: { rpcUrl },
    dotenv: { FUNDED_KEY },
  } = initProgram(program)

  const client = createPublicClient({
    transport: http(rpcUrl),
  })

  if (!fs.existsSync('.docker')) {
    fs.mkdirSync('.docker')
  }

  // Create deployer key if on dev, or if one doesn't exist.
  let fundedKey
  if (envName === 'dev' || !FUNDED_KEY) {
    console.log(
      chalk.blueBright(
        '🔑 Creating new deployer key and saving as FUNDED_KEY in .env...'
      )
    )
    fundedKey = generatePrivateKey()
    fs.writeFileSync(
      '.env',
      fs
        .readFileSync('.env', 'utf8')
        .replace(/FUNDED_KEY=[^\n]*(\n|$)/, `FUNDED_KEY=${fundedKey}\n`)
    )
  } else {
    console.log(chalk.blueBright('🔑 Using existing FUNDED_KEY from .env...'))
    fundedKey = FUNDED_KEY
  }

  if (!fundedKey) {
    throw new Error('FUNDED_KEY is not set')
  }

  const deployerAddress = privateKeyToAddress(fundedKey as Hex)
  console.log(
    chalk.cyanBright(`🏦 Deployer wallet address: ${deployerAddress}`)
  )

  // Fund deployer on dev
  if (envName === 'dev') {
    console.log(chalk.blueBright(`💸 Funding deployer...`))

    await client.request({
      method: 'anvil_setBalance' as any,
      params: [deployerAddress, ('0x' + parseEther('15').toString(16)) as Hex],
    })
    await sleep(1)

    const balance = await client.getBalance({
      address: deployerAddress,
    })

    console.log(
      chalk.greenBright(`✅ Deployer funded with ${formatEther(balance)} ETH`)
    )
  } else {
    console.log(chalk.yellowBright(`🔄 Waiting for deployer to be funded...`))

    while (true) {
      const balance = await client.getBalance({
        address: deployerAddress,
      })

      if (balance !== 0n) {
        console.log(
          chalk.greenBright(
            `✅ Deployer funded with ${formatEther(balance)} ETH`
          )
        )
        break
      }

      await sleep(3)
    }
  }
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
