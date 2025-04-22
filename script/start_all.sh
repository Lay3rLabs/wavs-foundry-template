#!/bin/bash

set -e

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/reecepbcups/wavs-middleware:0.0.1
LOG_FILE=.docker/start.log
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


## == Eigenlayer ==
# setup
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes "$MIDDLEWARE_IMAGE"
PRIVATE_KEY=$(cat .nodes/deployer)
# operator register
docker run --rm --network host --env-file .env -v ./.nodes:/root/.nodes --entrypoint /wavs/register.sh "$MIDDLEWARE_IMAGE" "$PRIVATE_KEY"


## == Setup Deployer
echo "Using Address: $(cast wallet address --private-key ${PRIVATE_KEY})"

SP=""
if [[ "$(uname)" == *"Darwin"* ]]; then
  SP=" "
fi

sed -i${SP}'' -e "s/^WAVS_CLI_ETH_CREDENTIAL=.*$/WAVS_CLI_ETH_CREDENTIAL=\"$PRIVATE_KEY\"/" .env
sed -i${SP}'' -e "s/^WAVS_SUBMISSION_CREDENTIAL=.*$/WAVS_SUBMISSION_CREDENTIAL=\"$PRIVATE_KEY\"/" .env
sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$PRIVATE_KEY\"/" .env


# == WAVS & Aggregator ==
docker compose up --remove-orphans &
trap "docker compose down && echo -e '\nKilled WAVS'" EXIT

# fin
date +%s > $LOG_FILE
wait
