#!/bin/bash

cd `git rev-parse --show-toplevel` || exit

DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    IPFS_GATEWAY=http://localhost:8080/ipfs/
elif [ "$DEPLOY_ENV" = "TESTNET" ]; then
    IPFS_GATEWAY=https://gateway.pinata.cloud/ipfs/
else
    echo "Unknown DEPLOY_ENV: $DEPLOY_ENV"
    return
fi

echo "${IPFS_GATEWAY}"
