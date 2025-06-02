#!/bin/bash
# set -e

SP=""; if [[ "$(uname)" == *"Darwin"* ]]; then SP=" "; fi

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
    docker kill wavs-${OPERATOR_INDEX} > /dev/null 2>&1 || true

    echo "Removing dir ${OPERATOR_LOC} ((may prompt for password))"
    sudo rm -rf ${OPERATOR_LOC}
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

IMAGE=ghcr.io/lay3rlabs/wavs:0.4.0-rc
WAVS_INSTANCE=wavs-${OPERATOR_INDEX}
IPFS_GATEWAY=\${IPFS_GATEWAY:-"https://gateway.pinata.cloud/ipfs/"}

docker kill \${WAVS_INSTANCE} > /dev/null 2>&1 || true
docker rm \${WAVS_INSTANCE} > /dev/null 2>&1 || true

docker run -d --rm --name \${WAVS_INSTANCE} --network host --env-file .env -v \$(pwd):/root/wavs \${IMAGE} wavs --home /root/wavs --ipfs-gateway \${IPFS_GATEWAY} --host 0.0.0.0 --log-level info
sleep 0.25

if [ ! "\$(docker ps -q -f name=\${WAVS_INSTANCE})" ]; then
  echo "Container \${WAVS_INSTANCE} is not running. Reason:"
  docker run --rm --name \${WAVS_INSTANCE} --network host --env-file .env -v \$(pwd):/root/wavs \${IMAGE} wavs --home /root/wavs --ipfs-gateway \${IPFS_GATEWAY} --host 0.0.0.0 --log-level info
fi

# give wavs a chance to start up & health check
sleep 3
EOF

cp wavs.toml ${OPERATOR_LOC}/wavs.toml

echo "Operator ${OPERATOR_INDEX} created at ${OPERATOR_LOC}"
