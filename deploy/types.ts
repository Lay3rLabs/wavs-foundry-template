import { DotenvParseOutput } from 'dotenv'

export type ComponentsConfigFile = {
  components: {
    disabled?: boolean
    filename: string
    package_name: string
    package_version: string
    trigger:
      | {
          event: {
            contract_json_path: string
            event: string
            chain?: string
          }
        }
      | {
          block_interval: {
            blocks: number
            chain?: string
          }
        }
      | {
          cron: {
            schedule: string
            start_time: string
            end_time: string
          }
        }
    submit: {
      contract_json_path: string
      chain?: string
    }
    config?:
      | {
          file: string
        }
      | {
          values: Record<string, unknown>
        }
    env_variables?: string[]
  }[]
  aggregator_components: {
    disabled?: boolean
    filename: string
    package_name: string
    package_version: string
    config?:
      | {
          file: string
        }
      | {
          values: Record<string, unknown>
        }
    env_variables?: string[]
  }[]
}

export type EnvName = 'dev' | 'prod'

export type IEnv = {
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
  uploadToIpfs: (file: string, apiKey?: string) => Promise<string>
}

export type EnvOverrides = {
  rpcUrl?: string
  ipfsGateway?: string
}

export type ProgramContext = {
  /** The environment name */
  envName: EnvName
  /** The environment context */
  env: IEnv
  /** Passed in options */
  options: Record<string, any>
  /** The environment variables loaded from the .env file */
  dotenv: DotenvParseOutput
}

export type ProcessConfigOptions = {
  env: IEnv
  extraValues?: Record<string, unknown>
  arrayUnwraps?: Record<string, number> // e.g., { "networks": 0 }
}

export type ExpandedComponent = ComponentsConfigFile['components'][number] & {
  _arrayUnwraps?: Record<string, number>
}
