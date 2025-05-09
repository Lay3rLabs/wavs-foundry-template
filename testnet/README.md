# WAVS Multi-Operator Testnet Example

This example demonstrates how to run a WAVS service with multiple operators. It builds upon the main example in the root directory but extends it to use two separate operators.

## Prerequisites

Make sure you have completed the system requirements setup from the main [README.md](../README.md).

## Setup and Run

### Create & Start

```bash
cd testnet/ || true

# The admin over the contracts / AVS
# TODO: put .env in the root of the foundry template, not in testnet?
export RPC_URL=${RPC_URL:-"https://ethereum-holesky.publicnode.com"}
DEPLOY_ENV=TESTNET sh ./create-deployer.sh

sh ./create-aggregator.sh

# these are funded later
sh ./create-operator.sh 1
sh ./create-operator.sh 2

mv ~/testnet/wavs-foundry-template/testnet/.operator1.env ~/testnet/wavs-1/.env
mv ~/testnet/wavs-foundry-template/testnet/.operator2.env ~/testnet/wavs-2/.env
mv ~/testnet/wavs-foundry-template/testnet/.aggregator.env ~/testnet/wavs-agg/.env

# Deploy AVS Contracts (prev: start.sh)
GIT_ROOT="$HOME/testnet/wavs-foundry-template"
docker run --rm --network host --env-file ${GIT_ROOT}/testnet/.env -v ${GIT_ROOT}/.nodes:/root/.nodes "ghcr.io/lay3rlabs/wavs-middleware:0.4.0-beta.2"
```

## Upload standard smart contracts

```bash
# new terminal
GIT_ROOT="$HOME/testnet/wavs-foundry-template"
cd ${GIT_ROOT}

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
GIT_ROOT="$HOME/testnet/wavs-foundry-template"
cd ${GIT_ROOT}

# Start wavs
(cd ~/testnet/wavs-1 && sh start.sh)
(cd ~/testnet/wavs-2 && sh start.sh)

# start aggregator / IPFS
(cd ~/testnet/wavs-agg && docker compose up -d)
# (cd ~/testnet/wavs-agg && docker compose down)
# (cd ~/testnet/wavs-agg && docker compose logs)


# Deploy the WASI component service & upload to each WAVS instance
# (required until we can read components from upstream registry)
export COMPONENT_FILENAME=evm_price_oracle.wasm
export TRIGGER_CHAIN=holesky
export SUBMIT_CHAIN=holesky

# TODO: move upload for each before we build it 9just for consistancy)
WAVS_ENDPOINT="http://3.75.195.165:8000" AGGREGATOR_URL="http://3.75.195.165:8001" sh ./script/build_service.sh
WAVS_ENDPOINT=http://3.75.195.165:9000 make upload-component

# Upload service.json to IPFS & deploy service with it
ipfs_cid=`IPFS_ENDPOINT=http://3.75.195.165:9501 SERVICE_FILE=.docker/service.json make upload-to-ipfs`
export SERVICE_URL="http://3.75.195.165:8080/ipfs/${ipfs_cid}"
curl ${SERVICE_URL}

WAVS_ENDPOINT="http://3.75.195.165:8000" CREDENTIAL=${DEPLOYER_PK} make deploy-service
WAVS_ENDPOINT="http://3.75.195.165:9000" CREDENTIAL=${DEPLOYER_PK} make deploy-service

# view all logs
# docker ps -q | xargs -L 1 -P `docker ps | wc -l` docker logs -f
```

## Register operators -> Eigenlayer

```bash
# this was moved in a previous step, ensure it is not the default values
source ~/testnet/wavs-1/.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`

# https://holesky-faucet.pk910.de/

# TODO: script to get operator address, send funds if needdd, then operator register
# fund operator acc so they can register
OPERATOR1_ADDR=`cast wallet address --private-key $AVS_PRIVATE_KEY`
echo ${OPERATOR1_ADDR}
cast send ${OPERATOR1_ADDR} --private-key ${DEPLOYER_PK} --value 0.0025ether --rpc-url ${RPC_URL}
# cast balance --ether ${OPERATOR1_ADDR} --rpc-url ${RPC_URL}

# TODO: I had to manually call this, also updated .env LOCAL_RPC to match the TESTNET_RPC_URL exactly.
GIT_ROOT=$HOME/testnet/wavs-foundry-template
ENV_FILE=${GIT_ROOT}/testnet/.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register
# cast call --rpc-url https://ethereum-holesky.publicnode.com 0x3F1c547b21f65e10480dE3ad8E19fAAC46C95034 "balanceOf(address)(uint256)" ${OPERATOR1_ADDR}

# Operator 2
source ~/testnet/wavs-2/.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
OPERATOR2_ADDR=`cast wallet address --private-key $AVS_PRIVATE_KEY`
echo ${OPERATOR2_ADDR}
cast send ${OPERATOR2_ADDR} --private-key ${DEPLOYER_PK} --value 0.025ether --rpc-url ${RPC_URL}
cast balance --ether ${OPERATOR2_ADDR} --rpc-url ${RPC_URL}

GIT_ROOT=$HOME/testnet/wavs-foundry-template
ENV_FILE=${GIT_ROOT}/testnet/.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# register a 3rd new wallet operator here to test with a 2/3 config, just don't run a node for it.

# Update threshold weight
# 1.8x a single operator weight (requires 2/3 of registered operators)
ECDSA_CONTRACT=`cat .nodes/avs_deploy.json | jq -r .addresses.stakeRegistry`
cast send ${ECDSA_CONTRACT} "updateStakeThreshold(uint256)" 1782625057707873 --rpc-url ${RPC_URL} --private-key ${DEPLOYER_PK}

# Verify registration for operators
(cp testnet/.env . && make operator-list)
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
