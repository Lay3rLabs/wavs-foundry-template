#!/usr/bin/bash
# set -e
SP=""; if [[ "$(uname)" == *"Darwin"* ]]; then SP=" "; fi

# if DEPLOY_ENV is not set, grab it from the `task get-deploy-status`
if [ -z "$DEPLOY_ENV" ]; then
    DEPLOY_ENV=$(task get-deploy-status)
fi
if [ -z "$RPC_URL" ]; then
    RPC_URL=`task get-rpc`
fi

if [ ! -f .env ]; then
    echo ".env file not found, attempting to copy create"
    cp .env.example .env
    if [ $? -ne 0 ]; then
        echo "Failed to copy .env.example to .env"
        return
    fi
fi

mkdir -p .docker

# Create new deployer (if required)
create_funded_key() {
    echo "Creating new FUNDED_KEY..."
    export FUNDED_KEY=$(cast wallet new-mnemonic --json | jq -r '.accounts[0].private_key')
    sed -i${SP}'' -e "s/^FUNDED_KEY=.*$/FUNDED_KEY=$FUNDED_KEY/" .env
}

# Setup deployer key based on environment
if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    echo "Setting up LOCAL environment deployer"
    create_funded_key
else
    # Check for existing key in non-local environments
    export FUNDED_KEY=$(task config:funded-key)

    if [ -z "$FUNDED_KEY" ]; then
        echo "No FUNDED_KEY found in .env, creating new one"
        create_funded_key
    else
        echo "Using existing FUNDED_KEY from .env"
    fi
fi

# Get deployer address
export DEPLOYER_ADDRESS=$(cast wallet address "$FUNDED_KEY")

# Fund deployer based on environment
if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    # Auto-fund deployer in local environment
    echo "Funding local deployer..."
    cast rpc anvil_setBalance "$DEPLOYER_ADDRESS" '15000000000000000000' --rpc-url "$RPC_URL" > /dev/null

    BALANCE=$(cast balance --ether "$DEPLOYER_ADDRESS" --rpc-url="$RPC_URL")
    echo "Local deployer $DEPLOYER_ADDRESS funded with ${BALANCE} ETH"
else
    # Wait for external funding in testnet/mainnet
    echo "Please fund deployer $DEPLOYER_ADDRESS with ETH"
    echo "You can change this address in the .env file if needed"
    sleep 5

    echo "Waiting for funding..."
    while true; do
        BALANCE=$(cast balance --ether "$DEPLOYER_ADDRESS" --rpc-url="$RPC_URL")

        if [ "$BALANCE" != "0.000000000000000000" ]; then
            echo "Deployer funded! Balance: $BALANCE ETH for $DEPLOYER_ADDRESS"
            break
        fi

        echo "  [!] Waiting for balance increase $DEPLOYER_ADDRESS... (current: $BALANCE ETH)"
        sleep 5
    done
fi
