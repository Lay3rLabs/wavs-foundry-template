# [WAVS](https://docs.wavs.xyz) Monorepo Template

**Template for getting started with developing WAVS applications**

A template for developing WebAssembly AVS applications using Rust and Solidity, configured for Windows *WSL*, Linux, and MacOS. The sample oracle service fetches the current price of a cryptocurrency from [CoinMarketCap](https://coinmarketcap.com) and saves it on chain.

## System Requirements

<details>
<summary>Core (Docker, Compose, Make, JQ, Node v21+)</summary>

### Docker
- **MacOS**: `brew install --cask docker`
- **Linux**: `sudo apt -y install docker.io`
- **Windows WSL**: [docker desktop wsl](https://docs.docker.com/desktop/wsl/#turn-on-docker-desktop-wsl-2) & `sudo chmod 666 /var/run/docker.sock`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

### Docker Compose
- **MacOS**: Already installed with Docker installer
- **Linux + Windows WSL**: `sudo apt-get install docker-compose-v2`
- [Compose Documentation](https://docs.docker.com/compose/)

### Make
- **MacOS**: `brew install make`
- **Linux + Windows WSL**: `sudo apt -y install make`
- [Make Documentation](https://www.gnu.org/software/make/manual/make.html)

### JQ
- **MacOS**: `brew install jq`
- **Linux + Windows WSL**: `sudo apt -y install jq`
- [JQ Documentation](https://jqlang.org/download/)

### Node.js
- **Required Version**: v21+
- [Installation via NVM](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)
</details>

<details>

<summary>Rust v1.84+</summary>

### Rust Installation

```bash docci-ignore
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install stable
rustup target add wasm32-wasip2
```

### Upgrade Rust

```bash docci-ignore
# Remove old targets if present
rustup target remove wasm32-wasi || true
rustup target remove wasm32-wasip1 || true

# Update and add required target
rustup update stable
rustup target add wasm32-wasip2
```

</details>

<details>
<summary>Cargo Components</summary>

### Install Cargo Components

```bash docci-ignore
# Install required cargo components
# https://github.com/bytecodealliance/cargo-component#installation
cargo install cargo-binstall
cargo binstall cargo-component warg-cli wkg --locked --no-confirm --force

# Configure default registry
# Found at: $HOME/.config/wasm-pkg/config.toml
wkg config --default-registry wa.dev
```

</details>

## Create Project

<!-- TODO: this would be unignored and run from an external CI, but not for now -->
```bash docci-ignore
# if foundry is not installed: `curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup`
forge init --template Lay3rLabs/wavs-foundry-template my-wavs --branch main
```

> [!TIP]
> Run `make help` to see all available commands and environment variable overrides.

### Solidity

Install the required packages to build the Solidity contracts. This project supports both [submodules](./.gitmodules) and [npm packages](./package.json).

```bash
# Install packages (npm & submodules)
make setup

# Build the contracts
forge build

# Run the solidity tests
forge test
```

### Build WASI components

Now build the WASI rust components into the `compiled` output directory.

> [!WARNING]
> If you get: `error: no registry configured for namespace "wavs"`
>
> run, `wkg config --default-registry wa.dev`

```bash
# or `make build` to include solidity compilation.
make wasi-build
```

### Execute WASI component directly

Test run the component locally to validate the business logic works. An ID of 1 is Bitcoin. Nothing will be saved on-chain, just the output of the component is shown.

```bash
COIN_MARKET_CAP_ID=1 make wasi-exec
```

## WAVS

> [!NOTE]
> If you are running on a Mac with an ARM chip, you will need to do the following:
> - Set up Rosetta: `softwareupdate --install-rosetta`
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_64/amd64 emulation on Apple Silicon")
>
> Configure one of the following networking:
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

### Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

```bash docci-background docci-delay-after=5
# cp .env.example .env

# Start the backend
#
# This must remain running in your terminal. Use another terminal to run other commands.
# You can stop the services with `ctrl+c`. Some MacOS terminals require pressing it twice.
make start-all
```

### Deploy Contract

Upload your service's trigger and submission contracts. The trigger contract is where WAVS will watch for events, and the submission contract is where the AVS service operator will submit the result on chain.

```bash docci-delay-per-cmd=1
export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy`
forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast
```

> [!TIP]
> You can see the deployed trigger address with `make get-trigger-from-deploy`
> and the deployed submission address with `make get-service-handler-from-deploy`

## Deploy Service

Deploy the compiled component with the contracts from the previous steps. Review the [makefile](./Makefile) for more details and configuration options.`TRIGGER_EVENT` is the event that the trigger contract emits and WAVS watches for. By altering `SERVICE_TRIGGER_ADDR` you can watch events for contracts others have deployed.

```bash docci-delay-per-cmd=1
# Build your service JSON
COMPONENT_FILENAME=eth_price_oracle.wasm sh ./script/build_service.sh

# Deploy the service JSON
SERVICE_CONFIG_FILE=.docker/service.json make deploy-service
```

## Trigger the Service

Anyone can now call the [trigger contract](./src/contracts/WavsTrigger.sol) which emits the trigger event WAVS is watching for from the previous step. WAVS then calls the service and saves the result on-chain.

```bash
export COIN_MARKET_CAP_ID=1
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig "run(string,string)" --rpc-url http://localhost:8545 --broadcast -v 4
```

## Show the result

Query the latest submission contract id from the previous request made.

```bash docci-delay-per-cmd=2 docci-output-contains="BTC"
# Get the latest TriggerId and show the result via `script/ShowResult.s.sol`
make show-result
```
