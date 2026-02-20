/**
 * Upload the service.json to IPFS and optionally set the service URI on WAVS.
 *
 * Usage:
 * ```
 * pnpm deploy:upload-service
 * ```
 */

import fs from 'fs'
import path from 'path'

import chalk from 'chalk'
import { Command } from 'commander'

import { initProgram } from './env'
import { execFull } from './utils'

const program = new Command('upload-service')
  .description(
    'Upload the service.json to IPFS and optionally set the service URI on WAVS'
  )
  .option('--no-wavs', 'Do not set the service URI on WAVS')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option(
    '-f, --service-file <serviceFile>',
    'The service.json file to upload',
    '.docker/service.json'
  )
  .option('-g, --ipfs-gateway <ipfsGateway>', 'The IPFS gateway to use')
  .option(
    '-a, --ipfs-api-key <ipfsApiKey>',
    'The API key to use for the IPFS upload (defaults to $WAVS_ENV_PINATA_API_KEY from .env)'
  )
  .option(
    '-k, --funded-key <fundedKey>',
    'The funded private key for the deployer (default: $FUNDED_KEY from .env)'
  )
  .option(
    '-r, --rpc-url <rpcUrl>',
    'The RPC URL for the chain (default: $RPC_URL from .env)'
  )
  .option(
    '-s, --service-manager-address <serviceManagerAddress>',
    'The WAVS service manager address (defaults to addresses.POAStakeRegistry from .nodes/poa_deploy.json)'
  )
  .option('-o, --output <output>', 'Save the CID to a file')

const main = async () => {
  const {
    env,
    options: {
      serviceFile,
      ipfsApiKey,
      fundedKey,
      serviceManagerAddress,
      // inverse of --no-wavs presence
      wavs,
      output,
    },
  } = initProgram(program)

  console.log(chalk.blueBright(`🚀 Uploading ${serviceFile} to IPFS...`))

  const cid = await env.uploadToIpfs(serviceFile, ipfsApiKey)

  // Save to output file if set
  if (output) {
    fs.writeFileSync(path.resolve(output), cid)
  }

  console.log(chalk.greenBright(`✅ Service uploaded to IPFS: ipfs://${cid}`))

  if (wavs) {
    console.log(chalk.greenBright('Setting service URI on service manager...'))

    if (!fundedKey) {
      console.error(chalk.redBright('❌ Error: FUNDED_KEY is not set'))
      process.exit(1)
    }

    await execFull({
      cmd: [
        'cast',
        'send',
        serviceManagerAddress,
        '"setServiceURI(string)"',
        `"ipfs://${cid}"`,
        '--rpc-url',
        env.rpcUrl,
        '--private-key',
        '"$FUNDED_KEY"',
      ],
      log: 'all',
      shell: true,
      env: {
        FUNDED_KEY: fundedKey,
      },
    })

    console.log(chalk.greenBright('✅ Service URI updated'))
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
