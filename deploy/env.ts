import fs from 'fs'
import path from 'path'

import { Command } from 'commander'

import { EnvName, EnvOverrides, IEnv, ProgramContext } from './types'
import { loadDotenv, readJsonKeyIfFileExists } from './utils'

type NonFunctionPropertyNames<T> = {
  [K in keyof T]: T[K] extends Function ? never : K
}[keyof T]

type OmitFunctions<T> = Pick<T, NonFunctionPropertyNames<T>>

abstract class EnvBase implements IEnv {
  rpcUrl: string
  registry: string
  serviceName: string
  wasiNamespace: string
  triggerChain: string
  submitChain: string
  ipfs: {
    pinApi: string
    gateway: string
  }
  aggregatorTimerDelaySeconds: number

  constructor(options: OmitFunctions<IEnv>) {
    this.rpcUrl = options.rpcUrl
    this.registry = options.registry
    this.serviceName = options.serviceName
    this.wasiNamespace = options.wasiNamespace
    this.triggerChain = options.triggerChain
    this.submitChain = options.submitChain
    this.ipfs = options.ipfs
    this.aggregatorTimerDelaySeconds = options.aggregatorTimerDelaySeconds
  }

  static get(envName: EnvName | string, overrides: EnvOverrides = {}): IEnv {
    switch (envName.toLowerCase()) {
      case 'dev':
        return new DevEnv(overrides)
      case 'prod':
        return new ProdEnv(overrides)
    }

    throw new Error(`Invalid environment: ${envName}`)
  }

  /**
   * Uploads a file to IPFS and returns the CID.
   */
  abstract uploadToIpfs(file: string, apiKey?: string): Promise<string>

  /**
   * Query IPFS for a CID and return the content.
   */
  async queryIpfs(uriOrCid: string): Promise<string> {
    const cid = uriOrCid.replace('ipfs://', '')
    const response = await fetch(this.ipfs.gateway + cid)

    if (!response.ok) {
      throw new Error(
        `Failed to query IPFS: ${response.status} ${response.statusText}. Body: ${await response.text().catch(() => '<unable to read body>')}`
      )
    }

    return response.text()
  }
}

export class DevEnv extends EnvBase {
  constructor({
    rpcUrl = 'http://127.0.0.1:8545',
    ipfsGateway = 'http://127.0.0.1:8080/ipfs/',
  }: EnvOverrides) {
    super({
      rpcUrl,
      registry: 'http://localhost:8090',
      serviceName: 'wavs-foundry-template',
      wasiNamespace: 'example',
      triggerChain: 'evm:31337',
      submitChain: 'evm:31337',
      ipfs: {
        pinApi: 'http://127.0.0.1:5001/api/v0/add?pin=true',
        gateway: ipfsGateway,
      },
      aggregatorTimerDelaySeconds: 0,
    })
  }

  async uploadToIpfs(file: string, _apiKey?: string): Promise<string> {
    const filePath = path.resolve(file)
    if (!fs.existsSync(filePath)) {
      throw new Error(`File ${filePath} does not exist`)
    }

    const formData = new FormData()
    formData.append(
      'file',
      new Blob([new Uint8Array(fs.readFileSync(filePath))])
    )

    const response = await fetch(this.ipfs.pinApi, {
      method: 'POST',
      body: formData,
    })

    if (!response.ok) {
      throw new Error(
        `Failed to upload file to IPFS: ${response.status} ${response.statusText}. Body: ${await response.text().catch(() => '<unable to read body>')}`
      )
    }

    const { Hash } = await response.json()

    // Verify the upload by querying IPFS for the file and checking the content exists.
    let error
    for (let i = 0; i < 5; i++) {
      try {
        const content = await this.queryIpfs(Hash)
        if (content) {
          return Hash
        } else {
          throw new Error('Uploaded file content empty.')
        }
      } catch (err) {
        await new Promise((resolve) => setTimeout(resolve, 1000))
        error = err
      }
    }

    throw new Error(`Failed to verify IPFS upload: ${error}`)
  }
}

