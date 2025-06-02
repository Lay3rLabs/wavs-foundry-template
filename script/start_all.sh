#!/bin/bash

set -e

if [ -f .env ] && grep -q '^TESTNET_RPC_URL=' .env; then
  TESTNET_RPC_URL=$(grep -E '^TESTNET_RPC_URL=' .env | cut -d '=' -f2- | tr -d '"')
else
  rpc_url="https://holesky.drpc.org"
  echo "No TESTNET_RPC_URL found in .env, using default ${rpc_url}"
  TESTNET_RPC_URL=${rpc_url}
fi

PORT=8545
MIDDLEWARE_IMAGE=ghcr.io/lay3rlabs/wavs-middleware:0.4.0-beta.6
FORK_RPC_URL=${FORK_RPC_URL:-"${TESTNET_RPC_URL}"}
DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

## == Start watcher ==
rm $LOG_FILE 2> /dev/null || true


## == Base Anvil Testnet Fork ==
if [ "$DEPLOY_ENV" = "TESTNET" ]; then
  echo "Running in testnet mode, nothing to do"
  exit 0
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
  docker compose ${FILES} pull
  docker compose ${FILES} up --force-recreate -d
  trap "docker compose ${FILES} down --remove-orphans && docker kill wavs-1 wavs-aggregator-1 > /dev/null 2>&1 && echo -e '\nKilled IPFS + Local WARG, and wavs instances'" EXIT

  echo "Started..."
  wait
fi
