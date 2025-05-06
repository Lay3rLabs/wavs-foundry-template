# WAVS Multi-Operator Testnet Example

This example demonstrates how to run a WAVS service with multiple operators. It builds upon the main example in the root directory but extends it to use two separate operators.

## Prerequisites

Make sure you have completed the system requirements setup from the main [README.md](../README.md).

## Setup and Run

### Create & Start

```bash
cd testnet/

sh ./create-aggregator.sh

sh ./create-operator.sh 1
sh ./create-operator.sh 2
```

### Start

```bash
# - Shows operators being used
# - Deploys Eigen Contracts
# - Funds Aggregator wallet (from .aggregator.env)
# - Start WAVS services using docker-compose
sh start.sh
```

## Upload standard smart contracts

```bash
cd $(git rev-parse --show-toplevel)

# This Deployer can be any private key, using
# pre funded account for simplicity.
export DEPLOYER_PK=$(cat ./.nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)

forge create SimpleSubmit --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```

```bash
cd $(git rev-parse --show-toplevel)

# Wait for deployment to complete (check for start.log)
while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done
docker compose -f docker-compose-multi.yml logs -f &


# Deploy the WASI component service & upload to each WAVS instance
# (required until we can read components from upstream registry)
export COMPONENT_FILENAME=evm_price_oracle.wasm
WAVS_ENDPOINT="http://127.0.0.1:8000" AGGREGATOR_URL="http://127.0.0.1:8001" sh ./script/build_service.sh
WAVS_ENDPOINT=http://127.0.0.1:9000 make upload-component


# Upload service.json to IPFS
ipfs_cid=`IPFS_ENDPOINT=http://127.0.0.1:5001 SERVICE_FILE=.docker/service.json make upload-to-ipfs`

export SERVICE_URL="http://127.0.0.1:8080/ipfs/${ipfs_cid}"
WAVS_ENDPOINT="http://127.0.0.1:8000" CREDENTIAL=${DEPLOYER_PK} make deploy-service
WAVS_ENDPOINT="http://127.0.0.1:9000" CREDENTIAL=${DEPLOYER_PK} make deploy-service

# Register Operators
source testnet/.operator1.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
ENV_FILE=testnet/.operator1.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# Operator 2
source testnet/.operator2.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
ENV_FILE=testnet/.operator2.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# Update threshold weight
# 1.8x a single operator weight (requires 2/3 of registered operators)
ECDSA_CONTRACT=`cat .nodes/avs_deploy.json | jq -r .addresses.stakeRegistry`
cast send ${ECDSA_CONTRACT} "updateStakeThreshold(uint256)" 1782625057707873 --rpc-url http://localhost:8545 --private-key ${DEPLOYER_PK}

# Verify registration for operators
make operator-list
```

## Contract call and aggregation

```bash
# Trigger the service (request CMC ID price)
export COIN_MARKET_CAP_ID=2
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast

TRIGGER_ID=`make get-trigger | grep "TriggerID:" | awk '{print $2}'`
TRIGGER_ID=${TRIGGER_ID} make show-result
```
