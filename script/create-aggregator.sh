#!/usr/bin/bash
set -e
# set -x

# have an optional argument $1, if set, use it as the agg index
# otherwise, use the default of 1
if [ -n "$1" ]; then
    AGGREGATOR_INDEX=$1
fi
if [ -z "$AGGREGATOR_INDEX" ]; then
    AGGREGATOR_INDEX=1
fi

if [ -z "$DEPLOY_ENV" ]; then
    DEPLOY_ENV=$(sh ./script/get-deploy-status.sh)
fi
if [ -z "$RPC_URL" ]; then
    RPC_URL=`sh ./script/get-rpc.sh`
fi

SP=""; if [[ "$(uname)" == *"Darwin"* ]]; then SP=" "; fi

cd $(git rev-parse --show-toplevel) || exit 1

mkdir -p .docker

# == Generate a new aggregator ==
TEMP_FILENAME=".docker/tmp.json"
cast wallet new-mnemonic --json > ${TEMP_FILENAME}
export AGG_MNEMONIC=`jq -r .mnemonic ${TEMP_FILENAME}`
export AGG_PK=`jq -r .accounts[0].private_key ${TEMP_FILENAME}`
AGGREGATOR_ADDR=`cast wallet address $AGG_PK`

# == infra files ==
AGG_LOC=infra/aggregator-${AGGREGATOR_INDEX}
mkdir -p ${AGG_LOC}

ENV_FILENAME="${AGG_LOC}/.env"
cp ./script/template/.env.example.aggregator ${ENV_FILENAME}

sed -i${SP}'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$/WAVS_AGGREGATOR_CREDENTIAL=\"$AGG_PK\"/" ${ENV_FILENAME}
sed -i${SP}'' -e "s/.%%MNEMONIC_REFERENCE%%$/ $AGG_MNEMONIC/" ${ENV_FILENAME}

cat > "${AGG_LOC}/start.sh" << EOF
#!/bin/bash
cd \$(dirname "\$0") || exit 1

IMAGE=ghcr.io/lay3rlabs/wavs:0.4.0-rc
INSTANCE=wavs-aggregator-${AGGREGATOR_INDEX}
IPFS_GATEWAY=\${IPFS_GATEWAY:-"https://gateway.pinata.cloud/ipfs/"}

docker kill \${INSTANCE} > /dev/null 2>&1 || true
docker rm \${INSTANCE} > /dev/null 2>&1 || true

docker run -d --name \${INSTANCE} --network host --stop-signal SIGKILL --env-file .env --user 1000:1000 -v .:/wavs \\
  \${IMAGE} wavs-aggregator --log-level debug --host 0.0.0.0 --port 8001 --ipfs-gateway \${IPFS_GATEWAY}

# give it a chance to start up
sleep 1
EOF

cp wavs.toml ${AGG_LOC}/wavs.toml

if [ "$DEPLOY_ENV" = "LOCAL" ]; then
    # Good DevEx, auto fund the deployer
    cast rpc anvil_setBalance "${AGGREGATOR_ADDR}" '15000000000000000000' --rpc-url ${RPC_URL} > /dev/null

    BAL=`cast balance --ether $AGGREGATOR_ADDR --rpc-url=${RPC_URL}`
    echo "Local aggregator \`${AGGREGATOR_ADDR}\` funded with ${BAL}ether"
else
    # New account on testnet, must be funded externally (i.e. metamask)
    echo "Fund aggregator ${AGGREGATOR_ADDR} with some ETH, or change this value in ${ENV_FILENAME}"
    sleep 5

    while true; do
        BALANCE=`cast balance --ether $AGGREGATOR_ADDR --rpc-url=${RPC_URL}`
        if [ "$BALANCE" != "0.000000000000000000" ]; then
            echo "Account balance is now $BALANCE"
            break
        fi
        echo "      [!] Waiting for balance to be funded by another account to this account..."
        sleep 5
    done
fi
