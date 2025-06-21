#!/bin/bash
# set -e

if git status --porcelain | grep -q "^.* components/"; then
    echo "Found pending changes in components/*, building"
    WASI_BUILD_DIR=components/evm-price-oracle make wasi-build
fi


### === Deploy Eigenlayer ===

# local: create deployer & auto fund. testnet: create & iterate check balance
bash ./script/create-deployer.sh
export DEPLOYER_PK=$(cat .nodes/deployer)
sleep 1

## Deploy Eigenlayer from Deployer
COMMAND=deploy make wavs-middleware
sleep 1

### === Deploy Service === ###

# Forge deploy SimpleSubmit & SimpleTrigger
source script/deploy-contracts.sh
sleep 1

### === Deploy Service ===
export COMPONENT_FILENAME=evm_price_oracle.wasm
export PKG_NAME="evmrustoracle"
export PKG_VERSION="0.1.0"
# ** Testnet Setup: https://wa.dev/account/credentials/new -> warg login
source script/upload-to-wasi-registry.sh || true
sleep 1

# Testnet: set values (default: local if not set)
# export TRIGGER_CHAIN=holesky
# export SUBMIT_CHAIN=holesky

# Package not found with wa.dev? -- make sure it is public
REGISTRY=${REGISTRY} source ./script/build-service.sh
sleep 0.5

# === Upload service.json to IPFS ===
# local: 127.0.0.1:5001 | testnet: https://app.pinata.cloud/. set PINATA_API_KEY to JWT token in .env
echo "Uploading to IPFS..."
export ipfs_cid=`SERVICE_FILE=.docker/service.json make upload-to-ipfs`
# LOCAL: http://127.0.0.1:8080 | TESTNET: https://gateway.pinata.cloud/
export IPFS_GATEWAY="$(bash script/get-ipfs-gateway.sh)"
export IPFS_URI="ipfs://${ipfs_cid}"
IPFS_URL="${IPFS_GATEWAY}${ipfs_cid}"
echo "IPFS_URL=${IPFS_URL}"
curl ${IPFS_URL}

if [ "$DEPLOYER_PK" ]; then
    echo ""
    echo "Setting service URI on WAVS Service Manager..."
    cast send ${WAVS_SERVICE_MANAGER_ADDRESS} 'setServiceURI(string)' "${IPFS_URI}" -r ${RPC_URL} --private-key ${DEPLOYER_PK}
fi

echo "IPFS_GATEWAY=${IPFS_GATEWAY}"
echo "IPFS_URI=${IPFS_URI}"

sleep 1

### === Create Aggregator ===

bash ./script/create-aggregator.sh 1
IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/aggregator-1/start.sh
sleep 1
wget -q --header="Content-Type: application/json" --post-data="{\"uri\": \"${IPFS_URI}\"}" ${AGGREGATOR_URL}/register-service -O -

### === Start WAVS ===
bash ./script/create-operator.sh 1
IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/wavs-1/start.sh

echo "Waiting for http://127.0.0.1:8080/app to be available..."
timeout=30
elapsed=0
while ! curl -s http://127.0.0.1:8080/app > /dev/null; do
    if [ $elapsed -ge $timeout ]; then
        echo "❌ Timeout: Server not available after $timeout seconds"
        return
    fi
    echo "Server not ready, waiting 2 seconds... ($elapsed/$timeout)"
    sleep 2
    elapsed=$((elapsed + 2))
done

# Deploy the service JSON to WAVS so it now watches and submits.
# 'opt in' for WAVS to watch (this is before we register to Eigenlayer)
WAVS_ENDPOINT=http://127.0.0.1:8000 SERVICE_URL=${IPFS_URI} IPFS_GATEWAY=${IPFS_GATEWAY} make deploy-service

### === Register service specific operator ===
SERVICE_INDEX=0 source ./script/avs-signing-key.sh

# TESTNET: set WAVS_SERVICE_MANAGER_ADDRESS
export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)
COMMAND="register ${OPERATOR_PRIVATE_KEY} ${AVS_SIGNING_ADDRESS} 0.001ether" make wavs-middleware

# Verify registration
COMMAND="list_operator" PAST_BLOCKS=500 make wavs-middleware
