/**
 * Deploy the entire WAVS stack.
 *
 * Usage:
 * ```
 * pnpm deploy:full
 * ```
 */

import fs from 'fs'
import path from 'path'
import readline from 'readline'

import chalk from 'chalk'
import { Command } from 'commander'

import { DEPLOYMENT_SUMMARY_FILE, POA_MIDDLEWARE_IMAGE } from './constants'
import { DEFAULT_OPTIONS, initProgram } from './env'
import { exec, execFull, loadDotenv, readJson, sleep } from './utils'

const program = new Command('deploy-script')
  .description('Deploy the entire WAVS stack.')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option('--no-component-upload', "Don't upload components")
  .option('--no-contract-upload', "Don't upload contracts")
  .option(
    '-r, --rpc-url <rpcUrl>',
    'The RPC URL for the chain (default: $RPC_URL from .env)'
  )
  .option(
    '-w, --wavs-url <wavsUrl>',
    'The WAVS operator URL for the service',
    'http://127.0.0.1:8041'
  )
  .option(
    '-a, --aggregator-url <aggregatorUrl>',
    'The aggregator URL for the service',
    'http://127.0.0.1:8040'
  )
  .option(
    '-t, --stake-threshold <stakeThreshold>',
    'The POA stake weight threshold',
    (t) => parseInt(t),
    1000
  )
  .option(
    '-q, --quorum <quorum>',
    'The POA quorum as a fraction (e.g. 1/2)',
    '2/3'
  )
  .option('-g, --ipfs-gateway <ipfsGateway>', 'The IPFS gateway to use')

