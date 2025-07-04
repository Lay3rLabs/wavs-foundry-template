#!/bin/bash

if [ -z "$WAVS_SERVICE_MANAGER_ADDRESS" ]; then
    if [ -f .nodes/avs_deploy.json ]; then
        echo "Using WAVS_SERVICE_MANAGER_ADDRESS from .nodes/avs_deploy.json"
        export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.WavsServiceManager' .nodes/avs_deploy.json)
    else
        echo "WAVS_SERVICE_MANAGER_ADDRESS is not set."
        return
    fi
fi

forge build
if [ $? -ne 0 ]; then
    echo "Forge build failed. Running 'npm install' and deleting the 'out/' and 'cache/' directory."
    npm install
    rm -rf out/ cache/
    forge build
fi

export DEPLOYER_PK=$(cat .nodes/deployer)

forge create SimpleSubmit --json --broadcast -r ${RPC_URL} --private-key "${DEPLOYER_PK}" --constructor-args "${WAVS_SERVICE_MANAGER_ADDRESS}" > .docker/submit.json
export SERVICE_SUBMISSION_ADDR=`jq -r '.deployedTo' .docker/submit.json`

forge create SimpleTrigger --json --broadcast -r ${RPC_URL} --private-key "${DEPLOYER_PK}" > .docker/trigger.json
export SERVICE_TRIGGER_ADDR=`jq -r '.deployedTo' .docker/trigger.json`

echo "RPC_URL=${RPC_URL}"
echo "WAVS_SERVICE_MANAGER_ADDRESS=${WAVS_SERVICE_MANAGER_ADDRESS}"
echo "SERVICE_SUBMISSION_ADDR=${SERVICE_SUBMISSION_ADDR}"
echo "SERVICE_TRIGGER_ADDR=${SERVICE_TRIGGER_ADDR}"

