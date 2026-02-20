/**
 * Build the service.json file.
 *
 * Usage:
 * ```
 * pnpm deploy:build-service
 * ```
 *
 * When setting a component's config values, there are a few different ways to
 * substitute values from the deployment context:
 *
 * 1. Use ${VAR_NAME} to substitute the environment variable VAR_NAME.
 * 2. Use ${get(json.path.to.key)} to get a value from the deployment summary
 *    JSON file.
 * 3. Use ${getEnv(field)} to get a value from the deployment environment
 *    defined in `./env.ts`, such as `aggregatorTimerDelaySeconds`.
 */

import chalk from 'chalk'
import { Command } from 'commander'

import {
  processComponentConfigFile,
  processComponentConfigValues,
} from './config'
import { WAVS_DOCKER_IMAGE } from './constants'
import { initProgram } from './env'
import { ComponentsConfigFile } from './types'
import {
  exec,
  execSilently,
  expandArrayUnwraps,
  keccak256,
  readJson,
  readJsonKey,
} from './utils'

const program = new Command('build-service')
  .description('Build the service.json')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option(
    '-c, --component-config-file <componentConfigFile>',
    'The component configuration file for the service',
    'config/components.json'
  )
  .option(
    '-o, --output <output>',
    'The output file for the service.json',
    '.docker/service.json'
  )
  .option(
    '-s, --service-manager-address <serviceManagerAddress>',
    'The WAVS service manager address (defaults to addresses.POAStakeRegistry from .nodes/poa_deploy.json)'
  )
  .option(
    '-a, --aggregator-url <aggregatorUrl>',
    'The aggregator URL for the service',
    'http://127.0.0.1:8040'
  )
  .option(
    '--fuel-limit <fuelLimit>',
    'The fuel limit for the service',
    '1000000000000'
  )

