# [WAVS](https://docs.wavs.xyz) Monorepo Template

**Template for getting started with developing WAVS applications**

A template for developing WebAssembly AVS applications using Rust and Solidity, configured for Windows *WSL*, Linux, and MacOS. The sample oracle service fetches the current price of a cryptocurrency from [CoinMarketCap](https://coinmarketcap.com) and saves it on chain via the operators.

**Languages**
 * [Rust (this example)](./components/evm-price-oracle/)
 * [Go](./components/golang-evm-price-oracle/README.md)
 * [JS / TS](./components/js-evm-price-oracle/README.md)

## System Requirements

<details>
<summary>Core (Docker, Compose, Make, JQ, Node v21+, Foundry)</summary>

## Ubuntu Base
- **Linux**: `sudo apt update && sudo apt install build-essential`

### Docker

If prompted, remove containerd with `sudo apt remove containerd.io`.

- **MacOS**: `brew install --cask docker`
- **Linux**: `sudo apt -y install docker.io`
- **Windows WSL**: [docker desktop wsl](https://docs.docker.com/desktop/wsl/#turn-on-docker-desktop-wsl-2) & `sudo chmod 666 /var/run/docker.sock`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

### Docker Compose
- **MacOS**: Already installed with Docker installer
> `sudo apt remove docker-compose-plugin` may be required if you get a `dpkg` error
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

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
nvm install --lts
```

### Foundry
```bash docci-ignore
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

</details>

<details>

<summary>Rust v1.85+</summary>

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

On Ubuntu LTS, if you later encounter errors like:

```bash
wkg: /lib/x86_64-linux-gnu/libm.so.6: version `GLIBC_2.38' not found (required by wkg)
wkg: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.39' not found (required by wkg)
```

If GLIB is out of date. Consider updating your system using:
```bash
sudo do-release-upgrade
```


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

> \[!TIP]
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

## Build WASI components

Now build the WASI components into the `compiled` output directory.

> \[!WARNING]
> If you get: `error: no registry configured for namespace "wavs"`
>
> run, `wkg config --default-registry wa.dev`

> \[!WARNING]
> If you get: `failed to find the 'wasm32-wasip1' target and 'rustup' is not available`
>
> `brew uninstall rust` & install it from <https://rustup.rs>

```bash
# This command only builds the rust component.
# Remove `WASI_BUILD_DIR` to build all components.
WASI_BUILD_DIR=components/evm-price-oracle make wasi-build
```

## Testing the Price Feed Component Locally

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

> \[!NOTE]
> If you are running on a Mac with an ARM chip, you will need to do the following:
> - Set up Rosetta: `softwareupdate --install-rosetta`
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_64/amd64 emulation on Apple Silicon")
>
> Configure one of the following networking:
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

## Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

```bash docci-background docci-delay-after=5
# Start the backend
#
# This must remain running in your terminal. Use another terminal to run other commands.
# You can stop the services with `ctrl+c`. Some MacOS terminals require pressing it twice.
cp .env.example .env

# Gets the RPC depending on `DEPLOY_ENV` in .env
export DEPLOY_ENV=`sh ./script/get-deploy-status.sh`
export RPC_URL=`sh ./script/get-rpc.sh`
echo "Using RPC_URL ${RPC_URL}"


# Create new operator
# cast wallet new-mnemonic --json > .docker/operator1.json
# export OPERATOR_MNEMONIC=`cat .docker/operator1.json | jq -r .mnemonic`
# export OPERATOR_PK=`cat .docker/operator1.json | jq -r '.accounts[0].private_key'`

# TODO: this should really just be a start anvil if local script
make start-all
```

## Create Deployer, upload Eigenlayer

```bash
# local: create deployer & auto fund. testnet: create & iterate check balance
sh ./script/create-deployer.sh

## == Deploy Eigenlayer from Deployer ==
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes ghcr.io/lay3rlabs/wavs-middleware:0.4.0-beta.2

# lets next section start, not sure we need this anymore though
# date +%s > .docker/start.log
```

Wait for full local deployment to be ready

```bash docci-delay-after=2
# while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done
```

## Deploy Service Contracts

**Key Concepts:**

*   **Trigger Contract:** Any contract that emits events, then WAVS monitors. When a relevant event occurs, WAVS triggers the execution of your WebAssembly component.
*   **Submission Contract:** This contract is used by the AVS service operator to submit the results generated by the WAVS component on-chain.

`SERVICE_MANAGER_ADDR` is the address of the Eigenlayer service manager contract. It was deployed in the previous step. Then you deploy the trigger and submission contracts which depends on the service manager. The service manager will verify that a submission is valid (from an authorized operator) before saving it to the blockchain. The trigger contract is any arbitrary contract that emits some event that WAVS will watch for. Yes, this can be on another chain (e.g. an L2) and then the submission contract on the L1 *(Ethereum for now because that is where Eigenlayer is deployed)*.

```bash docci-delay-per-cmd=2
export DEPLOYER_PK=$(cat .nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager .nodes/avs_deploy.json)

forge create SimpleSubmit --json --broadcast -r ${RPC_URL} --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r ${RPC_URL} --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```

## Deploy Service

Deploy the compiled component with the contract information from the previous steps. Review the [makefile](./Makefile) for more details and configuration options.`TRIGGER_EVENT` is the event that the trigger contract emits and WAVS watches for. By altering `SERVICE_TRIGGER_ADDR` you can watch events for contracts others have deployed.

```bash docci-delay-per-cmd=3


export COMPONENT_FILENAME=evm_price_oracle.wasm
if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    sh ./script/create-operator.sh 1
    sh ./infra/wavs-1/start.sh

    sh ./script/create-aggregator.sh 1


    # TODO: would require WAVS to start here until we can get
    # it to move to the else block here. then we could keep wavs off for now
    export WASM_DIGEST=$(make upload-component COMPONENT_FILENAME=$COMPONENT_FILENAME)
else
    # ** Setup: https://wa.dev/account/credentials
    export PKG_VERSION="0.1.0"
    export PKG_NAMESPACE=`warg info --namespaces | grep = | cut -d'=' -f1 | tr -d ' '`
    export PKG_NAME="${PKG_NAMESPACE}:evmpriceoraclerust"
    warg publish release --name ${PKG_NAME} --version ${PKG_VERSION} ./compiled/${COMPONENT_FILENAME} || true
fi

# Build your service JSON
AGGREGATOR_URL=http://127.0.0.1:8001 sh ./script/build_service.sh

# Upload service.json to IPFS
ipfs_cid=`IPFS_ENDPOINT=http://127.0.0.1:5001 SERVICE_FILE=.docker/service.json make upload-to-ipfs`

# Deploy the service JSON to WAVS so it now watches and submits.
#
# If CREDENTIAL is not set then the default WAVS_CLI .env account will be used
# You can `cast send ${WAVS_SERVICE_MANAGER} 'transferOwnership(address)'` to move it to another account.
SERVICE_URL="http://127.0.0.1:8080/ipfs/${ipfs_cid}" CREDENTIAL=${DEPLOYER_PK} make deploy-service
```


## Register service specific operator

Each service gets their own key path (hd_path). The first service starts at 1 and increments from there. Get the service ID

```bash
# hack: private key specific to this service
# This is generated from the AVS keys submit mnemonic
# this will be removed in the future. Then we can just --mnemonic-path the different index from source locally
# (where WAVS /service-key returns just the index)
# SERVICE_ID=`curl -s http://localhost:8000/app | jq -r .services[0].id`
# PK=`curl -s http://localhost:8000/service-key/${SERVICE_ID} | jq -rc .secp256k1 | tr -d '[]'`
# AVS_PRIVATE_KEY=`echo ${PK} | tr ',' ' ' | xargs printf "%02x" | tr -d '\n'`

source .env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`

# Faucet funds to the aggregator account to post on chain
cast send $(cast wallet address --private-key ${WAVS_AGGREGATOR_CREDENTIAL}) --rpc-url http://localhost:8545 --private-key ${DEPLOYER_PK} --value 1ether

# Register the operator with the WAVS service manager
AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} DELEGATION=0.01ether make operator-register

# Verify registration
make operator-list
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
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast
```

## Show the result

Query the latest submission contract id from the previous request made.

```bash docci-delay-per-cmd=2 docci-output-contains="1"
make get-trigger
```

```bash docci-delay-per-cmd=2 docci-output-contains="BTC"
TRIGGER_ID=1 make show-result
```

## Claude Code

To spin up a sandboxed instance of [Claude Code](https://docs.anthropic.com/en/docs/agents-and-tools/claude-code/overview) in a Docker container that only has access to this project's files, run the following command:

```bash docci-ignore
npm run claude-code
# or with no restrictions (--dangerously-skip-permissions)
npm run claude-code:unrestricted
```

