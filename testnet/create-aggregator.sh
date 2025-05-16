#!/usr/bin/bash
# set -e
SP=""; if [ "$(uname)" == *"Darwin"* ]; then SP=" "; fi

mkdir -p .docker

cp .env.example.aggregator .aggregator.env

# Create New, fund later
cast wallet new-mnemonic --json > .docker/aggregator.json
export AGG_PK=`jq -r .accounts[0].private_key .docker/aggregator.json`
sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$AGG_PK\"/" .aggregator.env

AGGREGATOR_ADDR=`cast wallet address $AGG_PK`

echo "Created aggregator: ${AGGREGATOR_ADDR}"

if [ -z "$DEPLOYER_PK" ]; then
  echo "create-aggregator: set DEPLOYER_PK and re-run this, or manually fund ${AGGREGATOR_ADDR}."
  exit 0
fi

VALUE=0.005ether
echo "Sending ${VALUE} to aggregator"
cast send ${AGGREGATOR_ADDR} --rpc-url ${RPC_URL} --private-key ${DEPLOYER_PK} --value ${VALUE}
