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
'''

# == Defaults ==

FILE_LOCATION=.docker/service.json
BASE_CMD="docker run --rm --network host -w /data -v $(pwd):/data ghcr.io/lay3rlabs/wavs:latest wavs-cli service --json true --home /data --file /data/${FILE_LOCATION}"

if [ -z "$TRIGGER_ADDRESS" ]; then
    TRIGGER_ADDRESS=`jq -r '.trigger' ".docker/script_deploy.json"`
fi
if [ -z "$SUBMIT_ADDRESS" ]; then
    SUBMIT_ADDRESS=`jq -r '.service_handler' ".docker/script_deploy.json"`
fi

if [ -z "$COMPONENT_FILENAME" ]; then
    COMPONENT_FILENAME="eth_price_oracle.wasm"
fi

if [ -z "$WASM_DIGEST" ]; then
    WASM_DIGEST=`make upload-component COMPONENT_FILENAME=$COMPONENT_FILENAME`
    WASM_DIGEST=$(echo ${WASM_DIGEST} | cut -d':' -f2)
fi

if [ -z "$TRIGGER_EVENT" ]; then
    TRIGGER_EVENT="NewTrigger(bytes)"
fi

# === Core ===

TRIGGER_EVENT_HASH=`cast keccak ${TRIGGER_EVENT}`

SERVICE_ID=`$BASE_CMD init --name demo | jq -r .id`
echo "Service ID: ${SERVICE_ID}"

COMPONENT_ID=`$BASE_CMD component add --digest ${WASM_DIGEST} | jq -r '.components | keys | .[0]'`
echo "Component ID: ${COMPONENT_ID}"

$BASE_CMD component permissions --id ${COMPONENT_ID} --http-hosts '*' --file-system true > /dev/null

WORKFLOW_ID=`$BASE_CMD workflow add --component-id ${COMPONENT_ID} | jq -r '.workflows | keys | .[0]'`
echo "Workflow ID: ${WORKFLOW_ID}"

$BASE_CMD trigger set-ethereum --workflow-id ${WORKFLOW_ID} --address ${TRIGGER_ADDRESS} --chain-name local --event-hash ${TRIGGER_EVENT_HASH} > /dev/null

$BASE_CMD submit set-ethereum --workflow-id ${WORKFLOW_ID} --address ${SUBMIT_ADDRESS} --chain-name local --max-gas 5000000 > /dev/null

$BASE_CMD validate > /dev/null

echo "Configuration file created at ${FILE_LOCATION}"
