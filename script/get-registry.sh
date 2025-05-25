#!/bin/bash

cd `git rev-parse --show-toplevel` || exit

DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

# Set RPC_URL based on DEPLOY_ENV
if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    REGISTRY=localhost:8090
elif [ "$DEPLOY_ENV" = "TESTNET" ]; then
    REGISTRY=wa.dev
else
    echo "Unknown DEPLOY_ENV: $DEPLOY_ENV"
    exit 1
fi

echo "${REGISTRY}"
