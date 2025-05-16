#!/bin/bash
set -e
set -x

SP=""; if [ "$(uname)" == *"Darwin"* ]; then SP=" "; fi

cd $(git rev-parse --show-toplevel) || exit 1

mkdir -p .docker

# require a number input as argument 1, if not, require OPERATOR_INDEX env variable
export OPERATOR_INDEX=${OPERATOR_INDEX:-$1}
if [ -z "$OPERATOR_INDEX" ]; then
  echo "Please provide an operator index as the first argument or set OPERATOR_INDEX environment variable."
  exit 1
fi

OPERATOR_LOC=infra/wavs-${OPERATOR_INDEX}
mkdir -p ${OPERATOR_LOC}


ENV_FILENAME="${OPERATOR_LOC}/.env"
cp ./script/template/.env.example.operator ${ENV_FILENAME}


TEMP_FILENAME=".docker/tmp.json"

cast wallet new-mnemonic --json > ${TEMP_FILENAME}
export OPERATOR_MNEMONIC=`jq -r .mnemonic ${TEMP_FILENAME}`
export OPERATOR_PK=`jq -r .accounts[0].private_key ${TEMP_FILENAME}`

sed -i${SP}'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$/WAVS_SUBMISSION_MNEMONIC=\"$OPERATOR_MNEMONIC\"/" ${ENV_FILENAME}
sed -i${SP}'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$/WAVS_CLI_EVM_CREDENTIAL=\"$OPERATOR_PK\"/" ${ENV_FILENAME}

rm ${TEMP_FILENAME}

echo "Operator ${OPERATOR_INDEX} created at ${OPERATOR_LOC}"