export class ProdEnv extends EnvBase {
  constructor({
    rpcUrl = 'https://ethereum-sepolia-rpc.publicnode.com',
    ipfsGateway = 'https://gateway.pinata.cloud/ipfs/',
  }: EnvOverrides) {
    super({
      rpcUrl,
      registry: 'https://wa.dev',
      serviceName: 'wavs-foundry-template',
      wasiNamespace: 'wavs-foundry',
      triggerChain: 'evm:11155111', // Sepolia
      submitChain: 'evm:11155111',
      ipfs: {
        pinApi: 'https://uploads.pinata.cloud/v3/files',
        gateway: ipfsGateway,
      },
      aggregatorTimerDelaySeconds: 3,
    })
  }

  async uploadToIpfs(file: string, apiKey?: string): Promise<string> {
    if (!apiKey) {
      throw new Error('API key is required for IPFS uploads')
    }

    const filePath = path.resolve(file)
    if (!fs.existsSync(filePath)) {
      throw new Error(`File ${filePath} does not exist`)
    }

    const formData = new FormData()
    formData.append(
      'file',
      new Blob([new Uint8Array(fs.readFileSync(filePath))])
    )
    formData.append('network', 'public')
    formData.append('name', `service-${Date.now()}.json`)

    const response = await fetch(this.ipfs.pinApi, {
      method: 'POST',
      body: formData,
      headers: { Authorization: `Bearer ${apiKey}` },
    })

    if (!response.ok) {
      throw new Error(
        `Failed to upload file to IPFS: ${response.status} ${response.statusText}. Body: ${await response.text().catch(() => '<unable to read body>')}`
      )
    }

    const {
      data: { cid },
    } = await response.json()

    // Verify the upload by querying IPFS for the file and checking the content exists.
    let error
    for (let i = 0; i < 5; i++) {
      try {
        const content = await this.queryIpfs(cid)
        if (content) {
          return cid
        } else {
          throw new Error('Uploaded file content empty.')
        }
      } catch (err) {
        await new Promise((resolve) => setTimeout(resolve, 1000))
        error = err
      }
    }

    throw new Error(`Failed to verify IPFS upload: ${error}`)
  }
}

/**
 * Default option values.
 */
export const DEFAULT_OPTIONS: Record<string, () => string | undefined> = {
  rpcUrl: () => process.env.RPC_URL,
  fundedKey: () => process.env.FUNDED_KEY,
  ipfsApiKey: () => process.env.WAVS_ENV_PINATA_API_KEY,
  serviceManagerAddress: () =>
    readJsonKeyIfFileExists(
      '.nodes/poa_deploy.json',
      'addresses.POAStakeRegistry'
    ),
}

/**
 * Initialize the program by parsing the arguments and returning the relevant
 * environment and options.
 */
export const initProgram = (program: Command): ProgramContext => {
  // Copy wavs.toml from example if it doesn't exist and example exists
  const wavsTomlFile = path.resolve(path.join(__dirname, '../wavs.toml'))
  const exampleWavsTomlFile = path.resolve(
    path.join(__dirname, '../wavs.toml.example')
  )
  if (!fs.existsSync(wavsTomlFile) && fs.existsSync(exampleWavsTomlFile)) {
    fs.copyFileSync(exampleWavsTomlFile, wavsTomlFile)
  }

  const dotenv = loadDotenv()

  program.parse(process.argv)

  const options = program.opts()
  // Set default environment before applying default options.
  // Normalize legacy bash env names (LOCAL→dev, TESTNET→prod) to TypeScript names.
  if (!options.env) {
    const rawEnv = process.env.DEPLOY_ENV?.toLowerCase() || 'dev'
    options.env =
      rawEnv === 'local' ? 'dev' : rawEnv === 'testnet' ? 'prod' : rawEnv
  }

  const appliedOptions = applyDefaultOptions(options)
  const env = getEnv(options.env, options)

  return {
    envName: options.env,
    env,
    options: appliedOptions,
    dotenv,
  }
}

/**
 * Gets the environment for the given environment name.
 */
export const getEnv = (
  envName: EnvName,
  overrides: EnvOverrides = {}
): IEnv => {
  const env = EnvBase.get(envName, overrides)
  if (!env) {
    throw new Error(`Invalid environment: ${envName}`)
  }

  return env
}

/**
 * Apply default option values.
 */
export const applyDefaultOptions = (options: Record<string, any>) =>
  Object.entries(DEFAULT_OPTIONS).reduce((acc, [key, value]) => {
    if (!options[key]) {
      acc[key] = value()
    }
    return acc
  }, options)
