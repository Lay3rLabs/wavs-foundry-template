/**
 * Deploy SimpleSubmit and SimpleTrigger contracts to the chain.
 *
 * Usage:
 * ```
 * pnpm deploy:contracts
 * ```
 */

import fs from 'fs'

import chalk from 'chalk'
import { Command } from 'commander'

import { DEPLOYMENT_SUMMARY_FILE } from './constants'
import { initProgram } from './env'
import { execFull } from './utils'

const program = new Command('deploy-contracts')
  .description('Deploy SimpleSubmit and SimpleTrigger contracts to the chain')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option(
    '-s, --service-manager-address <serviceManagerAddress>',
    'The WAVS service manager address (defaults to addresses.POAStakeRegistry from .nodes/poa_deploy.json)'
  )
  .option(
    '-k, --funded-key <fundedKey>',
    'The funded private key for the deployer (default: $FUNDED_KEY from .env)'
  )
  .option(
    '-r, --rpc-url <rpcUrl>',
    'The RPC URL for the chain (default: $RPC_URL from .env)'
  )

const main = async () => {
  const {
    env,
    options: { fundedKey, serviceManagerAddress },
  } = initProgram(program)

  if (!serviceManagerAddress) {
    throw new Error(
      'Service manager address is required. Deploy POA middleware first or provide --service-manager-address.'
    )
  }

  if (!fundedKey) {
    throw new Error(
      'Funded key is required. Provide --funded-key or set FUNDED_KEY in .env.'
    )
  }

  // Build contracts
  console.log(chalk.blueBright('🔨 Building contracts...'))
  await execFull({ cmd: ['forge', 'build'], log: 'all' })

  fs.mkdirSync('.docker', { recursive: true })

  // Deploy SimpleSubmit
  console.log(chalk.blueBright('🚀 Deploying SimpleSubmit...'))
  const submitOutput = await execFull({
    cmd: [
      'forge',
      'create',
      'SimpleSubmit',
      '--json',
      '--broadcast',
      '--rpc-url',
      env.rpcUrl,
      '--private-key',
      '"$FUNDED_KEY"',
      '--constructor-args',
      serviceManagerAddress,
    ],
    log: 'cmd',
    shell: true,
    env: { FUNDED_KEY: fundedKey },
  })

  const submitJson = JSON.parse(submitOutput.trim())
  fs.writeFileSync('.docker/submit.json', JSON.stringify(submitJson, null, 2))
  console.log(
    chalk.greenBright(`✅ SimpleSubmit deployed to ${submitJson.deployedTo}`)
  )

  // Deploy SimpleTrigger
  console.log(chalk.blueBright('🚀 Deploying SimpleTrigger...'))
  const triggerOutput = await execFull({
    cmd: [
      'forge',
      'create',
      'SimpleTrigger',
      '--json',
      '--broadcast',
      '--rpc-url',
      env.rpcUrl,
      '--private-key',
      '"$FUNDED_KEY"',
      '--constructor-args',
      serviceManagerAddress,
    ],
    log: 'cmd',
    shell: true,
    env: { FUNDED_KEY: fundedKey },
  })

  const triggerJson = JSON.parse(triggerOutput.trim())
  fs.writeFileSync('.docker/trigger.json', JSON.stringify(triggerJson, null, 2))
  console.log(
    chalk.greenBright(`✅ SimpleTrigger deployed to ${triggerJson.deployedTo}`)
  )

  // Write deployment summary
  const deploymentSummary = {
    service_id: '',
    rpc_url: env.rpcUrl,
    wavs_service_manager: serviceManagerAddress,
    evmpriceoracle_submit: submitJson,
    evmpriceoracle_trigger: triggerJson,
  }

  fs.writeFileSync(
    DEPLOYMENT_SUMMARY_FILE,
    JSON.stringify(deploymentSummary, null, 2)
  )

  console.log(
    chalk.greenBright(
      '🎉 All contracts deployed successfully! Deployment summary saved to .docker/deployment_summary.json'
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
