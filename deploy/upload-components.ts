/**
 * Upload components to the registry.
 *
 * Usage:
 * ```
 * pnpm deploy:upload-components
 * ```
 */

import fs from 'fs'
import path from 'path'

import chalk from 'chalk'
import { Command } from 'commander'

import { initProgram } from './env'
import { ComponentsConfigFile } from './types'
import { exec, execFull, execSilently, readJson } from './utils'

const program = new Command('upload-components')
  .description('Upload components to the registry')
  .option(
    '-e, --env <env>',
    'The deploy environment (dev or prod) (default: $DEPLOY_ENV from .env)'
  )
  .option(
    '-f, --folder <folder>',
    'The folder of compiled component WASM files',
    'compiled'
  )
  .option(
    '-c, --component-config-file <componentConfigFile>',
    'The component configuration file for the service',
    'config/components.json'
  )
  .option(
    '-s, --status-file <statusFile>',
    'The file to store the status of the upload'
  )

const tempHomes = new Set<string>()
let statusFilePath: string | null = null

const cleanupTempHomes = () => {
  for (const tempHome of tempHomes) {
    fs.rmSync(tempHome, {
      recursive: true,
      force: true,
    })
  }
}

const main = async () => {
  const {
    env,
    options: { componentConfigFile, folder, statusFile: _statusFile },
  } = initProgram(program)

  statusFilePath = _statusFile ? path.resolve(_statusFile) : null
  if (statusFilePath) {
    fs.writeFileSync(statusFilePath, 'UPLOADING')
  }

  const compiledFolder = path.resolve(folder)
  if (!fs.existsSync(compiledFolder)) {
    throw new Error(
      `Compiled folder not found: ${compiledFolder}. Run \`task build:wasi\` to build the components or correct the folder path.`
    )
  }

  const { components, aggregator_components } =
    readJson<ComponentsConfigFile>(componentConfigFile)

  const componentFiles = [
    ...new Set(
      [...components, ...aggregator_components]
        .filter((c) => !c.disabled)
        .map((component) => component.filename)
    ),
  ]

  let componentsNeedingBuild = componentFiles.filter(
    (filename) => !fs.existsSync(path.join(compiledFolder, filename))
  )

  if (componentsNeedingBuild.length > 0) {
    console.log(chalk.yellowBright(`\n🔧 Building components...`))
    await exec('task', 'build:wasi')
  }

  componentsNeedingBuild = componentFiles.filter(
    (filename) => !fs.existsSync(path.join(compiledFolder, filename))
  )

  if (componentsNeedingBuild.length > 0) {
    throw new Error(
      `Components still missing after build: ${componentsNeedingBuild.join(', ')}`
    )
  }

  const componentsToUpload = componentFiles.map(
    (filename) =>
      components.find((c) => c.filename === filename)! ||
      aggregator_components.find((c) => c.filename === filename)!
  )

  const results = await Promise.allSettled(
    componentsToUpload.map(async (component) => {
      try {
        const componentWasmPath = path.join(compiledFolder, component.filename)
        if (!fs.existsSync(componentWasmPath)) {
          throw new Error(
            `Component file not found: ${componentWasmPath}. This should not happen.`
          )
        }

        const fullName = `${env.wasiNamespace}:${component.package_name}@${component.package_version}`
        console.log(
          chalk.greenBright(
            `🚀 Uploading ${component.filename} as ${fullName} to ${env.registry}...`
          )
        )

        // Use temporary warg home to bypass locks from other uploads
        const tempHome = (
          await execSilently('mktemp', '-d', '-t', 'warg_home_XXXXXX')
        ).trim()
        tempHomes.add(tempHome)

        await execFull({
          cmd: ['warg', 'config', '--registry', env.registry],
          env: {
            WARG_HOME: tempHome,
          },
        })

        let output
        try {
          output = await execFull({
            cmd: [
              'warg',
              'publish',
              'release',
              '--name',
              `${env.wasiNamespace}:${component.package_name}`,
              '--version',
              component.package_version,
              componentWasmPath,
            ],
            log: 'none',
            env: {
              WARG_HOME: tempHome,
            },
          })
        } catch (error) {
          output = (error as Error).message
        }

        await execSilently('rm', '-rf', tempHome)

        if (output.includes('Unauthorized')) {
          throw new Error(`❌ ${fullName} failed: ${output}`)
        }

        if (
          output.includes('already released') ||
          output.includes('failed to prove inclusion')
        ) {
          console.log(
            chalk.yellowBright(
              `✅ ${fullName} already released (did not re-upload)`
            )
          )
          return
        }

        if (
          output.includes('submitted record') ||
          output.includes('published version')
        ) {
          console.log(chalk.blueBright(`✅ ${fullName} uploaded`))
          return
        }

        throw new Error(`❌ ${fullName} failed: ${output}`)
      } catch (error) {
        console.error(chalk.redBright(error))
        throw error
      }
    })
  )

  const successful = results.filter((result) => result.status === 'fulfilled')
  const failed = results.filter((result) => result.status === 'rejected')

  console.log(
    chalk.greenBright(
      `\n📊 Results: ✅ ${successful.length} success, ❌ ${failed.length} failed`
    )
  )

  if (failed.length > 0) {
    throw new Error(`❌ ${failed.length} components failed to upload`)
  }

  console.log(chalk.blueBright(`🎉 All components uploaded successfully`))

  if (statusFilePath) {
    fs.writeFileSync(statusFilePath, 'COMPLETED')
  }
}

main().catch((err) => {
  console.error(chalk.redBright(err.message))
  if (statusFilePath) {
    fs.writeFileSync(statusFilePath, 'ERROR')
  }
  cleanupTempHomes()
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