const main = async () => {
  const {
    env,
    options: {
      componentConfigFile,
      output,
      serviceManagerAddress,
      aggregatorUrl,
      fuelLimit,
    },
  } = initProgram(program)

  console.log(chalk.blueBright(`🚀 Building ${output}...`))

  const {
    components,
    aggregator_components: [aggregatorComponent],
  } = readJson<ComponentsConfigFile>(componentConfigFile)

  // Read deployment summary to expand array unwraps
  const deploymentSummary = readJson<Record<string, unknown>>(
    '.docker/deployment_summary.json'
  )

  // Expand components that use array[] unwrap syntax
  const expandedComponents = components.flatMap((component) =>
    expandArrayUnwraps(component, deploymentSummary)
  )

  const BASE_CMD = [
    'docker',
    'run',
    '--rm',
    '--network',
    'host',
    '-w',
    '/data',
    '-v',
    '.:/data',
    WAVS_DOCKER_IMAGE,
    'wavs-cli',
    'service',
    '--json',
    'true',
    '--home',
    '/data',
    '--file',
    `/data/${output}`,
  ]

  await execSilently(...BASE_CMD, 'init', '--name', env.serviceName)

  for (const component of expandedComponents) {
    if (component.disabled) {
      continue
    }

    const arrayUnwrapInfo = component._arrayUnwraps
      ? ` (${Object.entries(component._arrayUnwraps)
          .map(([k, v]) => `${k}[${v}]`)
          .join(', ')})`
      : ''

    console.log(
      chalk.greenBright(
        `\nBuilding workflow for component: ${component.filename}${arrayUnwrapInfo}`
      )
    )

    const submitAddress = readJsonKey(
      '.docker/deployment_summary.json',
      component.submit.contract_json_path
    )

    if (!submitAddress) {
      console.log(
        chalk.yellowBright(
          `Skipping because submit address (${component.submit.contract_json_path}) is not set`
        )
      )
      continue
    }

    const workflowId = JSON.parse(
      await execSilently(...BASE_CMD, 'workflow', 'add')
    )['workflow_id']

    console.log(chalk.greenBright(`  Workflow ID: ${workflowId}`))

    // Trigger

    if ('block_interval' in component.trigger) {
      const triggerChain =
        component.trigger.block_interval.chain ?? env.triggerChain
      await execSilently(
        ...BASE_CMD,
        'workflow',
        'trigger',
        '--id',
        workflowId,
        'set-block-interval',
        '--chain',
        triggerChain,
        '--n-blocks',
        BigInt(component.trigger.block_interval.blocks).toString()
      )
      console.log(
        chalk.greenBright(
          `  Trigger block interval: ${component.trigger.block_interval.blocks} on chain ${triggerChain}`
        )
      )
    } else if ('cron' in component.trigger) {
      await execSilently(
        ...BASE_CMD,
        'workflow',
        'trigger',
        '--id',
        workflowId,
        'set-cron',
        '--schedule',
        component.trigger.cron.schedule,
        '--start-time',
        component.trigger.cron.start_time,
        '--end-time',
        component.trigger.cron.end_time
      )
      console.log(
        chalk.greenBright(
          `  Trigger cron: ${component.trigger.cron.schedule}`
        )
      )
    } else if ('event' in component.trigger) {
      const triggerChain = component.trigger.event.chain ?? env.triggerChain
      const triggerAddress = readJsonKey(
        '.docker/deployment_summary.json',
        component.trigger.event.contract_json_path
      )
      const triggerEventHash = keccak256(component.trigger.event.event)
      await execSilently(
        ...BASE_CMD,
        'workflow',
        'trigger',
        '--id',
        workflowId,
        'set-evm',
        '--chain',
        triggerChain,
        '--address',
        triggerAddress,
        '--event-hash',
        triggerEventHash
      )
      console.log(
        chalk.greenBright(
          `  Trigger event: ${component.trigger.event.event} from contract ${triggerAddress} on chain ${triggerChain}`
        )
      )
    }

    // Component

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'component',
      '--id',
      workflowId,
      'set-source-registry',
      '--domain',
      new URL(env.registry).host,
      '--package',
      `${env.wasiNamespace}:${component.package_name}`,
      '--version',
      component.package_version
    )

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'component',
      '--id',
      workflowId,
      'permissions',
      '--http-hosts',
      '*',
      '--file-system',
      'true'
    )

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'component',
      '--id',
      workflowId,
      'fuel-limit',
      '--fuel',
      fuelLimit
    )

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'component',
      '--id',
      workflowId,
      'time-limit',
      '--seconds',
      '30'
    )

    if (component.config) {
      await execSilently(
        ...BASE_CMD,
        'workflow',
        'component',
        '--id',
        workflowId,
        'config',
        ...('file' in component.config
          ? processComponentConfigFile(component.config.file, {
              env,
              arrayUnwraps: component._arrayUnwraps,
            })
          : processComponentConfigValues(
              component.config.values,
              component.filename,
              { env, arrayUnwraps: component._arrayUnwraps }
            )
        ).flatMap(([key, value]) => ['--values', `${key}=${value}`])
      )
    }

    if (component.env_variables) {
      await execSilently(
        ...BASE_CMD,
        'workflow',
        'component',
        '--id',
        workflowId,
        'env',
        ...component.env_variables.flatMap((envVar: string) => [
          '--values',
          envVar,
        ])
      )
    }

    // Submit (aggregator)

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'submit',
      '--id',
      workflowId,
      'set-aggregator',
      '--url',
      aggregatorUrl
    )

    console.log(chalk.greenBright(`  Aggregator URL: ${aggregatorUrl}`))

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'submit',
      '--id',
      workflowId,
      'component',
      'set-source-registry',
      '--domain',
      new URL(env.registry).host,
      '--package',
      `${env.wasiNamespace}:${aggregatorComponent.package_name}`,
      '--version',
      aggregatorComponent.package_version
    )

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'submit',
      '--id',
      workflowId,
      'component',
      'permissions',
      '--http-hosts',
      '*',
      '--file-system',
      'true'
    )

    await execSilently(
      ...BASE_CMD,
      'workflow',
      'submit',
      '--id',
      workflowId,
      'component',
      'config',
      ...(aggregatorComponent.config
        ? 'file' in aggregatorComponent.config
          ? processComponentConfigFile(aggregatorComponent.config.file, {
              env,
              extraValues: {
                [env.submitChain]: submitAddress,
              },
              arrayUnwraps: component._arrayUnwraps,
            })
          : processComponentConfigValues(
              aggregatorComponent.config.values,
              aggregatorComponent.filename,
              {
                env,
                extraValues: { [env.submitChain]: submitAddress },
                arrayUnwraps: component._arrayUnwraps,
              }
            )
        : [[env.submitChain, submitAddress]]
      ).flatMap(([key, value]) => ['--values', `${key}=${value}`])
    )

    console.log(
      chalk.greenBright(
        `  Submit address: ${submitAddress} on chain ${env.submitChain}`
      )
    )

    if (aggregatorComponent.env_variables) {
      await execSilently(
        ...BASE_CMD,
        'workflow',
        'submit',
        '--id',
        workflowId,
        'component',
        'env',
        ...aggregatorComponent.env_variables.flatMap((envVar) => [
          '--values',
          envVar,
        ])
      )
    }
  }

  await execSilently(
    ...BASE_CMD,
    'manager',
    'set-evm',
    '--chain',
    env.submitChain,
    '--address',
    serviceManagerAddress
  )

  console.log()
  await exec(...BASE_CMD, 'validate')
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
