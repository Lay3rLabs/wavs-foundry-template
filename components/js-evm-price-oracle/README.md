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
npm --prefix ./components/js-evm-price-oracle/ install
```

## Build Component

Build all wasi components from the root of the repo. You can also run this command within each component directory.

```bash docci-output-contains="Successfully written"
# Builds only this component, not all.
WASI_BUILD_DIR=js-evm-price-oracle make wasi-build
```

## Execute Component

Run the component with the `wasi-exec` command in the root of the repo

```bash docci-output-contains="LTC"
COMPONENT_FILENAME=js_evm_price_oracle.wasm COIN_MARKET_CAP_ID=2 make wasi-exec
```

Build your smart contracts

```bash
forge build
```

### Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

```bash docci-background docci-delay-after=15
cp .env.example .env

# Create new operator
cast wallet new-mnemonic --json > .docker/operator1.json
export OPERATOR_MNEMONIC=`cat .docker/operator1.json | jq -r .mnemonic`
export OPERATOR_PK=`cat .docker/operator1.json | jq -r .accounts[0].private_key`

make start-all
```

Wait for full local deployment, then grab values

```bash docci-delay-after=2
while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done
```

Deploy the contracts

```bash docci-delay-after=1
export DEPLOYER_PK=$(cat .nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager .nodes/avs_deploy.json)

forge create SimpleSubmit --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```


Upload service

```bash docci-delay-per-cmd=2
export COMPONENT_FILENAME=js_evm_price_oracle.wasm

# === LOCAL ===
export IS_TESTNET=false
export WASM_DIGEST=$(make upload-component COMPONENT_FILENAME=$COMPONENT_FILENAME)

# === TESTNET ===
# - reference repo root README.md

# Build your service JSON with optional overrides in the script
sh ./script/build_service.sh

# Upload service.json to IPFS
ipfs_cid=`IPFS_ENDPOINT=http://127.0.0.1:5001 SERVICE_FILE=.docker/service.json make upload-to-ipfs`

SERVICE_URL="http://127.0.0.1:8080/ipfs/${ipfs_cid}" CREDENTIAL=${DEPLOYER_PK} make deploy-service
```


Register service specific operator

```bash docci-delay-per-cmd=2
source .env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`

# Faucet funds to the aggregator account to post on chain
cast send $(cast wallet address --private-key ${WAVS_AGGREGATOR_CREDENTIAL}) --rpc-url http://localhost:8545 --private-key ${DEPLOYER_PK} --value 1ether

# Register the operator with the WAVS service manager
AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# Verify registration
make operator-list
```

```bash docci-delay-after=2
# Request quote from CMC
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
