#!/bin/bash

set -e
# set -x

: '''
# Run:

sh ./build_service.sh

# Overrides:
- FILE_LOCATION: The save location of the configuration file
- TRIGGER_ADDRESS: The address to trigger the service
- SUBMIT_ADDRESS: The address to submit the service
- COMPONENT_FILENAME: The filename of the component to upload (ignored if WASM_DIGEST is used)
- WASM_DIGEST: The digest of the component to use that is already in WAVS
- TRIGGER_EVENT: The event to trigger the service (e.g. "NewTrigger(bytes)")
- FUEL_LIMIT: The fuel limit (wasm compute metering) for the service
- MAX_GAS: The maximum chain gas for the submission Tx
'''

# == Defaults ==

FUEL_LIMIT=${FUEL_LIMIT:-1000000000000}
MAX_GAS=${MAX_GAS:-5000000}
FILE_LOCATION=${FILE_LOCATION:-".docker/service.json"}
COMPONENT_FILENAME=${COMPONENT_FILENAME:-"evm_price_oracle.wasm"}
TRIGGER_EVENT=${TRIGGER_EVENT:-"NewTrigger(bytes)"}
TRIGGER_CHAIN=${TRIGGER_CHAIN:-"local"}
SUBMIT_CHAIN=${SUBMIT_CHAIN:-"local"}
AGGREGATOR_URL=${AGGREGATOR_URL:-""}
# used in make upload-component
WAVS_ENDPOINT=${WAVS_ENDPOINT:-"http://localhost:8000"}
export DOCKER_DEFAULT_PLATFORM=linux/amd64

BASE_CMD="docker run --rm --network host -w /data -v $(pwd):/data ghcr.io/lay3rlabs/wavs:0.4.0-beta.5 wavs-cli service --json true --home /data --file /data/${FILE_LOCATION}"

if [ -z "$SERVICE_MANAGER_ADDRESS" ]; then
    echo "SERVICE_MANAGER_ADDRESS is not set. Please set it to the address of the service manager."
    exit 1
fi


if [ -z "$TRIGGER_ADDRESS" ]; then
    TRIGGER_ADDRESS=`make get-trigger-from-deploy`
fi
if [ -z "$SUBMIT_ADDRESS" ]; then
    SUBMIT_ADDRESS=`make get-submit-from-deploy`
fi
if [ -z "$WASM_DIGEST" ]; then
    WASM_DIGEST=`make upload-component COMPONENT_FILENAME=$COMPONENT_FILENAME`
    WASM_DIGEST=$(echo ${WASM_DIGEST} | cut -d':' -f2)
fi

# === Core ===

TRIGGER_EVENT_HASH=`cast keccak ${TRIGGER_EVENT}`

SERVICE_ID=`$BASE_CMD init --name demo | jq -r .service.id`
echo "Service ID: ${SERVICE_ID}"

WORKFLOW_ID=`$BASE_CMD workflow add | jq -r .workflow_id`
echo "Workflow ID: ${WORKFLOW_ID}"

$BASE_CMD workflow trigger --id ${WORKFLOW_ID} set-evm --address ${TRIGGER_ADDRESS} --chain-name ${TRIGGER_CHAIN} --event-hash ${TRIGGER_EVENT_HASH} > /dev/null

# If no aggregator is set, use the default
SUB_CMD="set-evm"
if [ -n "$AGGREGATOR_URL" ]; then
    SUB_CMD="set-aggregator --url ${AGGREGATOR_URL}"
fi
$BASE_CMD workflow submit --id ${WORKFLOW_ID} ${SUB_CMD} --address ${SUBMIT_ADDRESS} --chain-name ${SUBMIT_CHAIN} --max-gas ${MAX_GAS} > /dev/null

$BASE_CMD workflow component --id ${WORKFLOW_ID} set-source-digest --digest ${WASM_DIGEST} > /dev/null

$BASE_CMD workflow component --id ${WORKFLOW_ID} permissions --http-hosts '*' --file-system true > /dev/null
$BASE_CMD workflow component --id ${WORKFLOW_ID} time-limit --seconds 30 > /dev/null
$BASE_CMD workflow component --id ${WORKFLOW_ID} env --values WAVS_ENV_SOME_SECRET > /dev/null
$BASE_CMD workflow component --id ${WORKFLOW_ID} config --values 'key=value,key2=value2' > /dev/null

$BASE_CMD manager set-evm --chain-name ${SUBMIT_CHAIN} --address `cast --to-checksum ${SERVICE_MANAGER_ADDRESS}` > /dev/null
$BASE_CMD validate > /dev/null

# inform aggregator if set
if [ -n "$AGGREGATOR_URL" ]; then
    wget -q --header="Content-Type: application/json" --post-data='{"service": '"$(cat ${FILE_LOCATION})"'}' ${AGGREGATOR_URL}/register-service -O -
fi

echo "Configuration file created at ${FILE_LOCATION}"
