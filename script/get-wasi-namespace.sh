#!/bin/bash

if [ -z "$REGISTRY" ]; then
    echo "REGISTRY is not set. Please set the REGISTRY environment variable." && exit 1
fi

# ===

cd `git rev-parse --show-toplevel` || exit

DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    export PKG_NAMESPACE="example"
    echo ${PKG_NAMESPACE}
    exit 0
else
    read -p "Enter the PKG_NAMESPACE for ${REGISTRY}: " namespace

    export PKG_NAMESPACE="${namespace}"
    echo "${PKG_NAMESPACE}"
fi
