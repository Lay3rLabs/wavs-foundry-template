#!/bin/bash
set -e

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


if [ -d "${OPERATOR_LOC}" ] && [ "$(ls -A ${OPERATOR_LOC})" ]; then
  read -p "Directory ${OPERATOR_LOC} already exists and is not empty. Do you want to remove it? (y/n): " -n 1 -r
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "\nRemoving ${OPERATOR_LOC}"
    docker kill wavs-${OPERATOR_INDEX} || true
    # ca/ & app/ directory requires this
    sudo rm -rf ${OPERATOR_LOC}/ca
    sudo rm -rf ${OPERATOR_LOC}/app
    rm -rf ${OPERATOR_LOC}
  else
    echo -e "\nExiting without changes."
    exit 1
  fi
fi

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


# Create startup script
cat > "${OPERATOR_LOC}/start.sh" << EOF
#!/bin/bash
cd \$(dirname "\$0") || exit 1

IMAGE=ghcr.io/lay3rlabs/wavs:248e294
WAVS_INSTANCE=wavs-${OPERATOR_INDEX}

docker kill \${WAVS_INSTANCE} || true
docker rm \${WAVS_INSTANCE} || true

docker run -d --rm --name \${WAVS_INSTANCE} --network host --env-file .env -v \$(pwd):/root/wavs \${IMAGE} wavs --home /root/wavs --host 0.0.0.0 --log-level info
sleep 0.5
EOF

cp wavs.toml ${OPERATOR_LOC}/wavs.toml

# TODO: if testnet change active_trigger_chains from local -> holesky

echo "Operator ${OPERATOR_INDEX} created at ${OPERATOR_LOC}"
