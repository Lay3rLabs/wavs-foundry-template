#!/usr/bin/bash
# set -e
SP=""; if [[ "$(uname)" == *"Darwin"* ]]; then SP=" "; fi

# if DEPLOY_ENV is not set, grab it from the ./script/get-deploy-status.sh
if [ -z "$DEPLOY_ENV" ]; then
    DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)
fi
if [ -z "$RPC_URL" ]; then
    RPC_URL=`sh ./script/get-rpc.sh`
fi

if [ ! -f .env ]; then
    echo ".env file not found, attempting to copy create"
    cp .env.example .env
    if [ $? -ne 0 ]; then
        echo "Failed to copy .env.example to .env"
        exit 1
    fi
fi

mkdir -p .docker

# Create new deployer
cast wallet new-mnemonic --json > .docker/deployer.json
export DEPLOYER_PK=`jq -r .accounts[0].private_key .docker/deployer.json`
export DEPLOYER_ADDRESS=`cast wallet address $DEPLOYER_PK`
sed -i${SP}'' -e "s/^FUNDED_KEY=.*$/FUNDED_KEY=$DEPLOYER_PK/" .env


if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    # Good DevEx, auto fund the deployer
    cast rpc anvil_setBalance "${DEPLOYER_ADDRESS}" '15000000000000000000' --rpc-url ${RPC_URL} > /dev/null

    BAL=`cast balance --ether $DEPLOYER_ADDRESS --rpc-url=${RPC_URL}`
    echo "Local deployer \`${DEPLOYER_ADDRESS}\` funded with ${BAL}ether"
else
    # New account on testnet, must be funded externally (i.e. metamask)
    echo "Fund deployer ${DEPLOYER_ADDRESS} with some ETH, or change this value in the .env"
    sleep 5

    while true; do
        BALANCE=`cast balance --ether $DEPLOYER_ADDRESS --rpc-url=${RPC_URL}`
        if [ "$BALANCE" != "0.000000000000000000" ]; then
            echo "Deployer balance is now $BALANCE"
            break
        fi
        echo "    [!] Waiting for balance to be funded by another account to this deployer..."
        sleep 5
    done
fi

