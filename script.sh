#!/bin/bash


# TODO: allow setting ENV variables to override? (match makefile)
# COMPONENT_FILENAME, TRIGGER_EVENT, SERVICE_TRIGGER_ADDR, SERVICE_SUBMISSION_ADDR, SERVICE_CONFIG
# TODO: ideally the builder is in WAVS, but for now it's in this script

# Define output file
output_file="service_config.json"

# Get user input for the various values
echo "Setting up your service configuration..."

SERVICE_ID=$(uuidgen)
read -p "Enter Service Name: (default: $SERVICE_ID) " SERVICE_NAME
if [ -z "$SERVICE_NAME" ]; then
    SERVICE_NAME=$SERVICE_ID
fi

# Upload component 1 and get digest
# TODO: make the component upload have an argument
echo "Uploading component 1 (eth_price_oracle.wasm)..."
DIGEST_1=$(curl -s -X POST http://127.0.0.1:8000/upload \
     --data-binary @./compiled/eth_price_oracle.wasm \
     -H "Content-Type: application/wasm" | jq -r '.digest')

if [[ -z "$DIGEST_1" || "$DIGEST_1" == "null" ]]; then
    echo "Failed to upload component 1. Please check if the file exists and the server is running."
    exit 1
fi

# parse out the hex part of the digest by grabbing the raw values within .digest & removing sha256
DIGEST_1=$(echo $DIGEST_1 | cut -d':' -f2)
echo "Component 1 uploaded successfully. Digest: $DIGEST_1"

DEFAULT_TRIGGER_ADDR=`jq -r '.trigger' "./.docker/script_deploy.json"`
DEFAULT_SUBMIT_ADDRESS=`jq -r '.service_handler' "./.docker/script_deploy.json"`

# Get contract addresses
read -p "Enter Trigger Contract Address: (default: $DEFAULT_TRIGGER_ADDR) " TRIGGER_ADDRESS
read -p "Enter Submit Contract Address: (default: $DEFAULT_SUBMIT_ADDRESS) " SUBMIT_ADDRESS

if [ -z "$TRIGGER_ADDRESS" ]; then
    TRIGGER_ADDRESS=$DEFAULT_TRIGGER_ADDR
fi
if [ -z "$SUBMIT_ADDRESS" ]; then
    SUBMIT_ADDRESS=$DEFAULT_SUBMIT_ADDRESS
fi


DEFAULT_TRIGGER_EVENT=`cast keccak "NewTrigger(bytes)"`
read -p "Enter Event Hash: (default: $DEFAULT_TRIGGER_EVENT) " EVENT_HASH
if [ -z "$EVENT_HASH" ]; then
    EVENT_HASH=$DEFAULT_TRIGGER_EVENT
fi


# Create the JSON file with the provided values
cat > "$output_file" << EOF
{
  "id": "$SERVICE_ID",
  "name": "$SERVICE_NAME",
  "components": {
    "component1": {
      "wasm": "$DIGEST_1",
      "permissions": {
        "allowed_http_hosts": "all",
        "file_system": true
      },
      "source": {
        "Digest": "$DIGEST_1"
      }
    }
  },
  "workflows": {
    "workflow1": {
      "trigger": {
        "eth_contract_event": {
          "address": "$TRIGGER_ADDRESS",
          "chain_name": "local",
          "event_hash": "$EVENT_HASH"
        }
      },
      "component": "component1",
      "submit": {
        "ethereum_contract": {
          "chain_name": "local",
          "address": "$SUBMIT_ADDRESS",
          "max_gas": null
        }
      }
    }
  },
  "status": "active",
  "config": {
    "fuel_limit": 100000000,
    "host_envs": [],
    "kv": [],
    "max_gas": null
  },
  "testable": true
}
EOF

echo "Configuration file created as $output_file"
