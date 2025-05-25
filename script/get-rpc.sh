#!/bin/bash

cd `git rev-parse --show-toplevel` || exit

# Convert to uppercase
DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

# Set RPC_URL based on DEPLOY_ENV
if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    RPC_URL=$(grep "^LOCAL_ETHEREUM_RPC_URL=" .env | cut -d '=' -f2)
elif [ "$DEPLOY_ENV" = "TESTNET" ]; then
    RPC_URL=$(grep "^TESTNET_RPC_URL=" .env | cut -d '=' -f2)
else
    echo "Unknown DEPLOY_ENV: $DEPLOY_ENV"
    exit 1
fi

echo "${RPC_URL}"
