#!/bin/bash

export DEFAULT_ENV_FILE=${DEFAULT_ENV_FILE:-"infra/wavs-1/.env"}

export SERVICE_ID=`curl -s http://localhost:8000/app | jq -r '.services[0].id'`
export HD_INDEX=`curl -s http://localhost:8000/service-key/${SERVICE_ID} | jq -rc '.secp256k1.hd_index'`

source ${DEFAULT_ENV_FILE}
export OPERATOR_PRIVATE_KEY=`cast wallet private-key --mnemonic "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index 0`
export AVS_SIGNING_ADDRESS=`cast wallet address --mnemonic-path "$WAVS_SUBMISSION_MNEMONIC" --mnemonic-index ${HD_INDEX}`

echo "HD_INDEX=${HD_INDEX}"
echo "SERVICE_ID=${SERVICE_ID}"
echo "OPERATOR_PRIVATE_KEY=*HIDDEN*"
echo "AVS_SIGNING_ADDRESS=${AVS_SIGNING_ADDRESS}"
