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

Build your smart contracts

```bash
forge build
```

### Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

```bash docci-background docci-delay-after=15
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

Deploy the contracts

```bash docci-delay-after=1
forge create SimpleSubmit --json --broadcast -r http://127.0.0.1:8545 --private-key "${PRIVATE_KEY}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r http://127.0.0.1:8545 --private-key "${PRIVATE_KEY}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```


Upload service

```bash docci-delay-per-cmd=2
# Build your service JSON with optional overrides in the script
COMPONENT_FILENAME=js_eth_price_oracle.wasm sh ./script/build_service.sh

SERVICE_CONFIG_FILE=.docker/service.json make deploy-service
```


```bash docci-delay-after=2
# Request BTC from CMC
export COIN_MARKET_CAP_ID=2
# Get the trigger address from previous Deploy forge script
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`
# Execute on the trigger contract, WAVS will pick this up and submit the result
# on chain via the operators.
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast
```

Show the result from the triggered service

```bash docci-output-contains="LTC"
TRIGGER_ID=1 make show-result
```
