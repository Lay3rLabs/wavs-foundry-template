#!/bin/bash

# set -x

: '''
# Run:

sh ./build_service.sh

# Overrides:
- FILE_LOCATION: The save location of the configuration file
- TRIGGER_EVENT: The event to trigger the service (e.g. "NewTrigger(bytes)")
- FUEL_LIMIT: The fuel limit (wasm compute metering) for the service
- MAX_GAS: The maximum chain gas for the submission Tx
- AGGREGATOR_URL: The URL of the aggregator service
'''

# == Defaults ==

FUEL_LIMIT=${FUEL_LIMIT:-1000000000000}
MAX_GAS=${MAX_GAS:-5000000}
FILE_LOCATION=${FILE_LOCATION:-".docker/service.json"}
TRIGGER_CHAIN=${TRIGGER_CHAIN:-"evm:31337"}
SUBMIT_CHAIN=${SUBMIT_CHAIN:-"evm:31337"}
AGGREGATOR_URL=${AGGREGATOR_URL:-""}
DEPLOY_ENV=${DEPLOY_ENV:-""}
REGISTRY=${REGISTRY:-"wa.dev"}
WAVS_SERVICE_MANAGER_ADDRESS=${WAVS_SERVICE_MANAGER_ADDRESS:-`task config:service-manager-address`}

# Function to substitute variables in config values
substitute_config_vars() {
    local config_str="$1"

    # Replace all ${VAR_NAME} patterns with their environment variable values
    while [[ "$config_str" =~ \$\{([^}]+)\} ]]; do
        var_name="${BASH_REMATCH[1]}"
        var_value="${!var_name}"
        if [ -z "$var_value" ]; then
            echo "⚠️  Warning: Variable ${var_name} is not set, using empty string" >&2
            var_value=""
        fi
        config_str="${config_str//\$\{${var_name}\}/${var_value}}"
    done

    echo "$config_str"
}

# Function to build config arguments from JSON object
build_config_args() {
    local config_json="$1"
    local args=""

    if [ -n "$config_json" ] && [ "$config_json" != "null" ] && [ "$config_json" != "{}" ]; then
        # Process each key-value pair
        echo "$config_json" | jq -c 'to_entries[]' | while IFS= read -r line; do
            key=$(echo "$line" | jq -r '.key')
            value=$(echo "$line" | jq -r '.value')
            # Substitute variables in the value
            value=$(substitute_config_vars "$value")
            args="${args} --values \"${key}=${value}\""
        done
    fi

    echo "$args"
}

# Function to build environment variable arguments
build_env_args() {
    local env_json="$1"
    local args=""

    if [ -n "$env_json" ] && [ "$env_json" != "null" ] && [ "$env_json" != "[]" ]; then
        # Process each environment variable
        echo "$env_json" | jq -c '.[]' | while IFS= read -r env_var; do
            env_var=$(echo "$env_var" | jq -r '.')
            args="${args} --values \"${env_var}\""
        done
    fi

    echo "$args"
}

BASE_CMD="docker run --rm --network host -w /data -v $(pwd):/data ghcr.io/lay3rlabs/wavs:1.4.1 wavs-cli service --json true --home /data --file /data/${FILE_LOCATION}"

if [ -z "$WAVS_SERVICE_MANAGER_ADDRESS" ]; then
    export WAVS_SERVICE_MANAGER_ADDRESS=$(jq -r '.contract' .docker/poa_sm_deploy.json)
    if [ -z "$WAVS_SERVICE_MANAGER_ADDRESS" ]; then
        echo "WAVS_SERVICE_MANAGER_ADDRESS is not set. Please set it to the address of the service manager."
        return
    fi
fi

if [ -z "$DEPLOY_ENV" ]; then
    DEPLOY_ENV=$(task get-deploy-status)
fi

# === Core ===

