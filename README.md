# [WAVS](https://docs.wavs.xyz) Monorepo Template

**Template for getting started with developing WAVS applications**

A template for developing WebAssembly AVS applications using Rust and Solidity, configured for Windows *WSL*, Linux, and MacOS. The sample oracle service fetches the current price of a cryptocurrency from [CoinMarketCap](https://coinmarketcap.com) and saves it on chain via the operators.

**Languages**
 * [Rust (this example)](./components/eth-price-oracle/)
 * [Go](./components/golang-eth-price-oracle/README.md)
 * [JS / TS](./components/js-eth-price-oracle/README.md)

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

```bash docci-ignore
# if foundry is not installed:
# `curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup`
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

Now build the WASI components into the `compiled` output directory.

> [!WARNING]
> If you get: `error: no registry configured for namespace "wavs"`
>
> run, `wkg config --default-registry wa.dev`

> [!WARNING]
> If you get: `failed to find the 'wasm32-wasip1' target and 'rustup' is not available`
>
> `brew uninstall rust` & install it from <https://rustup.rs>

```bash
# or `make build` to include solidity compilation.
# this command only builds the rust component.
# Remove `WASI_BUILD_DIR` to build all components.
WASI_BUILD_DIR=components/eth-price-oracle make wasi-build
```

### Testing the Price Feed Component Locally

How to test the component locally for business logic validation before on-chain deployment. An ID of 1 for the oracle component is Bitcoin.

```bash
COIN_MARKET_CAP_ID=1 make wasi-exec
```

Expected output:

```shell docci-ignore
input id: 1
resp_data: PriceFeedData {
    symbol: "BTC",
    timestamp: "2025-04-01T00:00:00.000Z",
    price: 82717.27035239758
}
INFO Fuel used: 653415

Result (hex encoded):
7b2273796d626f6c223a22425443222c2274696d657374616d70223a22323032352d30342d30315430303a34...

Result (utf8):
{"symbol":"BTC","timestamp":"2025-04-01T00:00:00.000Z","price":82717.27035239758}
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

```bash docci-background docci-delay-after=15
# Start the backend
#
# This must remain running in your terminal. Use another terminal to run other commands.
# You can stop the services with `ctrl+c`. Some MacOS terminals require pressing it twice.
# make start-all
cp .env.example .env

sh ./script/start_all.sh
```

Wait for full local deployment, then grab values

```bash docci-delay-after=2
while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done

export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager .nodes/avs_deploy.json)
export PRIVATE_KEY=$(cat .nodes/deployer)
export MY_ADDR=$(cast wallet address --private-key $PRIVATE_KEY)
```

### Deploy Service Contracts

**Key Concepts:**

*   **Trigger Contract:** Any contract that emits events, then WAVS monitors. When a relevant event occurs, WAVS triggers the execution of your WebAssembly component.
*   **Submission Contract:** This contract is used by the AVS service operator to submit the results generated by the WAVS component on-chain.

`SERVICE_MANAGER_ADDR` is the address of the Eigenlayer service manager contract. It was deployed in the previous step. Then you deploy the trigger and submission contracts which depends on the service manager. The service manager will verify that a submission is valid (from an authorized operator) before saving it to the blockchain. The trigger contract is any arbitrary contract that emits some event that WAVS will watch for. Yes, this can be on another chain (e.g. an L2) and then the submission contract on the L1 *(Ethereum for now because that is where Eigenlayer is deployed)*.

```bash
forge create SimpleSubmit --json --broadcast -r http://127.0.0.1:8545 --private-key "${PRIVATE_KEY}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r http://127.0.0.1:8545 --private-key "${PRIVATE_KEY}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```

## Deploy Service

Deploy the compiled component with the contract information from the previous steps. Review the [makefile](./Makefile) for more details and configuration options.`TRIGGER_EVENT` is the event that the trigger contract emits and WAVS watches for. By altering `SERVICE_TRIGGER_ADDR` you can watch events for contracts others have deployed.

```bash docci-delay-per-cmd=2
# Build your service JSON with optional overrides in the script
COMPONENT_FILENAME=eth_price_oracle.wasm AGGREGATOR_URL=http://127.0.0.1:8001 sh ./script/build_service.sh

# Deploy the service JSON to WAVS so it now watches and submits
# the results based on the service json configuration.
SERVICE_CONFIG_FILE=.docker/service.json make deploy-service
```

## Trigger the Service

Anyone can now call the [trigger contract](./src/contracts/WavsTrigger.sol) which emits the trigger event WAVS is watching for from the previous step. WAVS then calls the service and saves the result on-chain.

```bash
# Request BTC from CMC
export COIN_MARKET_CAP_ID=1
# Get the trigger address from previous Deploy forge script
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`
# Execute on the trigger contract, WAVS will pick this up and submit the result
# on chain via the operators.
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast -v 4
```

## Show the result

Query the latest submission contract id from the previous request made.

```bash docci-delay-per-cmd=2 docci-output-contains="1"
make get-trigger
```

```bash docci-delay-per-cmd=2 docci-output-contains="BTC"
TRIGGER_ID=1 make show-result
```
