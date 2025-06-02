#!/bin/bash

cd `git rev-parse --show-toplevel` || exit

DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    IPFS_GATEWAY=http://127.0.0.1:8080/ipfs/
elif [ "$DEPLOY_ENV" = "TESTNET" ]; then
    IPFS_GATEWAY=https://gateway.pinata.cloud/ipfs/
else
    echo "Unknown DEPLOY_ENV: $DEPLOY_ENV"
    exit 1
fi

echo "${IPFS_GATEWAY}"