const main = async () => {
  const {
    envName,
    env,
    options: {
      componentUpload,
      contractUpload,
      rpcUrl,
      wavsUrl,
      aggregatorUrl,
      stakeThreshold,
      quorum: _quorum,
    },
  } = initProgram(program)

  const quorum = (_quorum as string)
    .match(/^\s*(\d+)\s*\/\s*(\d+)\s*$/)
    ?.slice(1)
  if (!quorum || quorum.length !== 2 || Number(quorum[0]) > Number(quorum[1])) {
    throw new Error(
      "Quorum is invalid. Please ensure it's a valid fraction less than or equal to 1, like `1/2`."
    )
  }

  console.log(chalk.blueBright('🚀 Starting complete WAVS deployment...'))

  // Start component upload in background
  const componentUploadPromise = componentUpload
    ? execFull({
        cmd: ['pnpm', 'deploy:upload-components'],
        logColor: chalk.cyanBright,
      })
    : null

  // Create deployer, which sets FUNDED_KEY in .env
  await exec('pnpm', 'deploy:create-deployer')

  // Update FUNDED_KEY environment variable so other processes we spawn see the new one
  process.env.FUNDED_KEY = loadDotenv().FUNDED_KEY

  // Deploy service manager
  if (contractUpload) {
    console.log(chalk.blueBright('🚀 Deploying POA Service Manager...'))

    const poaMiddlewareBase =
      `docker run --rm --network host -v ./.nodes:/root/.nodes --env-file .env ${POA_MIDDLEWARE_IMAGE}`.split(
        ' '
      )

    await execFull({
      cmd: [...poaMiddlewareBase, 'deploy'],
      env: {
        RPC_URL: rpcUrl,
      },
    })
    await sleep(1)

    await execFull({
      cmd: [
        ...poaMiddlewareBase,
        'owner_operation',
        'updateStakeThreshold',
        stakeThreshold,
      ],
      env: {
        RPC_URL: rpcUrl,
      },
    })
    await sleep(1)

    await execFull({
      cmd: [...poaMiddlewareBase, 'owner_operation', 'updateQuorum', ...quorum],
      env: {
        RPC_URL: rpcUrl,
      },
    })
    await sleep(1)

    console.log(chalk.greenBright('✅ Deployed POA Service Manager'))
  }

  // Fetch service manager that we just deployed
  const serviceManagerAddress = DEFAULT_OPTIONS['serviceManagerAddress']()
  if (!serviceManagerAddress) {
    throw new Error('Failed to retrieve service manager address.')
  }
  console.log(
    `ℹ️  Using WAVS service manager address: ${serviceManagerAddress}`
  )

  if (envName === 'dev') {
    // Mine a block to ensure checkpoints are up to date
    await exec('cast', 'rpc', 'anvil_mine', '--rpc-url', rpcUrl)
  }

  // Deploy contracts
  if (contractUpload) {
    await exec('pnpm', 'deploy:contracts')
    await sleep(1)
  }

  // Wait for component upload to finish
  if (componentUploadPromise) {
    console.log('⏳ Waiting up to 3 minutes for component upload to finish...')
    await new Promise((resolve, reject) => {
      // Reject in 3 minutes if component upload not finished.
      const bail = setTimeout(
        () =>
          reject(
            new Error(
              '❌ Component upload not finished after 3 minutes, bailing.'
            )
          ),
        3 * 60 * 1_000
      )

      componentUploadPromise
        .then(resolve)
        .catch(reject)
        .finally(() => clearTimeout(bail))
    })

    console.log(
      '✅ All components uploaded successfully, waiting 7 seconds for registry to update...'
    )
    await sleep(7)
  }

  // Build service.json
  await exec('pnpm', 'deploy:build-service')
  await sleep(1)

  // Wait for user to confirm service.json before uploading on prod
  if (envName === 'prod') {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    })

    await new Promise((resolve) =>
      rl.question(
        'Make any changes you want to the service.json now. Press [Enter] to continue upload to IPFS...',
        resolve
      )
    )
  }

  // Upload service.json
  const serviceJsonCidOutputFile = path.resolve('.docker/service-cid')
  await exec('pnpm', 'deploy:upload-service', '-o', serviceJsonCidOutputFile)
  await sleep(1)

  const serviceJsonCid = fs.readFileSync(serviceJsonCidOutputFile)
  if (!serviceJsonCid) {
    throw new Error(
      `❌ service.json CID not found in ${serviceJsonCidOutputFile}`
    )
  }

  // Create and start aggregator
  await exec('pnpm', 'deploy:create-aggregator', '1', '-f')
  await execFull({
    cmd: ['bash', './infra/aggregator-1/start.sh'],
    env: {
      IPFS_GATEWAY: env.ipfs.gateway,
    },
  })
  await sleep(3)

  // Register service on aggregator
  const res = await fetch(aggregatorUrl + '/services', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      service_manager: {
        evm: {
          chain: env.submitChain,
          address: serviceManagerAddress,
        },
      },
    }),
  })
  if (!res.ok) {
    throw new Error(
      `❌ Failed to register service on aggregator: ${res.statusText} (${(await res.text().catch(() => '<unable to parse response body>')) || '<no body>'})`
    )
  }

  // Create and start operator
  await exec('pnpm', 'deploy:create-operator', '1', '-f')
  await execFull({
    cmd: ['bash', './infra/wavs-1/start.sh'],
    env: {
      IPFS_GATEWAY: env.ipfs.gateway,
    },
  })
  await sleep(3)

  // Deploy the service.json to WAVS
  await execFull({
    cmd: ['task', 'deploy:service'],
    env: {
      WAVS_ENDPOINT: wavsUrl,
      SERVICE_URL: `ipfs://${serviceJsonCid}`,
      IPFS_GATEWAY: env.ipfs.gateway,
    },
  })
  await sleep(3)

  // Get the service ID from WAVS
  const servicesRes = await fetch(wavsUrl + '/services')
  if (!servicesRes.ok) {
    throw new Error(
      `❌ Failed to fetch services from operator: ${servicesRes.statusText} (${(await servicesRes.text().catch(() => '<unable to parse response body>')) || '<no body>'})`
    )
  }

  const { service_ids } = await servicesRes.json()
  if (
    !service_ids ||
    !Array.isArray(service_ids) ||
    !service_ids.length ||
    !service_ids[0]
  ) {
    throw new Error('❌ WAVS operator unexpectedly returned no services')
  }

  const serviceId = service_ids[0]
  console.log(chalk.greenBright(`✅ Service ID: ${serviceId}`))

  // Update the service ID in the deployment summary
  const deploymentSummary = readJson(DEPLOYMENT_SUMMARY_FILE)
  deploymentSummary.service_id = serviceId
  fs.writeFileSync(
    DEPLOYMENT_SUMMARY_FILE,
    JSON.stringify(deploymentSummary, null, 2)
  )

  // Register service-specific operator
  await execFull({
    cmd: ['task', 'setup-avs-signing'],
    env: {
      SERVICE_ID: serviceId,
      HD_INDEX: '1',
    },
  })

  // Reset registry
  await exec('warg', 'reset', '--registry', env.registry).catch((err) =>
    console.log(
      chalk.yellowBright(`⚠️ Failed to reset registry (non-critical): ${err}`)
    )
  )

  // Run post-deploy script if it exists
  const postDeployScript = path.join(__dirname, './post-deploy.sh')
  if (fs.existsSync(postDeployScript)) {
    console.log(chalk.blueBright(`⚙️ Running ${postDeployScript}...`))
    await exec('bash', postDeployScript)
    console.log(chalk.greenBright(`✅ Post-deploy script complete!`))
  }

  // On dev, set up single operator POA
  if (envName === 'dev') {
    console.log(chalk.blueBright('⚙️ Setting up single operator POA...'))
    await exec('task', 'deploy:single-operator-poa-local')
  }

  console.log(chalk.greenBright('🎉🎉🎉 Deployment complete! 🎉🎉🎉'))
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
