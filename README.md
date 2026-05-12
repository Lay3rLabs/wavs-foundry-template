# [WAVS](https://docs.wavs.xyz) Monorepo Template

**Template for getting started with developing WAVS applications**

A template for developing WebAssembly AVS applications using Rust and Solidity, configured for Windows *WSL*, Linux, and MacOS. The sample oracle service fetches the current price of a cryptocurrency from [CoinMarketCap](https://coinmarketcap.com) and saves it on chain via the operators.

See [`components/evm-price-oracle/`](./components/evm-price-oracle/) for the reference Rust component.

## Prerequisites

### Linux build essentials

If on Linux (e.g. Ubuntu), install the essentials:

```bash
sudo apt update && sudo apt install build-essential
```

### Docker

If prompted, remove container with `sudo apt remove containerd.io`.

- **MacOS**: `brew install --cask docker`
- **Linux**: `sudo apt -y install docker.io`
- **Windows WSL**: [docker desktop wsl](https://docs.docker.com/desktop/wsl/#turn-on-docker-desktop-wsl-2) & `sudo chmod 666 /var/run/docker.sock`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

> **Note:** `sudo` is only used for Docker-related commands in this project. If you prefer not to use sudo with Docker, you can add your user to the Docker group with:
>
> ```bash
> sudo groupadd docker && sudo usermod -aG docker $USER
> ```
>
> After adding yourself to the group, log out and back in for changes to take effect.

> [!NOTE]
> If you are running on a Mac with an ARM chip, you will need to do the following:
>
> - Set up Rosetta: `softwareupdate --install-rosetta`
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_64/amd64 emulation on Apple Silicon")
>
> Configure one of the following networking:
>
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

### Docker Compose

- **MacOS**: Already installed with Docker installer
  > `sudo apt remove docker-compose-plugin` may be required if you get a `dpkg` error
- **Linux + Windows WSL**: `sudo apt-get install docker-compose-v2`
- [Compose Documentation](https://docs.docker.com/compose/)

### Task (Taskfile)

- **MacOS**: `brew install go-task`
- **Linux + Windows WSL**: `npm install -g @go-task/cli`
- [Task Documentation](https://taskfile.dev/)

### JQ

- **MacOS**: `brew install jq`
- **Linux + Windows WSL**: `sudo apt -y install jq`
- [JQ Documentation](https://jqlang.org/download/)

### Node.js

- **Required Version**: v21+
- [Installation via NVM](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
nvm install --lts
```

### pnpm

Install the latest version of pnpm: https://pnpm.io/installation

### Foundry

```bash docci-ignore
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

### Rust v1.85+

```bash docci-ignore
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install stable
rustup target add wasm32-wasip2
```

#### Upgrade Rust

```bash docci-ignore
# Remove old targets if present
rustup target remove wasm32-wasi || true
rustup target remove wasm32-wasip1 || true

# Update and add required target
rustup update stable
rustup target add wasm32-wasip2
```

### Cargo Components

```bash docci-ignore
# Install required cargo components
# https://github.com/bytecodealliance/cargo-component#installation
cargo install cargo-binstall
cargo binstall cargo-component wasm-tools warg-cli wkg --locked --no-confirm --force

# Configure default registry
# Found at: $HOME/.config/wasm-pkg/config.toml
wkg config --default-registry wa.dev

# Allow publishing to a registry
#
# if WSL: `warg config --keyring-backend linux-keyutils`
warg key new
```

On Ubuntu LTS, if you later encounter errors like:

```bash
wkg: /lib/x86_64-linux-gnu/libm.so.6: version `GLIBC_2.38' not found (required by wkg)
wkg: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.39' not found (required by wkg)
```

If GLIB is out of date. Consider updating your system using:

```bash
sudo do-release-upgrade
```

## Usage

### 1. Install dependencies

```bash
# Install packages (pnpm, forge submodules, WIT deps via wkg)
task -y setup
```

`task setup` runs `task wit:fetch` after pnpm/forge install. WIT deps land in `wit/deps/` and `wit-aggregator/deps/` (both gitignored — re-run `task wit:fetch` to refresh).

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
