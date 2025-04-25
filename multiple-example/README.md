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

```bash
# Start the multi-operator environment
cd multiple-example
./start_multi.sh


# Wait for deployment to complete (check for start.log)
while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done
docker compose -f docker-compose-multi.yml logs

GIT_ROOT=$(git rev-parse --show-toplevel)
cd ${GIT_ROOT}

# In a new terminal, deploy the service contracts
export DEPLOYER_PK=$(cat ./.nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)

forge create SimpleSubmit --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r http://127.0.0.1:8545 --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`

# Deploy the WASI component service & upload to each WAVS instance
COMPONENT_FILENAME=eth_price_oracle.wasm AGGREGATOR_URL=http://127.0.0.1:8001 sh ./script/build_service.sh
COMPONENT_FILENAME=eth_price_oracle.wasm WAVS_ENDPOINT=http://127.0.0.1:9000 make upload-component

# Now upload it to the 2nd wavs instance manually (since it's not watching for events to auto pull configs)
# WAVS_ENDPOINT="http://127.0.0.1:9000" SERVICE_CONFIG_FILE=.docker/service.json CREDENTIAL=${DEPLOYER_PK} make deploy-service

SERVICE_CONFIG_FILE=.docker/service.json CREDENTIAL=${DEPLOYER_PK} make deploy-service
WAVS_ENDPOINT="http://127.0.0.1:9000" SERVICE_CONFIG_FILE=.docker/service.json CREDENTIAL=${DEPLOYER_PK} make deploy-service

# Register both operators
# Operator 1
source multiple-example/.env1
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
docker run --rm --network host --env-file multiple-example/.env1 -v ./.nodes:/root/.nodes --entrypoint /wavs/register.sh "ghcr.io/lay3rlabs/wavs-middleware:0.4.0-alpha.5" "$AVS_PRIVATE_KEY"

# Operator 2
source multiple-example/.env2
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
docker run --rm --network host --env-file multiple-example/.env2 -v ./.nodes:/root/.nodes --entrypoint /wavs/register.sh "ghcr.io/lay3rlabs/wavs-middleware:0.4.0-alpha.5" "$AVS_PRIVATE_KEY"

# Verify registration for both operators
docker run --rm --network host --env-file multiple-example/.env1 -v ./.nodes:/root/.nodes --entrypoint /wavs/list_operator.sh ghcr.io/lay3rlabs/wavs-middleware:0.4.0-alpha.5

# Fund the aggregator account (only 1 is run)
source multiple-example/.env1
cast send $(cast wallet address --private-key ${WAVS_AGGREGATOR_CREDENTIAL}) --rpc-url http://localhost:8545 --private-key ${DEPLOYER_PK} --value 1ether

# Trigger the service (request BTC price)
export COIN_MARKET_CAP_ID=1
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url http://localhost:8545 --broadcast

# Show results
cd multiple-example
make get-trigger
TRIGGER_ID=1 make show-result
```

## How It Works

- The setup runs a single Anvil node for both operators
- Each operator has its own environment variables for unique mnemonics and ports
- Each operator's WAVS instance runs on a different port to avoid conflicts
- Both operators are registered with the same service manager
- When a trigger event is emitted, both operators will execute the WASI component and submit results
- The service manager will validate submissions from both operators

For more details on the WAVS architecture, see the main [README.md](../README.md) and the [docs](../docs/) directory.
