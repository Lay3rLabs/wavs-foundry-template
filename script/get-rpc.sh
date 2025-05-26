#!/bin/bash

cd `git rev-parse --show-toplevel` || exit

DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    RPC_URL=$(grep "^LOCAL_ETHEREUM_RPC_URL=" .env)
elif [ "$DEPLOY_ENV" = "TESTNET" ]; then
    RPC_URL=$(grep "^TESTNET_RPC_URL=" .env)
else
    echo "Unknown DEPLOY_ENV: $DEPLOY_ENV"
    exit 1
fi

echo "${RPC_URL}" | cut -d '=' -f2
