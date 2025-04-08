#!/bin/bash

set -e

# Take first argument as the version to upgrade to
VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

# conditional sudo, just for docker
if groups | grep -q docker; then
  SUDO="";
else
  SUDO="sudo";
fi

# pull this version to ensure we have it
if ! ${SUDO} docker pull ghcr.io/lay3rlabs/wavs:${VERSION}; then
    echo "Invalid WAVS version, cannot pull ghcr.io/lay3rlabs/wavs:${VERSION}"
    exit 1
fi

# Update Makefile
sed -E -i "s/ghcr.io\/lay3rlabs\/wavs:[^ ]+/ghcr.io\/lay3rlabs\/wavs:${VERSION}/g" Makefile

# Update docker-compose.yml
sed -E -i "s/ghcr.io\/lay3rlabs\/wavs:[^\"]+/ghcr.io\/lay3rlabs\/wavs:${VERSION}/g" docker-compose.yml

# Update Cargo.toml (for crates dependencies)
sed -E -i "s/wavs-wasi-utils = \"[^\"]+/wavs-wasi-utils = \"${VERSION}/g" Cargo.toml

# Update [package.metadata.component] in components/*/Cargo.toml (for wit)
sed -E -i "s/wavs:worker\/layer-trigger-world@[^\"]+/wavs:worker\/layer-trigger-world@${VERSION}/g" components/*/Cargo.toml

# Rebuild with cargo component build in order to update bindings and Cargo.lock
rm components/*/src/bindings.rs
make wasi-build
