#!/bin/bash

set -e

# Take first argument as the version to upgrade to
VERSION=$1

# TODO: make this conditional, just for docker
SUDO="sudo"

# pull this version to ensure we have it
if ! ${SUDO} docker pull ghcr.io/lay3rlabs/wavs:${VERSION}; then
    echo "Failed to pull ghcr.io/lay3rlabs/wavs:${VERSION}"
    exit 1
fi

# Update Makefile
sed -i "s/ghcr.io\/lay3rlabs\/wavs:[^\s]+/ghcr.io\/lay3rlabs\/wavs:${VERSION}/g" Makefile

# Update docker-compose.yml
sed -i "s/   image: \"ghcr.io\/lay3rlabs\/wavs:[^\"]+/   image: \"ghcr.io\/lay3rlabs\/wavs:${VERSION}/g" docker-compose.yml

# Update Cargo.toml (for crates dependencies)
sed -i "s/wavs-wasi-chain = \"[^\"]+/wavs-wasi-chain = \"${VERSION}/g" Cargo.toml

# Update [package.metadata.component] in components/*/Cargo.toml (for wit)
sed -i "s/wavs:worker\/layer-trigger-world@[^\"]+/wavs:worker\/layer-trigger-world@${VERSION}/g" components/*/Cargo.toml

# Rebuild with cargo component build

 