# Get PKG_NAMESPACE
if [ -z "$PKG_NAMESPACE" ]; then
    export PKG_NAMESPACE=`task get-wasi-namespace`
    if [ -z "$PKG_NAMESPACE" ]; then
        echo "PKG_NAMESPACE is not set. Please set the PKG_NAMESPACE environment variable."
        exit 1
    fi
fi

eval "${BASE_CMD} init --name en0va"

# Process component configurations from JSON file
if [ -z "${COMPONENT_CONFIGS_FILE}" ] || [ ! -f "${COMPONENT_CONFIGS_FILE}" ]; then
    # Try default location
    COMPONENT_CONFIGS_FILE="config/components.json"
    if [ ! -f "${COMPONENT_CONFIGS_FILE}" ]; then
        # Try .docker location
        COMPONENT_CONFIGS_FILE=".docker/components-config.json"
        if [ ! -f "${COMPONENT_CONFIGS_FILE}" ]; then
            echo "❌ Component configuration file not found"
            echo "Please specify COMPONENT_CONFIGS_FILE or ensure config/components.json or .docker/components-config.json exists"
            exit 1
        fi
    fi
fi

echo "Reading component configurations from: ${COMPONENT_CONFIGS_FILE}"

# Function to get aggregator component configuration
get_aggregator_config() {
    local config_file="$1"
    local config_json="{}"

    if [ -f "$config_file" ]; then
        config_json=$(jq '.aggregator_components[0] // {}' "$config_file")
    fi

    echo "$config_json"
}

# Get aggregator component configuration
AGGREGATOR_COMPONENT=$(get_aggregator_config "${COMPONENT_CONFIGS_FILE}")
AGG_PKG_NAME=$(echo "$AGGREGATOR_COMPONENT" | jq -r '.package_name // "aggregator"')
AGG_PKG_VERSION=$(echo "$AGGREGATOR_COMPONENT" | jq -r '.package_version // "0.1.0"')
AGG_CONFIG_VALUES=$(echo "$AGGREGATOR_COMPONENT" | jq '.config_values // {}')
AGG_ENV_VARIABLES=$(echo "$AGGREGATOR_COMPONENT" | jq '.env_variables // []')

# Export all required variables that might be used in config value substitutions
# These should be set by deploy-script.sh before calling this script
# echo "📋 Available configuration variables:"
# [ -n "${CHAIN_NAME}" ] && echo "  CHAIN_NAME: ${CHAIN_NAME}"
echo ""

