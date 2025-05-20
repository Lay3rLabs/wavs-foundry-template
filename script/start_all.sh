#!/bin/bash

set -e

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/lay3rlabs/wavs-middleware:local-0.4.0-beta.3
FORK_RPC_URL=${FORK_RPC_URL:-"https://ethereum-holesky-rpc.publicnode.com"}

## == Start watcher ==
rm $LOG_FILE 2> /dev/null || true

## == Base Anvil Testnet Fork ==
DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)
if [ "$DEPLOY_ENV" = "TESTNET" ]; then
  echo "Running in testnet mode, nothing to do"
fi

if [ "$DEPLOY_ENV" = "LOCAL" ]; then
  anvil --fork-url ${FORK_RPC_URL} --port ${PORT} &
  anvil_pid=$!
  trap "kill -9 $anvil_pid && echo -e '\nKilled anvil'" EXIT
  while ! cast block-number --rpc-url http://localhost:${PORT} > /dev/null 2>&1
  do
    sleep 0.25
  done

  FILES="-f docker-compose.yml -f telemetry/docker-compose.yml"
  docker compose ${FILES} up --force-recreate -d
  trap "docker compose ${FILES} down --remove-orphans && docker kill wavs-1 wavs-aggregator-1 > /dev/null 2>&1 && echo -e '\nKilled IPFS + Local WARG, and wavs instances'" EXIT

  echo "Started..."
  wait
fi
