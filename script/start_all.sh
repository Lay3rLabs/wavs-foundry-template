#!/bin/bash

set -e

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/reecepbcups/wavs-middleware:0.0.2
LOG_FILE=.docker/start.log
OPERATOR_PRIVATE_KEY=${OPERATOR_PRIVATE_KEY:-""}
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

if [[ -z "$OPERATOR_PRIVATE_KEY" ]]; then
  echo "You must set OPERATOR_PRIVATE_KEY"
  exit 1
fi
if [[ -z "$OPERATOR_MNEMONIC" ]]; then
  echo "You must set OPERATOR_MNEMONIC"
  exit 1
fi

## == Eigenlayer ==
# setup
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes "$MIDDLEWARE_IMAGE"
# PRIVATE_KEY=$(cat .nodes/deployer)
# operator register
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes --entrypoint /wavs/register.sh "$MIDDLEWARE_IMAGE" "$OPERATOR_PRIVATE_KEY"


## == Setup Deployer
echo "Using Address: $(cast wallet address --private-key ${OPERATOR_PRIVATE_KEY})"

SP=""
if [[ "$(uname)" == *"Darwin"* ]]; then
  SP=" "
fi

sed -i${SP}'' -e "s/^WAVS_CLI_ETH_CREDENTIAL=.*$/WAVS_CLI_ETH_CREDENTIAL=\"$OPERATOR_PRIVATE_KEY\"/" .env
sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$OPERATOR_PRIVATE_KEY\"/" .env
sed -i${SP}'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$/WAVS_SUBMISSION_MNEMONIC=\"$OPERATOR_MNEMONIC\"/" .env
# TODO: WAVS_SUBMISSION_MNEMONIC CAN NOT BE A PRIVATE KEY, need to figure out which / how to register
# https://github.com/Lay3rLabs/WAVS/issues/555#issuecomment-2821757422


# == WAVS & Aggregator ==
docker compose up --remove-orphans &
trap "docker compose down && echo -e '\nKilled WAVS'" EXIT

# fin
date +%s > $LOG_FILE
wait
