#!/usr/bin/bash
# set -e
SP="" #; if [[ "$(uname)" == *"Darwin"* ]]; then SP=" "; fi

# testnet or local
DEPLOY_ENV=${DEPLOY_ENV:-TESTNET}
# FUNDED_KEY

mkdir -p .docker

cp .env.example .env

# Create New, fund later
cast wallet new-mnemonic --json > .docker/deployer.json
export DEPLOYER_PK=`jq -r .accounts[0].private_key .docker/deployer.json`
sed -i${SP}'' -e "s/^FUNDED_KEY=.*$/FUNDED_KEY=$DEPLOYER_PK/" .env

# echo to fund the account
ADDR=`cast wallet address $DEPLOYER_PK`
echo "Fund deployer ${ADDR} with some ETH, or change this value in the .env"

# TODO: poll for iteration of the balance here

cast b --ether ${ADDR} --rpc-url=${RPC_URL}
