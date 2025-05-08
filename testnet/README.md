# WAVS Multi-Operator Testnet Example

This example demonstrates how to run a WAVS service with multiple operators. It builds upon the main example in the root directory but extends it to use two separate operators.

## Prerequisites

Make sure you have completed the system requirements setup from the main [README.md](../README.md).

## Setup and Run

### Create & Start

```bash
cd testnet/ || true

export RPC_URL=${RPC_URL:-"https://ethereum-holesky.publicnode.com"} # TODO: set as testnet / .env help here

sh ./create-aggregator.sh

cast wallet new-mnemonic --json > .docker/funded_key.json
export FUNDED_KEY=`jq -r .accounts[0].private_key .docker/funded_key.json`

# TODO: change to allow for TESTNET vs LOCAL gen (require another argument)
# TODO: also put funded key in its own .env instead of here
sh ./create-operator.sh 1 ${FUNDED_KEY}
sh ./create-operator.sh 2 ${FUNDED_KEY}

mv /root/wavs-foundry-template/testnet/.operator1.env /root/wavs-1/.env
mv /root/wavs-foundry-template/testnet/.operator2.env /root/wavs-2/.env
mv /root/wavs-foundry-template/testnet/.aggregator.env /root/wavs-agg/.env

# TODO: Faucet funds to FUNDED_KEY

# TODO: faucet fund deployer, aggregator here (operators later)
DEPLOYER_ADDR=`cast wallet address --private-key $FUNDED_KEY`
# cast balance --ether ${DEPLOYER_ADDR} --rpc-url $RPC_URL
echo "Balance of ${DEPLOYER_ADDR} to fund"

# TODO: move operator index 1 payments here too for initial registration?
source /root/wavs-agg/.env
AGGREGATOR_ADDR=$(cast wallet address --private-key ${WAVS_AGGREGATOR_CREDENTIAL})
# cast balance --ether $AGGREGATOR_ADDR --rpc-url ${RPC_URL}
cast send ${AGGREGATOR_ADDR} --rpc-url ${RPC_URL} --private-key ${FUNDED_KEY} --value 0.005ether

# deploys AVS contracts
sh start.sh
```

## Upload standard smart contracts

```bash
# new terminal
cd $(git rev-parse --show-toplevel)

# Wait for deployment to complete (check for start.log)
# while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done

# This Deployer can be any private key, using
# pre funded account for simplicity.
export DEPLOYER_PK=$(cat ./.nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)


forge create SimpleSubmit --json --broadcast -r ${RPC_URL} --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r ${RPC_URL} --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```

## Setup WAVS instances

```bash
cd $(git rev-parse --show-toplevel)

# Start wavs
(cd /root/wavs-1 && sh start.sh)
(cd /root/wavs-2 && sh start.sh)

# start aggregator / IPFS
(cd /root/wavs-agg && docker compose up -d)


# Deploy the WASI component service & upload to each WAVS instance
# (required until we can read components from upstream registry)
export COMPONENT_FILENAME=evm_price_oracle.wasm
export TRIGGER_CHAIN=holesky
export SUBMIT_CHAIN=holesky
WAVS_ENDPOINT="http://5.161.229.43:8000" AGGREGATOR_URL="http://5.161.229.43:8001" sh ./script/build_service.sh
WAVS_ENDPOINT=http://5.161.229.43:9000 make upload-component

# Upload service.json to IPFS & deploy service with it
ipfs_cid=`IPFS_ENDPOINT=http://5.161.229.43:5001 SERVICE_FILE=.docker/service.json make upload-to-ipfs`
export SERVICE_URL="http://5.161.229.43:8080/ipfs/${ipfs_cid}"
curl ${SERVICE_URL}

WAVS_ENDPOINT="http://5.161.229.43:8000" CREDENTIAL=${DEPLOYER_PK} make deploy-service
WAVS_ENDPOINT="http://5.161.229.43:9000" CREDENTIAL=${DEPLOYER_PK} make deploy-service
```

## Register operators -> Eigenlayer

```bash
# this was moved in a previous step, ensure it is not the default values
source /root/wavs-1/.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`

# https://holesky-faucet.pk910.de/
# fund operator acc so they can register
OPERATOR1_ADDR=`cast wallet address --private-key $AVS_PRIVATE_KEY`
echo ${OPERATOR1_ADDR}
cast send ${OPERATOR1_ADDR} --private-key ${DEPLOYER_PK} --value 0.0025ether --rpc-url ${RPC_URL}
cast balance --ether ${OPERATOR1_ADDR} --rpc-url ${RPC_URL}

# ENV_FILE=testnet/.operator1.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register
ENV_FILE=/root/wavs-1/.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register
# cast call --rpc-url https://ethereum-holesky.publicnode.com 0x3f1c547b21f65e10480de3ad8e19faac46c95034 "balanceOf(address)(uint256)" ${OPERATOR1_ADDR}

# Operator 2
source /root/wavs-2/.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
OPERATOR2_ADDR=`cast wallet address --private-key $AVS_PRIVATE_KEY`
echo ${OPERATOR2_ADDR}
cast send ${OPERATOR2_ADDR} --private-key ${DEPLOYER_PK} --value 0.025ether --rpc-url ${RPC_URL}
cast balance --ether ${OPERATOR2_ADDR} --rpc-url ${RPC_URL}

ENV_FILE=/root/wavs-2/.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# register a 3rd new wallet operator here to test with a 2/3 config, just don't run a node for it.

# Update threshold weight
# 1.8x a single operator weight (requires 2/3 of registered operators)
ECDSA_CONTRACT=`cat .nodes/avs_deploy.json | jq -r .addresses.stakeRegistry`
cast send ${ECDSA_CONTRACT} "updateStakeThreshold(uint256)" 1782625057707873 --rpc-url ${RPC_URL} --private-key ${DEPLOYER_PK}

# Verify registration for operators
make operator-list
```

## Contract call and aggregation

```bash
# Trigger the service (request CMC ID price)
export COIN_MARKET_CAP_ID=2
# TODO: change from anvil -> just PRIVATE_KEY
export ANVIL_PRIVATE_KEY=${DEPLOYER_PK}
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url ${RPC_URL} --broadcast

TRIGGER_ID=`make get-trigger | grep "TriggerID:" | awk '{print $2}'`
TRIGGER_ID=${TRIGGER_ID} make show-result
```
