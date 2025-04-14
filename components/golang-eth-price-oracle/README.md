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
WASI_BUILD_DIR=golang-eth-price-oracle make wasi-build
```

## Execute Component

Run the component with the `wasi-exec` command in the root of the repo

```bash docci-output-contains="LTC"
COMPONENT_FILENAME=golang_eth_price_oracle.wasm COIN_MARKET_CAP_ID=2 make wasi-exec
```

## Run in a local environment

Start all services

```bash docci-background docci-delay-after=5
anvil --fork-url https://ethereum-holesky-rpc.publicnode.com
```

Build your smart contracts

```bash
forge build
```

Deploy eigen middleware

```bash
cp .env.example .env

docker run --rm --network host --env-file .env  -v ./.nodes:/root/.nodes ghcr.io/reecepbcups/wavs-middleware:0.0.1
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager .nodes/avs_deploy.json)
```

```bash
export PRIVATE_KEY=$(cat .nodes/deployer)
export MY_ADDR=$(cast wallet address --private-key $PRIVATE_KEY)
sed -i 's/test test test test test test test test test test test junk/'$PRIVATE_KEY'/' .env
```

Register the operator

```bash
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes --entrypoint /wavs/register.sh ghcr.io/reecepbcups/wavs-middleware:0.0.1 "$PRIVATE_KEY"

docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes --entrypoint /wavs/list_operator.sh ghcr.io/reecepbcups/wavs-middleware:0.0.1
```

Run WAVS & the aggregator

```bash docci-background docci-delay-after=5
docker compose up --remove-orphans &
```


Deploy the contracts

```bash docci-delay-after=1
forge create SimpleSubmit --json --broadcast --rpc-url http://127.0.0.1:8545 --private-key "${PRIVATE_KEY}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast --rpc-url http://127.0.0.1:8545 --private-key "${PRIVATE_KEY}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```

Deploy the component

```bash docci-delay-after=1
COMPONENT_FILENAME=golang_eth_price_oracle.wasm sh ./script/build_service.sh

SERVICE_CONFIG_FILE=.docker/service.json make deploy-service
```

Trigger the service

```bash docci-delay-after=2
export COIN_MARKET_CAP_ID=1
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`

forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast -v 4
```

Show the result from the triggered service

```bash docci-output-contains="BTC"
make show-result
```
