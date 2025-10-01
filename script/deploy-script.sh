#!/bin/bash
set -e
# set -x

STATUS_FILE=".docker/component-upload-status"

# Store the PID of the background process
if [[ "${SKIP_COMPONENT_UPLOAD}" != "true" ]]; then
    bash script/upload-components-background.sh &
    UPLOAD_PID=$!
fi

# Function to clean up on exit
cleanup() {
    echo "Cleaning up..."
    # Kill the background upload process if it's still running
    if [ -n "$UPLOAD_PID" ] && kill -0 $UPLOAD_PID 2>/dev/null; then
        echo "Terminating background upload process (PID: $UPLOAD_PID)..."
        kill -TERM $UPLOAD_PID 2>/dev/null
        # Give it a moment to terminate gracefully, then force kill if needed
        sleep 1
        kill -9 $UPLOAD_PID 2>/dev/null || true
    fi
    # Clean up the status file
    rm -f "$STATUS_FILE"
    echo "Cleanup complete"
    exit 1
}

# Set up trap to handle Ctrl+C (SIGINT) and other termination signals
trap cleanup INT TERM EXIT

# if RPC_URL is not set, use default by calling command
if [ -z "$RPC_URL" ]; then
    export RPC_URL=$(task get-rpc)
fi
if [ -z "$AGGREGATOR_URL" ]; then
    export AGGREGATOR_URL=http://127.0.0.1:8001
fi

# local: create deployer & auto fund. testnet: create & iterate check balance
bash ./script/create-deployer.sh
export FUNDED_KEY=$(task config:funded-key)

if [[ "${SKIP_CONTRACT_UPLOAD}" != "true" ]]; then
    echo "🟢 Deploying POA Service Manager..."
    POA_MIDDLEWARE="docker run --rm --network host -v ./.nodes:/root/.nodes --env-file .env ghcr.io/lay3rlabs/poa-middleware:1.0.1"
    $POA_MIDDLEWARE deploy
    sleep 1 # for Base
    $POA_MIDDLEWARE owner_operation updateStakeThreshold 1000
    sleep 1 # for Base
    $POA_MIDDLEWARE owner_operation updateQuorum 2 3
fi

if [ "$(task get-deploy-status)" = "LOCAL" ]; then
    # required for the checkpoint stuff, ref: aurtur / https://github.com/Lay3rLabs/EN0VA/pull/31/commits/d205e9c65f91fb5b0b5bca672d8d28d6c7f672f9#diff-e3d8246ec3421fa3a204fe7a8f0586acfad4888ae82f5b8c6d130cb907705c80R75-R78
    cast rpc anvil_mine --rpc-url $(task get-rpc)
fi

WAVS_SERVICE_MANAGER_ADDRESS=`task config:service-manager-address`
echo "ℹ️ Using WAVS Service Manager address: ${WAVS_SERVICE_MANAGER_ADDRESS}"

### === Deploy Contract === ###

# TODO: remove deploy-contracts.sh once this works
export DEPLOY_ENV=`task get-deploy-status`
if [[ "${SKIP_CONTRACT_UPLOAD}" != "true" ]]; then
    echo "🚀 Starting contract deployment..."

    # Build the contracts
    echo "Building contracts..."
    forge build

    echo "🔧 Configuration:"
    echo "   RPC_URL: ${RPC_URL}"
    echo "   WAVS_SERVICE_MANAGER_ADDRESS: ${WAVS_SERVICE_MANAGER_ADDRESS}"

    mkdir -p .docker
    echo "📦 Deploying contracts..."

    # TODO: we can probably put this in the components.json to simple and make it scale right?
    forge create SimpleSubmit --json --broadcast \
        --rpc-url $RPC_URL \
        --private-key $FUNDED_KEY \
        --constructor-args "${WAVS_SERVICE_MANAGER_ADDRESS}"  > .docker/submit.json

    forge create SimpleTrigger --json --broadcast \
        --rpc-url $RPC_URL \
        --private-key $FUNDED_KEY \
        --constructor-args "${WAVS_SERVICE_MANAGER_ADDRESS}"  > .docker/trigger.json

    # Create simplified deployment summary
    jq -n \
      --arg service_id "" \
      --arg rpc_url "${RPC_URL}" \
      --arg wavs_service_manager "${WAVS_SERVICE_MANAGER_ADDRESS}" \
      --slurpfile submit .docker/submit.json \
      --slurpfile trigger .docker/trigger.json \
      '{
        service_id: $service_id,
        rpc_url: $rpc_url,
        wavs_service_manager: $wavs_service_manager,
        evmpriceoracle_submit: $submit[0],
        evmpriceoracle_trigger: $trigger[0],
      }' \
      > .docker/deployment_summary.json

    sleep 1
