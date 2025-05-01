#!/bin/bash

set -e

mkdir -p ../.docker

SP=""
if [[ "$(uname)" == *"Darwin"* ]]; then
  SP=" "
fi

# Create operator 1 keys
cast wallet new-mnemonic --json > ../.docker/operator1.json
export OPERATOR1_MNEMONIC=`cat ../.docker/operator1.json | jq -r .mnemonic`
export OPERATOR1_PK=`cat ../.docker/operator1.json | jq -r .accounts[0].private_key`

sed -i${SP}'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$/WAVS_SUBMISSION_MNEMONIC=\"$$OPERATOR1_MNEMONIC\"/" .env1
sed -i${SP}'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$/WAVS_CLI_EVM_CREDENTIAL=\"$$OPERATOR1_PK\"/" .env1
sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$$OPERATOR1_PK\"/" .env1

# Create operator 2 keys
cast wallet new-mnemonic --json > ../.docker/operator2.json
export OPERATOR2_MNEMONIC=`cat ../.docker/operator2.json | jq -r .mnemonic`
export OPERATOR2_PK=`cat ../.docker/operator2.json | jq -r .accounts[0].private_key`

sed -i${SP}'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$/WAVS_SUBMISSION_MNEMONIC=\"$$OPERATOR2_MNEMONIC\"/" .env2
sed -i${SP}'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$/WAVS_CLI_EVM_CREDENTIAL=\"$$OPERATOR2_PK\"/" .env2
