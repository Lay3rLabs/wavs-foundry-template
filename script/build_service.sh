#!/bin/bash

set -e

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
COMPONENT_FILENAME=${COMPONENT_FILENAME:-"eth_price_oracle.wasm"}
TRIGGER_EVENT=${TRIGGER_EVENT:-"NewTrigger(bytes)"}
TRIGGER_CHAIN=${TRIGGER_CHAIN:-"local"}
SUBMIT_CHAIN=${SUBMIT_CHAIN:-"local"}

BASE_CMD="docker run --rm --network host -w /data -v $(pwd):/data ghcr.io/lay3rlabs/wavs:0.4.0-alpha.2 wavs-cli service --json true --home /data --file /data/${FILE_LOCATION}"

if [ -z "$TRIGGER_ADDRESS" ]; then
    TRIGGER_ADDRESS=`jq -r '.deployedTo' ".docker/trigger.json"`
fi
if [ -z "$SUBMIT_ADDRESS" ]; then
    SUBMIT_ADDRESS=`jq -r '.deployedTo' ".docker/submit.json"`
fi
if [ -z "$WASM_DIGEST" ]; then
    WASM_DIGEST=`make upload-component COMPONENT_FILENAME=$COMPONENT_FILENAME`
    WASM_DIGEST=$(echo ${WASM_DIGEST} | cut -d':' -f2)
fi
if [ -z "$SERVICE_MANAGER_ADDRESS" ]; then
    echo "SERVICE_MANAGER_ADDRESS is not set. Please set it to the address of the service manager."
    exit 1
fi

# === Core ===

TRIGGER_EVENT_HASH=`cast keccak ${TRIGGER_EVENT}`

SERVICE_ID=`$BASE_CMD init --name demo | jq -r .id`
echo "Service ID: ${SERVICE_ID}"


WORKFLOW_ID=`$BASE_CMD workflow add | jq -r '.workflows | keys | .[0]'`
echo "Workflow ID: ${WORKFLOW_ID}"


$BASE_CMD trigger set-ethereum --workflow-id ${WORKFLOW_ID} --address ${TRIGGER_ADDRESS} --chain-name ${TRIGGER_CHAIN} --event-hash ${TRIGGER_EVENT_HASH} > /dev/null

$BASE_CMD submit set-ethereum --workflow-id ${WORKFLOW_ID} --address ${SUBMIT_ADDRESS} --chain-name ${SUBMIT_CHAIN} --max-gas ${MAX_GAS} > /dev/null

COMPONENT_ID=`$BASE_CMD component add --id ${WORKFLOW_ID} --digest ${WASM_DIGEST} | jq -r '.workflows | keys | .[0]'`
echo "Component ID: ${COMPONENT_ID}"


$BASE_CMD component permissions --id ${COMPONENT_ID} --http-hosts '*' --file-system true > /dev/null

# TODO: https://github.com/Lay3rLabs/WAVS/issues/516
echo $SERVICE_MANAGER_ADDRESS
jq --argjson manager '{"ethereum":{"chain_name":"'${SUBMIT_CHAIN}'","address":"'${SERVICE_MANAGER_ADDRESS}'"}}' '.manager = $manager' ${FILE_LOCATION} > ${FILE_LOCATION}.tmp && mv ${FILE_LOCATION}.tmp ${FILE_LOCATION}

# TODO: fuel limit / max exec seconds set

$BASE_CMD validate > /dev/null

echo "Configuration file created at ${FILE_LOCATION}"
