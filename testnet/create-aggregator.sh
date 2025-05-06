#!/bin/bash
set -e
SP=""; if [[ "$(uname)" == *"Darwin"* ]]; then SP=" "; fi

mkdir -p .docker

cp .env.example.aggregator .aggregator.env

# Create New, fund later
cast wallet new-mnemonic --json > .docker/aggregator.json
export AGG_PK=`jq -r .accounts[0].private_key .docker/aggregator.json`
sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$AGG_PK\"/" .aggregator.env
