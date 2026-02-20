import { spawn } from 'child_process'
import fs from 'fs'
import path from 'path'

import { keccak_256 } from '@noble/hashes/sha3.js'
import chalk, { ChalkInstance } from 'chalk'
import dotenv, { DotenvParseOutput } from 'dotenv'

import { ComponentsConfigFile, ExpandedComponent } from './types'

/**
 * Simple dot-notation path getter (replaces lodash.get for simple paths).
 */
const getByPath = (obj: any, pathStr: string): any =>
  pathStr.split('.').reduce((acc: any, key: string) => acc?.[key], obj)

/**
 * Loads the environment variables from the .env file, creating from the
 * .env.example file if it doesn't exist.
 */
export const loadDotenv = (): DotenvParseOutput => {
  const envFile = path.resolve(path.join(__dirname, '../.env'))
  if (!fs.existsSync(envFile)) {
    const exampleEnvFile = path.resolve(path.join(__dirname, '../.env.example'))
    if (!fs.existsSync(exampleEnvFile)) {
      throw new Error(
        `Example environment file ${exampleEnvFile} does not exist.`
      )
    }
    fs.copyFileSync(exampleEnvFile, envFile)
  }

  const { error, parsed } = dotenv.config({ path: envFile })
  if (error) {
    throw new Error(`Error loading .env file: ${error.message}`)
  }
  if (!parsed) {
    throw new Error('No environment variables loaded from .env file')
  }

  return parsed
}

/**
 * Reads a file and returns the contents as a string.
 */
export const readFile = (file: string): string => {
  const filePath = path.resolve(file)

  if (!fs.existsSync(filePath)) {
    throw new Error(`File ${filePath} does not exist`)
  }

  return fs.readFileSync(filePath, 'utf8')
}

/**
 * Reads a JSON file and returns the object. Throws an error if the file does
 * not exist.
 */
export const readJson = <T = any>(file: string): T => JSON.parse(readFile(file))

/**
 * Reads a JSON file and returns the object. Returns undefined if the file does
 * not exist.
 */
export const readJsonIfFileExists = <T = any>(file: string): T | undefined =>
  fs.existsSync(file) ? readJson<T>(file) : undefined

/**
 * Reads a JSON file and returns the value of a key from the object.
 * The key is a dot-notation path.
 */
export const readJsonKey = <T = any>(file: string, keyPath: string): T => {
  const json = readJson(file)
  return getByPath(json, keyPath) as T
}

/**
 * Reads a JSON file (if it exists) and returns the value of a key from the
 * object. Returns undefined if the file does not exist.
 */
export const readJsonKeyIfFileExists = <T = any>(
  file: string,
  keyPath: string
): T | undefined => {
  const json = readJsonIfFileExists(file)
  if (json) {
    return getByPath(json, keyPath) as T
  }
  return undefined
}

/**
 * Executes a command and returns a promise that resolves when the command
 * completes (with the stdout) or rejects with the status code if the command
 * fails.
 */
export const execFull = ({
  cmd,
  log = 'all',
  logColor = chalk.blueBright,
  env,
  shell = false,
}: {
  cmd: string[]
  log?: 'cmd' | 'all' | 'none'
  logColor?: ChalkInstance
  env?: Record<string, string>
  shell?: boolean
}): Promise<string> =>
  new Promise((resolve, reject) => {
    const command = cmd[0]
    const args = cmd.slice(1)

    if (log === 'cmd' || log === 'all') {
      console.log(chalk.gray(`$ ${cmd.join(' ')}`))
    }

    const childProcess = spawn(command, args, {
      stdio: 'pipe',
      env: {
        ...process.env,
        ...env,
      },
      shell,
    })

    let stdout = ''
    childProcess.stdout.on('data', (data) => {
      const line = data.toString()
      stdout += line
      if (log === 'all') {
        process.stdout.write(logColor(line))
      }
    })

    let stderr = ''
    childProcess.stderr.on('data', (data) => {
      const line = data.toString()
      stderr += line
      if (log === 'all') {
        process.stdout.write(chalk.redBright(line))
      }
    })

    childProcess.on('close', (code) => {
      if (code === 0) {
        resolve(stdout)
      } else {
        reject(
          new Error(
            `Command \`${cmd.join(' ')}\` failed with code ${code}:\n${stdout}\n${stderr}`
          )
        )
      }
    })
  })

/**
 * Executes a command, logging all output to the console.
 */
export const exec = (...cmd: string[]): Promise<string> =>
  execFull({ cmd, log: 'all' })

/**
 * Executes a command silently, capturing stdout without printing.
 */
export const execSilently = (...cmd: string[]): Promise<string> =>
  execFull({ cmd, log: 'none' })

/**
 * Computes the keccak256 hash of a string and returns it as a hex string.
 */
export const keccak256 = (str: string): string =>
  Buffer.from(keccak_256(new TextEncoder().encode(str))).toString('hex')

/**
 * Sleep for a number of seconds.
 */
export const sleep = (seconds: number) =>
  new Promise<void>((resolve) => setTimeout(resolve, seconds * 1_000))

/**
 * Detect array unwrap patterns (e.g., "networks[]") in a component.
 */
const detectArrayUnwraps = (component: object): string[] => {
  const matches = JSON.stringify(component).matchAll(/\${(\w+)\[\]}/g)
  return [...new Set([...matches].map((m) => m[1]))]
}

/**
 * Expand a component for all elements of arrays using the [] unwrap syntax.
 */
export const expandArrayUnwraps = (
  component: ComponentsConfigFile['components'][number],
  deploymentSummary: Record<string, unknown>
): ExpandedComponent[] => {
  if (component.disabled) {
    return [component]
  }

  const arrayKeys = detectArrayUnwraps(component)
  if (arrayKeys.length === 0) {
    return [component]
  }

  if (arrayKeys.length > 1) {
    throw new Error(
      `Multiple array unwraps not supported yet: ${arrayKeys.join(', ')}`
    )
  }

  const arrayKey = arrayKeys[0]
  const array = deploymentSummary[arrayKey] as unknown[]

  if (!Array.isArray(array)) {
    throw new Error(
      `Array unwrap "${arrayKey}[]" used but "${arrayKey}" is not an array in deployment summary`
    )
  }

  return array.map((_, index) => ({
    ...JSON.parse(
      JSON.stringify(component).replaceAll(
        `\${${arrayKey}[]}`,
        `${arrayKey}.${index}`
      )
    ),
    _arrayUnwraps: { [arrayKey]: index },
  }))
}
