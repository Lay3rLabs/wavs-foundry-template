# WAVS Multi-Operator Testnet Example

This example demonstrates how to run a WAVS service with multiple operators. It builds upon the main example in the root directory but extends it to use two separate operators.

## Prerequisites

Make sure you have completed the system requirements setup from the main [README.md](../README.md).

## Setup and Run

### Create & Start

```bash
cd testnet/ || true

sh ./create-aggregator.sh

# TODO: change to allow for TESTNET vs LOCAL gen (require another argument)
sh ./create-operator.sh 1
sh ./create-operator.sh 2

# - Shows operators being used
# - Deploys Eigen AVS specific contracts
# - Funds Aggregator wallet (from .aggregator.env) # TODO: real testnet this needs to be pre-funded with FUNDED_KEY instead
# - Start WAVS services using docker-compose
sh start.sh
```

## Upload standard smart contracts

```bash
# new terminal
cd $(git rev-parse --show-toplevel)

# Wait for deployment to complete (check for start.log)
while [ ! -f .docker/start.log ]; do echo "waiting for start.log" && sleep 1; done

# This Deployer can be any private key, using
# pre funded account for simplicity.
export DEPLOYER_PK=$(cat ./.nodes/deployer)
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)

forge create SimpleSubmit --json --broadcast -r https://1rpc.io/holesky --private-key "${DEPLOYER_PK}" --constructor-args "${SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r .deployedTo .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r https://1rpc.io/holesky --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r .deployedTo .docker/trigger.json`
```

## Setup WAVS instances

```bash
cd $(git rev-parse --show-toplevel)

# Start wavs
# docker compose -f testnet/docker-compose-multi.yml logs -f &
(cd /root/wavs-1 && sh start.sh)
(cd /root/wavs-2 && sh start.sh)

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

WAVS_ENDPOINT="http://5.161.229.43:8000" CREDENTIAL=${DEPLOYER_PK} make deploy-service
WAVS_ENDPOINT="http://5.161.229.43:9000" CREDENTIAL=${DEPLOYER_PK} make deploy-service
```

## Register operators -> Eigenlayer

```bash
cp /root/wavs-foundry-template/testnet/.operator1.env /root/wavs-1/.env
source /root/wavs-1/.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`

# fund operator acc so they can register
OPERATOR1_ADDR=`cast wallet address --private-key $AVS_PRIVATE_KEY`

# https://holesky-faucet.pk910.de/
cast send ${OPERATOR1_ADDR} --private-key ${DEPLOYER_PK} --value 0.1ether --rpc-url https://ethereum-holesky-rpc.publicnode.com
cast balance --ether ${OPERATOR1_ADDR} --rpc-url https://ethereum-holesky-rpc.publicnode.com
echo ${OPERATOR1_ADDR}


# stake some ETH -> stETH. This is done in the operator-register but required some tweaking
export MINT_FUNCTION="submit(address _referral)"
cast send "0x3f1c547b21f65e10480de3ad8e19faac46c95034" "$MINT_FUNCTION" "$OPERATOR1_ADDR" "0x0000000000000000000000000000000000000000" --private-key "$AVS_PRIVATE_KEY" --value 0.01ether --rpc-url "https://ethereum-holesky-rpc.publicnode.com"


# cast balance --ether 0x95Bc97c9c6918852a2f74Be8649CEA5467Be0D60 --rpc-url https://ethereum-holesky-rpc.publicnode.com
cast send 0x3f1c547b21f65e10480de3ad8e19faac46c95034 "submit(address _referral)" ${OPERATOR1_ADDR} --private-key 0x883eced02d8ce72c4350b78de82b53559f8bb6d89840b20e4dfe56a62cb8b3de --value 10000000000000000 --rpc-url https://ethereum-holesky-rpc.publicnode.com

# cast call --rpc-url https://ethereum-holesky.publicnode.com 0x3f1c547b21f65e10480de3ad8e19faac46c95034 "balanceOf(address)(uint256)" ${OPERATOR1_ADDR}

# !IMPORTANT:
# - Testnet by may 15th
# - testnet needs: we (layer), deploy an AVS contract, run agg, run 3 diff operators (keep all online)
# - next week: strong ask> noah takes what he has NFT demo Monday. Test Tuesday, Wed internal testing + UI. Thurs announcement
# - Repo / tag with a contract. Ethan here to help, give him my SSH key. Deploy UI somewhere
# - req: 4 boxes from Ethan, SSH key.

# ENV_FILE=testnet/.operator1.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register
ENV_FILE=/root/wavs-1/.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# Operator 2
source testnet/.operator2.env
AVS_PRIVATE_KEY=`cast wallet private-key --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 1`
ENV_FILE=testnet/.operator2.env AVS_PRIVATE_KEY=${AVS_PRIVATE_KEY} make operator-register

# register a 3rd new wallet operator here to test with a 2/3 config, just don't run a node for it.

# Update threshold weight
# 1.8x a single operator weight (requires 2/3 of registered operators)
ECDSA_CONTRACT=`cat .nodes/avs_deploy.json | jq -r .addresses.stakeRegistry`
cast send ${ECDSA_CONTRACT} "updateStakeThreshold(uint256)" 1782625057707873 --rpc-url https://ethereum-holesky-rpc.publicnode.com --private-key ${DEPLOYER_PK}

# Verify registration for operators
make operator-list
```

## Contract call and aggregation

```bash
# Trigger the service (request CMC ID price)
export COIN_MARKET_CAP_ID=2
# TODO: change from anvil -> just PRIVATE_KEY
export ANVIL_PRIVATE_KEY=${DEPLOYER_PK}
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig 'run(string,string)' --rpc-url https://ethereum-holesky-rpc.publicnode.com --broadcast

export RPC_URL=https://ethereum-holesky-rpc.publicnode.com
TRIGGER_ID=`make get-trigger | grep "TriggerID:" | awk '{print $2}'`
TRIGGER_ID=${TRIGGER_ID} make show-result
```
