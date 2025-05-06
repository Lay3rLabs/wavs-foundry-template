# WAVS Multi-Operator Example

This example demonstrates how to run a WAVS service with multiple operators. It builds upon the main example in the root directory but extends it to use two separate operators.

## Prerequisites

Make sure you have completed the system requirements setup from the main [README.md](../README.md).

## Directory Structure

```
multiple-example/
├── .env1                # Environment variables for operator 1
├── .env2                # Environment variables for operator 2
├── docker-compose.yml   # Multi-operator Docker configuration
├── README.md            # This file
└── start_multi.sh       # Script to start both operators
```

## Multi-Operator Flow

1. Start an Ethereum node (anvil)
2. Deploy EigenLayer contracts
3. Start two WAVS instances (with different ports)
4. Deploy service contracts
5. Deploy the WASI component
6. Register both operators with the service
7. Trigger the service and observe multiple operators submitting results

## Setup and Run

### Create & Start

```bash
cd multiple-example/

sh ./create-aggregator.sh

sh ./create-operator.sh 1
sh ./create-operator.sh 2
```

### Start

```bash docci-background
# - Shows operators being used
# - Deploys Eigen Contracts
# - Funds Aggregator wallet (from .aggregator.env)
# - Start WAVS services using docker-compose
sh start.sh
```


```bash
# new terminal, run below
cd $(git rev-parse --show-toplevel)

# Wait for deployment to complete (check for start.log)
while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done
docker compose -f docker-compose-multi.yml logs -f &

# Upload smart contracts from some account
export DEPLOYER_PK=$(cat ./.nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)

forge create SimpleSubmit --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`


# Deploy the WASI component service & upload to each WAVS instance
# (required until we can read components from upstream registry)
# WAVS1
export COMPONENT_FILENAME=evm_price_oracle.wasm
WAVS_ENDPOINT="http://127.0.0.1:8000" AGGREGATOR_URL="http://127.0.0.1:8001" sh ./script/build_service.sh
# WAVS2
WAVS_ENDPOINT=http://127.0.0.1:9000 make upload-component


# Upload service.json to IPFS
ipfs_cid=`IPFS_ENDPOINT=http://127.0.0.1:5001 SERVICE_FILE=.docker/service.json make upload-to-ipfs`

export SERVICE_URL="http://127.0.0.1:8080/ipfs/${ipfs_cid}"
WAVS_ENDPOINT="http://127.0.0.1:8000" CREDENTIAL=${DEPLOYER_PK} make deploy-service
WAVS_ENDPOINT="http://127.0.0.1:9000" CREDENTIAL=${DEPLOYER_PK} make deploy-service



# Register Operators
source multiple-example/.operator1.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
ENV_FILE=multiple-example/.operator1.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# Operator 2
source multiple-example/.operator2.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
ENV_FILE=multiple-example/.operator2.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# Update threshold weight
ECDSA_CONTRACT=`cat .nodes/avs_deploy.json | jq -r .addresses.stakeRegistry`

# 1.8x a single operator weight (requires 2/3 of registered operators)
cast send ${ECDSA_CONTRACT} "updateStakeThreshold(uint256)" 1782625057707873 --rpc-url http://localhost:8545 --private-key ${DEPLOYER_PK}

# Verify registration for operators
make operator-list


# Trigger the service (request CMC ID price)
export COIN_MARKET_CAP_ID=2
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast


# Show results
TRIGGER_ID=`make get-trigger | grep "TriggerID:" | awk '{print $2}'`
TRIGGER_ID=${TRIGGER_ID} make show-result
```
