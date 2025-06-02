#!/bin/bash

export RPC_URL=`bash ./script/get-rpc.sh`

# local: 127.0.0.1:5001
# testnet: https://app.pinata.cloud/. set PINATA_API_KEY to JWT token in .env

echo "Uploading to IPFS..."
export ipfs_cid=`SERVICE_FILE=${SERVICE_FILE} make upload-to-ipfs`

# LOCAL: http://127.0.0.1:8080
# TESTNET: https://gateway.pinata.cloud/
export IPFS_GATEWAY="$(bash script/get-ipfs-gateway.sh)"

export IPFS_URI="ipfs://${ipfs_cid}"

IPFS_URL="${IPFS_GATEWAY}${ipfs_cid}"
echo "IPFS_URL=${IPFS_URL}"

curl ${IPFS_URL}

if [ "$DEPLOYER_PK" ]; then
    echo ""
    echo "Setting service URI on WAVS Service Manager..."
    cast send ${WAVS_SERVICE_MANAGER_ADDRESS} 'setServiceURI(string)' "${IPFS_URI}" -r ${RPC_URL} --private-key ${DEPLOYER_PK}
fi

echo ""
echo "IPFS_GATEWAY=${IPFS_GATEWAY}"
echo "IPFS_URI=${IPFS_URI}"
