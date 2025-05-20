#!/bin/bash

set -e

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/lay3rlabs/wavs-middleware:0.4.0-beta.2
FORK_RPC_URL=${FORK_RPC_URL:-"https://ethereum-holesky-rpc.publicnode.com"}

## == Start watcher ==
rm $LOG_FILE 2> /dev/null || true

## == Base Anvil Testnet Fork ==
DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)
if [ "$DEPLOY_ENV" = "LOCAL" ]; then
  anvil --fork-url ${FORK_RPC_URL} --port ${PORT} &
  anvil_pid=$!
  trap "kill -9 $anvil_pid && echo -e '\nKilled anvil'" EXIT
  while ! cast block-number --rpc-url http://localhost:${PORT} > /dev/null 2>&1
  do
    sleep 0.25
  done
fi


docker compose up --force-recreate -d
trap "docker compose down --remove-orphans && echo -e '\nKilled IPFS + Local WARG'" EXIT

## == Setup Deployer
# echo "Using Address: $(cast wallet address --private-key ${OPERATOR_PK})"

# SP=""; if [ "$(uname)" == *"Darwin"* ]; then SP=" "; fi

# sed -i${SP}'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$/WAVS_CLI_EVM_CREDENTIAL=\"$OPERATOR_PK\"/" .env
# sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$OPERATOR_PK\"/" .env
# # this is what we generate other addresses in service manager based off of.
# sed -i${SP}'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$/WAVS_SUBMISSION_MNEMONIC=\"$OPERATOR_MNEMONIC\"/" .env

# # == WAVS & Aggregator ==
# docker compose up --remove-orphans &
# trap "docker compose down && echo -e '\nKilled WAVS'" EXIT

# fin

echo "Started..."
wait
