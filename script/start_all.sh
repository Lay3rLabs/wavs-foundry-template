#!/bin/bash

set -e

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/lay3rlabs/wavs-middleware:0.4.0-beta.2
LOG_FILE=.docker/start.log
OPERATOR_PK=${OPERATOR_PK:-""}
OPERATOR_MNEMONIC=${OPERATOR_MNEMONIC:-""}
export DOCKER_DEFAULT_PLATFORM=linux/amd64

## == Start watcher ==
rm $LOG_FILE 2> /dev/null || true

## == Base Anvil Testnet Fork ==
anvil --fork-url https://ethereum-holesky-rpc.publicnode.com --port ${PORT} &
anvil_pid=$!
trap "kill -9 $anvil_pid && echo -e '\nKilled anvil'" EXIT
while ! cast block-number --rpc-url http://localhost:${PORT} > /dev/null 2>&1
do
  sleep 0.25
done

if [[ -z "$OPERATOR_PK" ]]; then
  echo "You must set OPERATOR_PK"
  exit 1
fi
if [[ -z "$OPERATOR_MNEMONIC" ]]; then
  echo "You must set OPERATOR_MNEMONIC"
  exit 1
fi

## == Eigenlayer ==
# setup
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes "$MIDDLEWARE_IMAGE"


## == Setup Deployer
echo "Using Address: $(cast wallet address --private-key ${OPERATOR_PK})"

SP=""
if [[ "$(uname)" == *"Darwin"* ]]; then
  SP=" "
fi

sed -i${SP}'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$/WAVS_CLI_EVM_CREDENTIAL=\"$OPERATOR_PK\"/" .env
sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$OPERATOR_PK\"/" .env
# this is what we generate other addresses in service manager based off of.
sed -i${SP}'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$/WAVS_SUBMISSION_MNEMONIC=\"$OPERATOR_MNEMONIC\"/" .env

# == WAVS & Aggregator ==
docker compose -f docker-compose.yml -f docker-compose.telemetry.yml up --remove-orphans &
trap "docker compose down && echo -e '\nKilled WAVS'" EXIT

# fin
date +%s > $LOG_FILE
wait
