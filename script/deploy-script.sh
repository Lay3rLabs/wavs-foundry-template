#!/bin/bash
# set -e

if [ ! -d compiled/ ] || [ -z "$(find compiled/ -name '*.wasm')" ]; then
    echo "No WASM files found in compiled/. Building components."
    make wasi-build
fi

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
if [ "$(sh ./script/get-deploy-status.sh)" = "TESTNET" ]; then
    read -p "Enter the component filename (default: ${COMPONENT_FILENAME}): " input_filename
    if [ -n "$input_filename" ]; then
        export COMPONENT_FILENAME="$input_filename"
    fi
fi

export PKG_NAME="evmrustoracle"
if [ "$(sh ./script/get-deploy-status.sh)" = "TESTNET" ]; then
    read -p "Enter the package name (default: ${PKG_NAME}): " input_pkg_name
    if [ -n "$input_pkg_name" ]; then
        export PKG_NAME="$input_pkg_name"
    fi
fi

export PKG_VERSION="0.1.0"
if [ "$(sh ./script/get-deploy-status.sh)" = "TESTNET" ]; then
    read -p "Enter the package version (default: ${PKG_VERSION}): " input_pkg_version
    if [ -n "$input_pkg_version" ]; then
        export PKG_VERSION="$input_pkg_version"
    fi
fi

# ** Testnet Setup: https://wa.dev/account/credentials/new -> warg login
source script/upload-to-wasi-registry.sh || true
sleep 1

# Testnet: set values (default: local if not set)
if [ "$(sh ./script/get-deploy-status.sh)" = "TESTNET" ]; then
    export TRIGGER_CHAIN=holesky
    export SUBMIT_CHAIN=holesky
fi

# Package not found with wa.dev? -- make sure it is public
REGISTRY=${REGISTRY} source ./script/build-service.sh
sleep 1

# === Upload service.json to IPFS ===
# local: 127.0.0.1:5001 | testnet: https://app.pinata.cloud/. set PINATA_API_KEY to JWT token in .env
echo "Uploading to IPFS..."
export ipfs_cid=`SERVICE_FILE=.docker/service.json make upload-to-ipfs`
# LOCAL: http://127.0.0.1:8080 | TESTNET: https://gateway.pinata.cloud/
export IPFS_GATEWAY="$(bash script/get-ipfs-gateway.sh)"
export IPFS_URI="ipfs://${ipfs_cid}"
IPFS_URL="${IPFS_GATEWAY}${ipfs_cid}"
echo "IPFS_URL=${IPFS_URL}"

echo "Querying to verify IPFS upload... (120 second timeout)"
curl ${IPFS_URL} --connect-timeout 120 --max-time 120 --show-error --fail

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
sleep 5

# Deploy the service JSON to WAVS so it now watches and submits.
# 'opt in' for WAVS to watch (this is before we register to Eigenlayer)
WAVS_ENDPOINT=http://127.0.0.1:8000 SERVICE_URL=${IPFS_URI} IPFS_GATEWAY=${IPFS_GATEWAY} make deploy-service

### === Register service specific operator ===

# OPERATOR_PRIVATE_KEY, AVS_SIGNING_ADDRESS
SERVICE_INDEX=0 source ./script/avs-signing-key.sh

# TODO: move this check into the middleware (?)
if [ "$(sh ./script/get-deploy-status.sh)" = "TESTNET" ]; then
    export OPERATOR_ADDRESS=$(cast wallet address --private-key ${OPERATOR_PRIVATE_KEY})
    while true; do
        BALANCE=$(cast balance ${OPERATOR_ADDRESS} --rpc-url ${RPC_URL} --ether)
        if [ "$BALANCE" != "0" ]; then
            echo "OPERATOR_ADDRESS has balance: $BALANCE"
            break
        else
            echo "Waiting for ${OPERATOR_ADDRESS} (operator) to have a balance..."
            sleep 5
        fi
    done
fi

export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager ./.nodes/avs_deploy.json)
COMMAND="register ${OPERATOR_PRIVATE_KEY} ${AVS_SIGNING_ADDRESS} 0.001ether" make wavs-middleware

# Verify registration
COMMAND="list_operators" PAST_BLOCKS=500 make wavs-middleware

echo "✅ Deployment complete!"
