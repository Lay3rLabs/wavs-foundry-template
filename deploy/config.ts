import path from 'path'

import { IEnv, ProcessConfigOptions } from './types'
import { readJson, readJsonKey } from './utils'

/**
 * Process component config file and return list of keys and values.
 *
 * @param file - The component config file to read
 * @param options - Processing options
 * @returns The list of keys and values
 */
export const processComponentConfigFile = (
  file: string,
  options: ProcessConfigOptions
): [string, unknown][] => {
  const config = processComponentConfigValues(
    readJson(
      path.resolve(path.join(__dirname, `../config/${file}.template.json`))
    ),
    file,
    options
  )

  return config
}

/**
 * Process component config values and return list of keys and values.
 *
 * Supports the following substitution patterns:
 * - `${VAR_NAME}` — environment variable
 * - `${get(json.path.to.key)}` — value from deployment summary JSON
 * - `${join(json.path.to.array)}` — comma-joined array from deployment summary JSON
 * - `${getEnv(field)}` — value from the IEnv deployment environment object
 *
 * @param config - The component config values
 * @param name - The name/file of the component, for error messages
 * @param options - Processing options
 * @returns The list of keys and values
 */
export const processComponentConfigValues = (
  config: Record<string, unknown>,
  name: string,
  options: ProcessConfigOptions
): [string, unknown][] =>
  Object.entries({ ...config, ...options.extraValues }).map(([key, value]) => {
    return [
      key,
      typeof value === 'string'
        ? value
            // Replace array[] patterns with array.{index} based on arrayUnwraps context.
            .replaceAll(/\${(\w+)\[\]}/g, (_, arrayKey) => {
              const index = options.arrayUnwraps?.[arrayKey]
              if (index === undefined) {
                throw new Error(
                  `Array unwrap "${arrayKey}[]" used in component "${name}" config value "${key}" but no index provided`
                )
              }
              return `${arrayKey}.${index}`
            })
            // Replace all ${get(json.path.to.key)} patterns with the value from deployment summary.
            .replaceAll(/\${get\(([^)]+)\)}/g, (_, p1) =>
              readJsonKey('.docker/deployment_summary.json', p1)
            )
            // Replace all ${join(json.path.to.array)} patterns with comma-separated values.
            .replaceAll(/\${join\(([^)]+)\)}/g, (_, p1) => {
              const arr = readJsonKey('.docker/deployment_summary.json', p1)
              if (!Array.isArray(arr)) {
                throw new Error(
                  `join() expects an array but got ${typeof arr} for path "${p1}" in component "${name}" config value "${key}"`
                )
              }
              return arr.join(',')
            })
            // Replace all ${getEnv(field)} patterns with the value from the deployment environment.
            .replaceAll(/\${getEnv\(([^)]+)\)}/g, (_, p1) => {
              if (!(p1 in options.env)) {
                throw new Error(
                  `Invalid environment key: ${p1} not found in env.`
                )
              }
              const envVar = options.env[p1 as keyof IEnv]
              if (envVar === undefined) {
                throw new Error(
                  `Env field ${p1} does not exist in deployment environment but is needed for component "${name}" config value "${key}"`
                )
              }
              return envVar.toString()
            })
            // Replace all ${VAR_NAME} patterns with their environment variable values.
            .replaceAll(/\${\s*([^{}]+)\s*}/g, (_, p1) => {
              const envVar = process.env[p1]
              if (!envVar) {
                throw new Error(
                  `Environment variable ${p1} is not set but needed for component "${name}" config value "${key}"`
                )
              }
              return envVar
            })
        : value,
    ]
  })