fi

### === Deploy Services ===

# Require component configuration file
COMPONENT_CONFIGS_FILE="config/components.json"

if [ ! -f "$COMPONENT_CONFIGS_FILE" ]; then
    echo "❌ Component configuration file not found: $COMPONENT_CONFIGS_FILE"
    echo "Please run 'script/configure-components.sh init' to create the configuration."
    exit 1
fi

echo "Using component configuration from: $COMPONENT_CONFIGS_FILE"

# Testnet: set values (default: local if not set)
if [ "$(task get-deploy-status)" = "TESTNET" ]; then
    export TRIGGER_CHAIN=evm:$(task get-chain-id)
    export SUBMIT_CHAIN=evm:$(task get-chain-id)
fi

# Determine chain name based on deployment environment
if [ "$(task get-deploy-status)" = "TESTNET" ]; then
    export CHAIN_NAME=evm:$(task get-chain-id)
    export AGGREGATOR_TIMER_DELAYER_SECS=3 # base wait ~1 block
else
    export CHAIN_NAME=evm:31337 # local
    export AGGREGATOR_TIMER_DELAYER_SECS=0
fi
echo "✅ Chain Name: ${CHAIN_NAME}"

echo "📋 All configuration variables exported for component-specific substitution"

# wait for STATUS_FILE to contain the status COMPLETED in its content, check every 0.5 seconds for up to 60 seconds then error
if [[ "${SKIP_COMPONENT_UPLOAD}" != "true" ]]; then
    echo "Waiting for component uploads to complete..."
    timeout 300 bash -c "
        trap 'exit 130' INT TERM
        while ! grep -q 'COMPLETED' '$STATUS_FILE' 2>/dev/null; do
            sleep 0.5
        done
    "
    if [ $? -ne 0 ]; then
        echo "❌ Component uploads did not complete in time or failed."
        exit 1
    fi
    echo "✅ All components uploaded successfully"
    # clear tmp file
    rm -f $STATUS_FILE

    seconds=2
    echo "Waiting for ${seconds} seconds for registry to update..."
    sleep ${seconds}
fi

# Create service with multiple workflows
echo "Creating service with multiple component workflows..."
export COMPONENT_CONFIGS_FILE="$COMPONENT_CONFIGS_FILE"
# All required variables are now exported for component-specific substitution
REGISTRY=$(task get-registry) source ./script/build-service.sh
sleep 1

# === Upload service.json to IPFS ===
# local: 127.0.0.1:5001 | testnet: https://app.pinata.cloud/. set PINATA_API_KEY to JWT token in .env
echo "Uploading to IPFS..."

export PINATA_API_KEY=$(grep ^WAVS_ENV_PINATA_API_KEY= .env | cut -d '=' -f2-)
# if not LOCAL, ensure PINATA_API_KEY is set or PINATA_API_KEY. If neither, require input
if [ "$(task get-deploy-status)" != "LOCAL" ]; then
    if [ -z "$PINATA_API_KEY" ]; then
        read -p "Enter your Pinata JWT API Key (or set WAVS_ENV_PINATA_API_KEY in .env): " PINATA_API_KEY
        if [ -z "$PINATA_API_KEY" ]; then
            echo "❌ Pinata API Key is required for TESTNET deployments."
            exit 1
        fi
        export PINATA_API_KEY
    fi

    read -p "Make any changes you want to the service.json now. Press [Enter] to continue upload to IPFS..."
fi

