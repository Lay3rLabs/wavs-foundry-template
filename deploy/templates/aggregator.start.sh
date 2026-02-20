#!/bin/bash

cd $(dirname "$0") || return

IMAGE=INSERT_IMAGE_HERE
INSTANCE=INSERT_INSTANCE_NAME_HERE
IPFS_GATEWAY=${IPFS_GATEWAY:-"https://gateway.pinata.cloud/ipfs/"}

docker kill ${INSTANCE} > /dev/null 2>&1 || true
docker rm ${INSTANCE} > /dev/null 2>&1 || true

docker run -d --name ${INSTANCE} --network host --stop-signal SIGKILL --env-file .env -v .:/wavs -v ./data:/root/wavs ${IMAGE} wavs-aggregator --log-level info --host 0.0.0.0 --ipfs-gateway ${IPFS_GATEWAY}

# if first argument is "log", tail the logs immediately
if [ "$1" = "log" ]; then
  echo "Tailing logs for ${INSTANCE}..."
  docker logs -f ${INSTANCE}
else
  # otherwise give WAVS a chance to start up & health check
  echo "Giving WAVS aggregator 1 second to start up & health check..."
  sleep 1
fi
