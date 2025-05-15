#!/bin/bash

set -e
# set -x

GIT_ROOT=$(git rev-parse --show-toplevel)

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/lay3rlabs/wavs-middleware:0.4.0-beta.2
LOG_FILE="$GIT_ROOT/.docker/start.log"
export DOCKER_DEFAULT_PLATFORM=linux/amd64

# Remove log file if it exists
rm $LOG_FILE 2> /dev/null || true

OPERATORS=`find . -type f -name ".operator[0-9].env" | sort -t r -k 2 -n`
if [ -z "$OPERATORS" ]; then
  echo "No operator files found. Please create at least one operator env file (sh create-operator.sh)."
  exit 1
fi

for file in ${OPERATORS}; do
  source $file
  OPERATOR_INDEX=$(echo $file | awk -F'.operator' '{print $2}' | grep -o '^[0-9]*')

  ETH_ADDR=$(cast wallet address --mnemonic "${WAVS_SUBMISSION_MNEMONIC}")
  echo "Using Operator ${OPERATOR_INDEX} Address: ${ETH_ADDR}"
done

# Start Anvil
echo "Starting Anvil..."
anvil --fork-url https://ethereum-holesky-rpc.publicnode.com --port ${PORT} &
anvil_pid=$!
trap "kill -9 $anvil_pid && echo -e '\nKilled anvil'" EXIT

# Wait for Anvil to start
while ! cast block-number --rpc-url http://localhost:${PORT} > /dev/null 2>&1
do
  sleep 0.25
done
echo "Anvil started successfully"

# Deploy EigenLayer contracts
echo "Deploying EigenLayer contracts..."
cd ${GIT_ROOT} && docker run --rm --network host --env-file testnet/.operator1.env -v ./.nodes:/root/.nodes "$MIDDLEWARE_IMAGE"
echo "EigenLayer contracts deployed"

echo "Funding WAVS Aggregator..."
source testnet/.aggregator.env
export DEPLOYER_PK=$(cat ./.nodes/deployer) # from eigenlayer deploy (funded account)
AGGREGATOR_ADDR=$(cast wallet address --private-key ${WAVS_AGGREGATOR_CREDENTIAL})
cast send ${AGGREGATOR_ADDR} --rpc-url http://localhost:8545 --private-key ${DEPLOYER_PK} --value 1ether

# Start WAVS services using docker-compose
echo "Starting WAVS services for both operators..."
cd ${GIT_ROOT} && docker compose -f testnet/docker-compose-multi.yml up --remove-orphans -d
trap "cd ${GIT_ROOT} && docker compose -f testnet/docker-compose-multi.yml down && echo -e '\nKilled WAVS services'" EXIT

# Mark successful startup
echo "Multi-operator environment started successfully"
date +%s > $LOG_FILE

# Keep running until interrupted
echo "Press Ctrl+C to stop the services"
wait $anvil_pid
