#!/bin/bash
set -e
SP=""; if [[ "$(uname)" == *"Darwin"* ]]; then SP=" "; fi

mkdir -p .docker

# require a number input as argument 1, if not, require OPERATOR_INDEX env variable
export OPERATOR_INDEX=${OPERATOR_INDEX:-$1}
if [ -z "$OPERATOR_INDEX" ]; then
  echo "Please provide an operator index as the first argument or set OPERATOR_INDEX environment variable."
  exit 1
fi

ENV_FILENAME=".operator${OPERATOR_INDEX}.env"
cp .env.example.operator ${ENV_FILENAME}


OPERATOR_FILENAME=".docker/operator${OPERATOR_INDEX}.json"

cast wallet new-mnemonic --json > ${OPERATOR_FILENAME}
export OPERATOR_MNEMONIC=`jq -r .mnemonic ${OPERATOR_FILENAME}`
export OPERATOR_PK=`jq -r .accounts[0].private_key ${OPERATOR_FILENAME}`

sed -i${SP}'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$/WAVS_SUBMISSION_MNEMONIC=\"$OPERATOR_MNEMONIC\"/" ${ENV_FILENAME}
sed -i${SP}'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$/WAVS_CLI_EVM_CREDENTIAL=\"$OPERATOR_PK\"/" ${ENV_FILENAME}
