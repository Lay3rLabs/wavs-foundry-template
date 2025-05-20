#!/usr/bin/bash
set -e
set -x


# ensure $1 is input, if not error
if [ -z "$1" ]; then
    echo "Usage: $0 <address> [minimum_ether_balance]"
    echo "Ex: $0 0x841120433b8a543C17A75D7eDBaD41B70cB20034 0.001"
    exit 1
fi

ADDRESS=$1
MINIMUM_BALANCE=${2:-0.0001}

if [ -z "$DEPLOY_ENV" ]; then
    DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)
fi
if [ -z "$RPC_URL" ]; then
    RPC_URL=`sh ./script/get-rpc.sh`
fi

SP=""; if [ "$(uname)" == *"Darwin"* ]; then SP=" "; fi

cd $(git rev-parse --show-toplevel) || exit 1

mkdir -p .docker

CURRENT_BALANCE=`cast balance --ether $ADDRESS --rpc-url=${RPC_URL}`
if [ $(echo "$CURRENT_BALANCE > $MINIMUM_BALANCE" | bc) -eq 1 ]; then
    echo "Account already funded (${CURRENT_BALANCE}) to expected minimum (${MINIMUM_BALANCE}), exiting..."
    exit 0
fi


if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    # auto fund exact expected for local
    cast rpc anvil_setBalance "${ADDRESS}" `cast to-wei ${MINIMUM_BALANCE}` --rpc-url ${RPC_URL} > /dev/null
    BAL=`cast balance --ether $ADDRESS --rpc-url=${RPC_URL}`
    echo "Account \`${ADDRESS}\` funded with ${BAL}ether"
    exit 0
fi


# == Production ==
DEPLOYER_PK=${DEPLOYER_PK:-$(jq -r .accounts[0].private_key .docker/deployer.json)}
DEPLOYER_ADDRESS=`cast wallet address $DEPLOYER_PK`
DEPLOYER_BAL=`cast balance --ether $DEPLOYER_ADDRESS --rpc-url=${RPC_URL}`

if [ $(echo "$DEPLOYER_BAL > $MINIMUM_BALANCE" | bc) -eq 1 ]; then
    echo "Deployer balance is $DEPLOYER_BAL"
    echo "Deployer has enough balance to fund the address, sending..."
    cast send ${ADDRESS} --value `cast to-wei ${MINIMUM_BALANCE}` --rpc-url ${RPC_URL} --private-key ${DEPLOYER_PK}
else
    echo "Deployer balance is $DEPLOYER_BAL"
    echo "Deployer does not have enough balance to fund ${ADDRESS}, manually send some funds..."
    sleep 5

    while true; do
        BALANCE=`cast balance --ether $ADDRESS --rpc-url=${RPC_URL}`
        if [ "$BALANCE" != "0.000000000000000000" ]; then
            break
        fi
        echo "      [!] Waiting for balance to be funded by another account to this account..."
        sleep 5
    done
fi

BALANCE=`cast balance --ether $ADDRESS --rpc-url=${RPC_URL}`
echo "Account balance is now $BALANCE"
