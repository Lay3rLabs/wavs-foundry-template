#!/bin/bash

if [ -z "$REGISTRY" ]; then
    echo "REGISTRY is not set. Please set the REGISTRY environment variable." && exit 1
fi
if [ -z "$PKG_NAME" ]; then
    echo "PKG_NAME is not set. Please set the PKG_NAME environment variable." && exit 1
fi
if [ -z "$PKG_VERSION" ]; then
    echo "PKG_VERSION is not set. Please set the PKG_VERSION environment variable." && exit 1
fi
if [ -z "$COMPONENT_FILENAME" ]; then
    echo "COMPONENT_FILENAME is not set. Please set the COMPONENT_FILENAME environment variable." && exit 1
fi

# ===

cd `git rev-parse --show-toplevel` || exit

if [ ! -f .env ]; then
    echo ".env file not found, attempting to copy create"
    cp .env.example .env

    if [ $? -ne 0 ]; then
        echo "Failed to copy .env.example to .env"
        exit 1
    fi
fi

DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)

# If local, we wkg upload TO local
if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    echo "LOCAL"
    # Local: example is the default namespace
    PKG_NAMESPACE=example

    # `failed to send request to registry server: error sending request for url`? - warg reset
    # TODO: root inclusion issue does not matter for localhost, why is it happening though?
    warg publish release --registry http://${REGISTRY} --name ${PKG_NAMESPACE}:${PKG_NAME} --version ${PKG_VERSION} ./compiled/${COMPONENT_FILENAME} || true
    exit 0
else
    read -p "Enter the PKG_NAMESPACE for wa.dev: " PKG_NAMESPACE
    warg publish release --registry https://${REGISTRY} --name ${PKG_NAMESPACE}:${PKG_NAME} --version ${PKG_VERSION} ./compiled/${COMPONENT_FILENAME} || true
fi

