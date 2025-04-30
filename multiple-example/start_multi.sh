#!/bin/bash

# set -x
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/lay3rlabs/wavs-middleware:0.4.0-alpha.5
LOG_FILE="$GIT_ROOT/.docker/start.log"
export DOCKER_DEFAULT_PLATFORM=linux/amd64

# Create parent directories if they don't exist
# mkdir -p ../.docker
# mkdir -p ../.nodes

# Remove log file if it exists
rm $LOG_FILE 2> /dev/null || true

# Get operator keys from .env files or create new ones if they don't exist
if [ ! -f ../.docker/operator1.json ] || [ ! -f ../.docker/operator2.json ]; then
  echo "Creating new operator keys..."
  make create-operators
fi

# Load environment variables from .env files
source .env1
OPERATOR1_PK=$WAVS_CLI_EVM_CREDENTIAL
OPERATOR1_MNEMONIC=$WAVS_SUBMISSION_MNEMONIC

source .env2
OPERATOR2_PK=$WAVS_CLI_EVM_CREDENTIAL
OPERATOR2_MNEMONIC=$WAVS_SUBMISSION_MNEMONIC

# Verify keys exist
if [[ -z "$OPERATOR1_PK" ]] || [[ -z "$OPERATOR1_MNEMONIC" ]]; then
  echo "Operator 1 keys missing. Please run 'make create-operators'"
  exit 1
fi

if [[ -z "$OPERATOR2_PK" ]] || [[ -z "$OPERATOR2_MNEMONIC" ]]; then
  echo "Operator 2 keys missing. Please run 'make create-operators'"
  exit 1
fi

echo "Using Operator 1 Address: $(cast wallet address --private-key ${OPERATOR1_PK})"
echo "Using Operator 2 Address: $(cast wallet address --private-key ${OPERATOR2_PK})"

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
cd ${GIT_ROOT} && docker run --rm --network host --env-file multiple-example/.env1 -v ./.nodes:/root/.nodes "$MIDDLEWARE_IMAGE"
cd multiple-example
echo "EigenLayer contracts deployed"

# Start WAVS services using docker-compose
echo "Starting WAVS services for both operators..."
cd ${GIT_ROOT} && docker compose -f docker-compose-multi.yml up --remove-orphans -d
trap "cd ${GIT_ROOT} && docker compose -f docker-compose-multi.yml down && echo -e '\nKilled WAVS services'" EXIT

# Mark successful startup
echo "Multi-operator environment started successfully"
date +%s > $LOG_FILE

# Keep running until interrupted
echo "Press Ctrl+C to stop the services"
wait $anvil_pid
