# Golang Ethereum Price Oracle

A WAVS component that fetches the price of a crypto currency from CoinMarketCap and returns it to the Ethereum contract, in Go.

## System Setup

### Mac

```bash docci-os=mac
brew tap tinygo-org/tools
brew install tinygo
```

### Arch Linux

```bash docci-ignore
sudo pacman -Sy tinygo
```

### Ubuntu Linux

```bash docci-os=linux docci-if-not-installed="tinygo"
# https://tinygo.org/getting-started/install/linux/
wget --quiet https://github.com/tinygo-org/tinygo/releases/download/v0.37.0/tinygo_0.37.0_amd64.deb
sudo dpkg -i tinygo_0.37.0_amd64.deb && rm tinygo_0.37.0_amd64.deb
```

## Core Packages

```bash docci-if-not-installed="cast"
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

```bash
make setup
```

```bash docci-if-not-installed="cargo-binstall"
cargo install cargo-binstall
```

```bash docci-if-not-installed="wasm-tools"
cargo binstall wasm-tools --no-confirm
```

<!-- matches the value in the wavs-wasi for generation of the bindings -->
```bash occi-if-not-installed="wit-bindgen-go"
go install go.bytecodealliance.org/cmd/wit-bindgen-go@ecfa620df5beee882fb7be0740959e5dfce9ae26
wit-bindgen-go --version
```

## Verify installs

```bash
tinygo version
wkg --version
```

## Build Component

Build all wasi components from the root of the repo. You can also run this command within each component directory.

```bash
# Builds only this component, not all.
WASI_BUILD_DIR=golang-evm-price-oracle make wasi-build
```

## Execute Component

Run the component with the `wasi-exec` command in the root of the repo

```bash docci-output-contains="LTC"
COMPONENT_FILENAME=golang_evm_price_oracle.wasm COIN_MARKET_CAP_ID=2 make wasi-exec
```

Build your smart contracts

```bash
forge build
```

## Run in a local environment

Start all services

```bash docci-background docci-delay-after=25
cp .env.example .env

# Create new operator
cast wallet new-mnemonic --json > .docker/operator1.json
export OPERATOR_MNEMONIC=`cat .docker/operator1.json | jq -r .mnemonic`
export OPERATOR_PK=`cat .docker/operator1.json | jq -r .accounts[0].private_key`

make start-all
```

Wait for full local deployment, then grab values

```bash docci-ignore
while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done
```

## Deploy Service Contracts

```bash docci-delay-per-cmd=3
export DEPLOYER_PK=$(cat .nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager .nodes/avs_deploy.json)

forge create SimpleSubmit --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```

## Deploy Service

```bash docci-delay-per-cmd=3
COMPONENT_FILENAME=golang_evm_price_oracle.wasm sh ./script/build_service.sh

# TODO: kill this at the end of the test
cd .docker && python3 -m http.server 9999 &

SERVICE_URL=http://0.0.0.0:9999/service.json CREDENTIAL=${DEPLOYER_PK} make deploy-service
```

## Register service specific operator

```bash docci-delay-per-cmd=2
source .env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`

# Register the operator with the WAVS service manager
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes --entrypoint /wavs/register.sh "ghcr.io/lay3rlabs/wavs-middleware:0.4.0-alpha.5" "$AVS_PRIVATE_KEY"

# Verify registration
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes --entrypoint /wavs/list_operator.sh ghcr.io/lay3rlabs/wavs-middleware:0.4.0-alpha.5

# Faucet funds to the aggregator account to post on chain
cast send $(cast wallet address --private-key ${WAVS_AGGREGATOR_CREDENTIAL}) --rpc-url http://localhost:8545 --private-key ${DEPLOYER_PK} --value 1ether
```

Trigger the service

```bash docci-delay-after=2
export COIN_MARKET_CAP_ID=1
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast
```

Show the result from the triggered service

```bash docci-output-contains="BTC"
make show-result
```
