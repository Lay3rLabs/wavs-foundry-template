# Typescript Ethereum Price Oracle

A WAVS component that fetches the price of a crypto currency from CoinMarketCap and returns it to the Ethereum contract, in Typescript.

## System Setup

Follow the main [README.md](../../README.md) to install all the necessary dependencies.

## Core Packages

```bash docci-if-not-installed="cast"
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

```bash docci-if-not-installed="wasm-tools"
cargo binstall wasm-tools --no-confirm
```

```bash
make setup
```

```bash
npm --prefix ./components/js-eth-price-oracle/ install
```

## Build Component

Build all wasi components from the root of the repo. You can also run this command within each component directory.

```bash docci-output-contains="Successfully written"
# Builds only this component, not all.
WASI_BUILD_DIR=js-eth-price-oracle make wasi-build
```

## Execute Component

Run the component with the `wasi-exec` command in the root of the repo

```bash docci-output-contains="LTC"
COMPONENT_FILENAME=js_eth_price_oracle.wasm COIN_MARKET_CAP_ID=2 make wasi-exec
```

## Run in a local environment

Start all services

```bash docci-background docci-delay-after=5
make start-all
```

Build your smart contracts

```bash
forge build
```

Deploy the contracts

```bash docci-delay-after=1
export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy`

forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast
```

Deploy the component

```bash docci-delay-after=1
COMPONENT_FILENAME=js_eth_price_oracle.wasm sh ./script/build_service.sh

SERVICE_CONFIG_FILE=.docker/service.json make deploy-service
```

Trigger the service

```bash docci-delay-after=1
export COIN_MARKET_CAP_ID=2
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`

forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig "run(string,string)" --rpc-url http://localhost:8545 --broadcast -v 4
```

Show the result from the triggered service

```bash docci-output-contains="LTC"
make show-result
```