jq -c '.components[]' "${COMPONENT_CONFIGS_FILE}" | while IFS= read -r component; do
    COMP_DISABLED=$(echo "$component" | jq -r '.disabled // false')
    if [ "$COMP_DISABLED" = "true" ]; then
        continue
    fi

    COMP_FILENAME=$(echo "$component" | jq -r '.filename')
    COMP_PKG_NAME=$(echo "$component" | jq -r '.package_name')
    COMP_PKG_VERSION=$(echo "$component" | jq -r '.package_version')
    COMP_SUBMIT_JSON_PATH=$(echo "$component" | jq -r '.submit_json_path')
    COMP_TRIGGER_BLOCK_INTERVAL=$(echo "$component" | jq -r '.trigger_block_interval // ""')
    COMP_TRIGGER_CRON_SCHEDULE=$(echo "$component" | jq -r '.trigger_cron.schedule // ""')
    COMP_TRIGGER_CRON_START_TIME=$(echo "$component" | jq -r '.trigger_cron.start_time // ""')
    COMP_TRIGGER_CRON_END_TIME=$(echo "$component" | jq -r '.trigger_cron.end_time // ""')

    # Extract component-specific config values and env variables
    COMP_CONFIG_VALUES=$(echo "$component" | jq '.config_values // {}')
    COMP_ENV_VARIABLES=$(echo "$component" | jq '.env_variables // []')

    echo "Creating workflow for component: ${COMP_FILENAME}"
    WORKFLOW_ID=`eval "$BASE_CMD workflow add" | jq -r .workflow_id`

    echo "  Workflow ID: ${WORKFLOW_ID}"
    echo "  Package: ${PKG_NAMESPACE}:${COMP_PKG_NAME}@${COMP_PKG_VERSION}"
    echo "  Submit: ${COMP_SUBMIT_ADDRESS}"

    if [ -n "$COMP_TRIGGER_BLOCK_INTERVAL" ]; then
        eval "$BASE_CMD workflow trigger --id ${WORKFLOW_ID} set-block-interval --chain ${TRIGGER_CHAIN} --n-blocks ${COMP_TRIGGER_BLOCK_INTERVAL}" > /dev/null

        echo "  Trigger block interval: ${COMP_TRIGGER_BLOCK_INTERVAL}"
    elif [ -n "$COMP_TRIGGER_CRON_SCHEDULE" ]; then
        # Build cron command arguments
        CRON_CMD_ARGS="--schedule '${COMP_TRIGGER_CRON_SCHEDULE}'"

        if [ "$COMP_TRIGGER_CRON_START_TIME" != "null" ] && [ -n "$COMP_TRIGGER_CRON_START_TIME" ]; then
            CRON_CMD_ARGS="${CRON_CMD_ARGS} --start-time ${COMP_TRIGGER_CRON_START_TIME}"
        fi

        if [ "$COMP_TRIGGER_CRON_END_TIME" != "null" ] && [ -n "$COMP_TRIGGER_CRON_END_TIME" ]; then
            CRON_CMD_ARGS="${CRON_CMD_ARGS} --end-time ${COMP_TRIGGER_CRON_END_TIME}"
        fi

        eval "$BASE_CMD workflow trigger --id ${WORKFLOW_ID} set-cron ${CRON_CMD_ARGS}" > /dev/null

        echo "  Trigger cron: ${COMP_TRIGGER_CRON_SCHEDULE}"
        if [ "$COMP_TRIGGER_CRON_START_TIME" != "null" ] && [ -n "$COMP_TRIGGER_CRON_START_TIME" ]; then
            echo "  Start time: ${COMP_TRIGGER_CRON_START_TIME}"
        fi
        if [ "$COMP_TRIGGER_CRON_END_TIME" != "null" ] && [ -n "$COMP_TRIGGER_CRON_END_TIME" ]; then
            echo "  End time: ${COMP_TRIGGER_CRON_END_TIME}"
        fi
    else
        COMP_TRIGGER_EVENT=$(echo "$component" | jq -r '.trigger_event')
        COMP_TRIGGER_JSON_PATH=$(echo "$component" | jq -r '.trigger_json_path')

        # Extract addresses from JSON paths
        COMP_TRIGGER_ADDRESS=`jq -r ".${COMP_TRIGGER_JSON_PATH}" .docker/deployment_summary.json`
        COMP_SUBMIT_ADDRESS=`jq -r ".${COMP_SUBMIT_JSON_PATH}" .docker/deployment_summary.json`

        # Validate addresses
        if [ -z "$COMP_TRIGGER_ADDRESS" ] || [ "$COMP_TRIGGER_ADDRESS" = "null" ]; then
            echo "❌ Trigger address not found for component: ${COMP_FILENAME} at path: ${COMP_TRIGGER_JSON_PATH}"
            exit 1
        fi
        if [ -z "$COMP_SUBMIT_ADDRESS" ] || [ "$COMP_SUBMIT_ADDRESS" = "null" ]; then
            echo "❌ Submit address not found for component: ${COMP_FILENAME} at path: ${COMP_SUBMIT_JSON_PATH}"
            exit 1
        fi

        COMP_TRIGGER_EVENT_HASH=`cast keccak ${COMP_TRIGGER_EVENT}`

        echo "  Trigger: ${COMP_TRIGGER_ADDRESS} (${COMP_TRIGGER_EVENT})"

        eval "$BASE_CMD workflow trigger --id ${WORKFLOW_ID} set-evm --address ${COMP_TRIGGER_ADDRESS} --chain ${TRIGGER_CHAIN} --event-hash ${COMP_TRIGGER_EVENT_HASH}" > /dev/null
    fi

    # Set submit to use aggregator component
    if [ -n "$AGGREGATOR_URL" ]; then
        eval "$BASE_CMD workflow submit --id ${WORKFLOW_ID} set-aggregator --url ${AGGREGATOR_URL}" > /dev/null

        # Configure aggregator component for this workflow
        echo "  📋 Configuring aggregator component"
        eval "$BASE_CMD workflow submit --id ${WORKFLOW_ID} component set-source-registry --domain ${REGISTRY} --package ${PKG_NAMESPACE}:${AGG_PKG_NAME} --version ${AGG_PKG_VERSION}" > /dev/null
        eval "$BASE_CMD workflow submit --id ${WORKFLOW_ID} component permissions --http-hosts '*' --file-system true" > /dev/null

        # Set aggregator component environment variables
        AGG_ENV_ARGS=$(build_env_args "$AGG_ENV_VARIABLES")
        if [ -n "$AGG_ENV_ARGS" ]; then
            eval "$BASE_CMD workflow submit --id ${WORKFLOW_ID} component env ${AGG_ENV_ARGS}" > /dev/null
        fi

        # Set aggregator component configuration routing
        # (( --values is found in ${AGG_CONFIG_ARGS} already ))
        AGG_CONFIG_ARGS=$(build_config_args "$AGG_CONFIG_VALUES")
        if [ -n "$AGG_CONFIG_ARGS" ]; then
            echo "  📋 Configuring aggregator (vars: ${AGG_CONFIG_ARGS})"
        fi
        eval "$BASE_CMD workflow submit --id ${WORKFLOW_ID} component config --values \"${SUBMIT_CHAIN}=${COMP_SUBMIT_ADDRESS}\" ${AGG_CONFIG_ARGS}" > /dev/null
    else
        eval "$BASE_CMD workflow submit --id ${WORKFLOW_ID} set-none" > /dev/null
    fi
    eval "$BASE_CMD workflow component --id ${WORKFLOW_ID} set-source-registry --domain ${REGISTRY} --package ${PKG_NAMESPACE}:${COMP_PKG_NAME} --version ${COMP_PKG_VERSION}"

    eval "$BASE_CMD workflow component --id ${WORKFLOW_ID} permissions --http-hosts '*' --file-system true" > /dev/null
    eval "$BASE_CMD workflow component --id ${WORKFLOW_ID} time-limit --seconds 30" > /dev/null

    # Set component-specific environment variables
    ENV_ARGS=$(build_env_args "$COMP_ENV_VARIABLES")
    if [ -n "$ENV_ARGS" ]; then
        echo "  📋 Setting environment variables"
        eval "$BASE_CMD workflow component --id ${WORKFLOW_ID} env ${ENV_ARGS}" > /dev/null
    fi

    # Set component-specific config values
    CONFIG_ARGS=$(build_config_args "$COMP_CONFIG_VALUES")
    if [ -n "$CONFIG_ARGS" ]; then
        echo "  📋 Configuring component"
        eval "$BASE_CMD workflow component --id ${WORKFLOW_ID} config ${CONFIG_ARGS}" > /dev/null
    # else
    #     echo "  ⚠️  No configuration values specified for ${COMP_FILENAME}"
    fi

    eval "$BASE_CMD workflow component --id ${WORKFLOW_ID} fuel-limit --fuel ${FUEL_LIMIT}" > /dev/null

    echo "  ✅ Workflow configured for ${COMP_FILENAME}"
    echo ""
done

eval "$BASE_CMD manager set-evm --chain ${SUBMIT_CHAIN} --address `cast --to-checksum ${WAVS_SERVICE_MANAGER_ADDRESS}`" > /dev/null
eval "$BASE_CMD validate" > /dev/null

echo "Configuration file created ${FILE_LOCATION}. Watching events from '${TRIGGER_CHAIN}' & submitting to '${SUBMIT_CHAIN}'."
