#!/bin/bash

export WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS=${WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS:-"true"}

# Define output file
output_file=".docker/service_config.json"

# Get user input for the various values
echo "Setting up your service configuration..."

SERVICE_ID=$(uuidgen)
if [ "$WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS" == "true" ]; then
    echo "Using default values for all prompts. To disable this, set WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS=false"
else
    read -p "Enter Service Name: (default: $SERVICE_ID) " SERVICE_NAME
fi
if [ -z "$SERVICE_NAME" ]; then
    SERVICE_NAME=$SERVICE_ID
fi

# Upload component 1 and get digest

DEFAULT_COMPONENT_FILENAME=eth_price_oracle.wasm
if [ "$WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS" == "true" ]; then
    echo "Using default values for all prompts. To disable this, set WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS=false"
else
  read -p "Enter Component Filename: (default: $DEFAULT_COMPONENT_FILENAME) " COMPONENT_FILENAME
fi
if [ -z "$COMPONENT_FILENAME" ]; then
    COMPONENT_FILENAME=$DEFAULT_COMPONENT_FILENAME
fi
export COMPONENT_FILENAME=$COMPONENT_FILENAME

WASM_DIGEST=`make upload-component`
if [[ -z "$WASM_DIGEST" || "$WASM_DIGEST" == "null" ]]; then
    echo "Failed to upload component 1. Please check if the file exists and the server is running."
    exit 1
fi

# parse out the hex part of the digest by grabbing the raw values within .digest & removing sha256
WASM_DIGEST=$(echo $WASM_DIGEST | cut -d':' -f2)
echo "Component 1 uploaded successfully. Digest: $WASM_DIGEST"

DEFAULT_TRIGGER_ADDR=`jq -r '.trigger' "./.docker/script_deploy.json"`
DEFAULT_SUBMIT_ADDRESS=`jq -r '.service_handler' "./.docker/script_deploy.json"`

# Get contract addresses
if [ "$WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS" == "true" ]; then
    echo "Using default values for all prompts. To disable this, set WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS=false"
else
  read -p "Enter Trigger Contract Address: (default: $DEFAULT_TRIGGER_ADDR) " TRIGGER_ADDRESS
  read -p "Enter Submit Contract Address: (default: $DEFAULT_SUBMIT_ADDRESS) " SUBMIT_ADDRESS
fi

if [ -z "$TRIGGER_ADDRESS" ]; then
    TRIGGER_ADDRESS=$DEFAULT_TRIGGER_ADDR
fi
if [ -z "$SUBMIT_ADDRESS" ]; then
    SUBMIT_ADDRESS=$DEFAULT_SUBMIT_ADDRESS
fi


DEFAULT_TRIGGER_EVENT=`cast keccak "NewTrigger(bytes)"`
if [ "$WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS" == "true" ]; then
    echo "Using default values for all prompts. To disable this, set WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS=false"
else
  read -p "Enter Event Hash: (default: $DEFAULT_TRIGGER_EVENT) " EVENT_HASH
fi
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
      "wasm": "$WASM_DIGEST",
      "permissions": {
        "allowed_http_hosts": "all",
        "file_system": true
      },
      "source": {
        "Digest": "$WASM_DIGEST"
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