export ipfs_cid=`SERVICE_FILE=.docker/service.json make upload-to-ipfs`
# LOCAL: http://127.0.0.1:8080 | TESTNET: https://gateway.pinata.cloud/
export IPFS_GATEWAY="$(task get-ipfs-gateway)"
export IPFS_URI="ipfs://${ipfs_cid}"
IPFS_URL="${IPFS_GATEWAY}${ipfs_cid}"
echo "IPFS_URL=${IPFS_URL}"

echo "Querying to verify IPFS upload... (120 second timeout)"
curl ${IPFS_URL} --connect-timeout 120 --max-time 120 --show-error --fail
while [ $? -ne 0 ]; do
    echo "IPFS upload not yet available. Please ensure the CID is correct and try again."
    read -p "Enter the IPFS URI (e.g., ipfs://bafkreicglpmavzsomzghbmemauv4i4jkxgaxsqefruxtplulul7o2sg33e): " IPFS_URI
    ipfs_cid=$(echo $IPFS_URI | sed 's|ipfs://||')
    IPFS_URL="${IPFS_GATEWAY}${ipfs_cid}"
    curl ${IPFS_URL} --connect-timeout 120 --max-time 120 --show-error --fail
done


if [ "$FUNDED_KEY" ]; then
    echo ""
    echo "Setting service URI on WAVS Service Manager..."
    # if ` Error: Failed to estimate gas: server returned an error response: error code 3: execution reverted, data: "0x"`, then ServiceManager upload failed. retry
    cast send ${WAVS_SERVICE_MANAGER_ADDRESS} 'setServiceURI(string)' "${IPFS_URI}" -r ${RPC_URL} --private-key ${FUNDED_KEY}
fi

echo "IPFS_GATEWAY=${IPFS_GATEWAY}"
echo "IPFS_URI=${IPFS_URI}"

sleep 1

### === Create Aggregator ===

bash ./script/create-aggregator.sh 1
IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/aggregator-1/start.sh
sleep 3
curl -s -X POST -H "Content-Type: application/json" -d "{
  \"service_manager\": {
    \"evm\": {
      \"chain\": \"${CHAIN_NAME}\",
      \"address\": \"${WAVS_SERVICE_MANAGER_ADDRESS}\"
    }
  }
}" ${AGGREGATOR_URL}/services

### === Start WAVS ===
bash ./script/create-operator.sh 1
IPFS_GATEWAY=${IPFS_GATEWAY} bash ./infra/wavs-1/start.sh
sleep 3

# Deploy the service JSON to WAVS so it now watches and submits.
# 'opt in' for WAVS to watch (this is before we register to Eigenlayer)
WAVS_ENDPOINT=http://127.0.0.1:8000 SERVICE_URL=${IPFS_URI} IPFS_GATEWAY=${IPFS_GATEWAY} make deploy-service
sleep 3

export SERVICE_ID=${SERVICE_ID:-`task config:service-id`}
if [ -z "$SERVICE_ID" ]; then
    echo "❌ Failed to retrieve service ID"
    exit 1
fi
echo "✅ Service ID: ${SERVICE_ID}"

# Update the deployment summary with the service ID
jq ".service_id = \"${SERVICE_ID}\"" .docker/deployment_summary.json > .docker/deployment_summary.json.tmp
mv .docker/deployment_summary.json.tmp .docker/deployment_summary.json

### === Register service specific operator ===

# OPERATOR_PRIVATE_KEY, AVS_SIGNING_ADDRESS
eval "$(task setup-avs-signing HD_INDEX=1 | tail -4)"

# Reset registry after deployment is complete
echo "Cleaning up registry data..."
REGISTRY=$(task get-registry)
if [ -n "$REGISTRY" ]; then
    PROTOCOL="https"
    if [[ "$REGISTRY" == *"localhost"* ]] || [[ "$REGISTRY" == *"127.0.0.1"* ]]; then
        PROTOCOL="http"
    fi
    warg reset --registry ${PROTOCOL}://${REGISTRY} || echo "Registry reset failed (non-critical)"
fi

# Remove trap for normal exit
trap - INT TERM EXIT

echo "✅ Deployment complete!"

# if post-deploy.sh exists, run it
if [ -f "script/post-deploy.sh" ]; then
    echo "Running post-deploy.sh..."
    bash script/post-deploy.sh

    echo "✅ post-deploy.sh completed!"
fi
