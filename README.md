# [WAVS](https://docs.wavs.xyz) Monorepo Template

**Template for getting started with developing WAVS applications**

A template for developing WebAssembly AVS applications using Rust and Solidity, configured for Windows *WSL*, Linux, and MacOS. The sample oracle service fetches the current price of a cryptocurrency from [CoinMarketCap](https://coinmarketcap.com) and saves it on chain via the operators.

**Languages**
 * [Rust (this example)](./components/evm-price-oracle/)
 * [Go](./components/golang-evm-price-oracle/README.md)
 * [JS / TS](./components/js-evm-price-oracle/README.md)

## Usage

### 1. System setup

Follow the instructions in [README_SETUP.md](./README_SETUP.md) to ensure your system is set up with the necessary tools and dependencies.

Then install dependencies:

```bash
# Install packages (pnpm & forge submodules)
task -y setup
```

### 2. Solidity

This project supports [pnpm packages](./package.json), you can add git submodules if you need.

```bash
# Build the contracts
task build:forge

# Run the solidity tests
task test
```

### 3. Build WASI components

Build the WASI components into the `compiled` output directory.

> \[!WARNING]
> If you get: `error: no registry configured for namespace "wavs"`
>
> run, `wkg config --default-registry wa.dev`

> \[!WARNING]
> If you get: `failed to find the 'wasm32-wasip1' target and 'rustup' is not available`
>
> `brew uninstall rust` & install it from <https://rustup.rs>

```bash
# Remove `WASI_BUILD_DIR` to build all components.
WASI_BUILD_DIR=components/evm-price-oracle task -y build:wasi
```

### 4. Test the component locally

Validate business logic before on-chain deployment. An ID of `1` is Bitcoin.

```bash
INPUT_DATA="1" COMPONENT_FILENAME=evm_price_oracle.wasm task wasi:exec
```

Expected output:

```shell docci-ignore
Decoded crypto ID: 1
resp_data: PriceFeedData { symbol: "BTC", timestamp: "2025-10-01T18:12:11", price: 116999.97 }
INFO Fuel used: 702137

Time elapsed (ms): 123

Result (hex encoded): 7b2273796d626f6c2...

Result (utf8): {"symbol":"BTC","timestamp":"2025-10-01T18:12:11","price":116999.97}

Ordering: 0
```

### 5. Start backend services

> [!NOTE]
> This must remain running in your terminal. Use new terminals to run other commands. You can stop the services with `ctrl+c`. Some terminals require pressing it twice.

```bash docci-background docci-delay-after=5
# Create a .env file from the example
cp .env.example .env

# Starts anvil + IPFS and WARG registry.
task -y start-all-local
```

### 6. Deploy and run WAVS

This script automates the complete WAVS deployment process, including contract deployments, component uploads, and operator registration, in a single command:

```bash
task deploy-full
```

### 7. Trigger the service

Anyone can now call the [trigger contract](./src/contracts/WavsTrigger.sol) to emit the trigger event WAVS is watching for. WAVS then calls the service and saves the result on-chain.

```bash
# Get the trigger address from the deployment summary
export SERVICE_TRIGGER_ADDR=`jq -r '.evmpriceoracle_trigger.deployedTo' .docker/deployment_summary.json`

export RPC_URL=`task get-rpc`
export FUNDED_KEY=`task config:funded-key`

# Request BTC price from CoinMarketCap (ID=1)
export INPUT_DATA=`cast abi-encode "addTrigger(string)" "1"`

forge script ./src/script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${INPUT_DATA} --sig 'run(string,string)' --rpc-url ${RPC_URL} --broadcast --private-key ${FUNDED_KEY}
```

### 8. Show the result

Query the latest submission from the previous request.

```bash docci-delay-per-cmd=2 docci-output-contains="1"
RPC_URL=${RPC_URL} forge script ./src/script/ShowResult.s.sol ${SERVICE_TRIGGER_ADDR} --sig 'trigger(string)' --rpc-url ${RPC_URL}
```

```bash docci-delay-per-cmd=2 docci-output-contains="BTC"
export SERVICE_SUBMIT_ADDR=`jq -r '.evmpriceoracle_submit.deployedTo' .docker/deployment_summary.json`
RPC_URL=${RPC_URL} forge script ./src/script/ShowResult.s.sol ${SERVICE_SUBMIT_ADDR} 1 --sig 'data(string,uint64)' --rpc-url ${RPC_URL}
```

## AI Coding Agents

This template contains rulefiles for building components with Claude Code and Cursor. Read the [AI-powered component creation guide](./docs/handbook/ai.mdx) for usage instructions.

### Claude Code

To spin up a sandboxed instance of [Claude Code](https://docs.anthropic.com/en/docs/agents-and-tools/claude-code/overview) in a Docker container that only has access to this project's files, run the following command:

```bash docci-ignore
pnpm run claude-code
# or with no restrictions (--dangerously-skip-permissions)
pnpm run claude-code:unrestricted
```
