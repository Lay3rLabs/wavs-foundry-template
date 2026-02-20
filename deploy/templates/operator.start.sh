#!/bin/bash

cd $(dirname "$0") || return

IMAGE=INSERT_IMAGE_HERE
INSTANCE=INSERT_INSTANCE_NAME_HERE
IPFS_GATEWAY=${IPFS_GATEWAY:-"https://gateway.pinata.cloud/ipfs/"}

docker kill ${INSTANCE} > /dev/null 2>&1 || true
docker rm ${INSTANCE} > /dev/null 2>&1 || true

docker run -d --name ${INSTANCE} --network host --env-file .env -v .:/root/wavs ${IMAGE} wavs --home /root/wavs --ipfs-gateway ${IPFS_GATEWAY} --host 0.0.0.0 --log-level info
sleep 0.25

if [ ! "$(docker ps -q -f name=${INSTANCE})" ]; then
  echo "Container ${INSTANCE} is not running. Reason:"
  docker run --name ${INSTANCE} --network host --env-file .env -v .:/root/wavs ${IMAGE} wavs --home /root/wavs --ipfs-gateway ${IPFS_GATEWAY} --host 0.0.0.0 --log-level info
fi

# if first argument is "log", tail the logs immediately
if [ "$1" = "log" ]; then
  echo "Tailing logs for ${INSTANCE}..."
  docker logs -f ${INSTANCE}
else
  # otherwise give WAVS a chance to start up & health check
  echo "Giving WAVS operator 3 seconds to start up & health check..."
  sleep 3
fi
