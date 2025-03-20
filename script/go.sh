# TODO: move to docci

forge build

# deploy contracts
export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy`
forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast


# deploy component
WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS=true
DEFAULT_COMPONENT_FILENAME=golang_eth_price_oracle.wasm sh ./script.sh
SERVICE_CONFIG_FILE=service_config.json make deploy-service


# trigger service
export COIN_MARKET_CAP_ID=1
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig "run(string,string)" --rpc-url http://localhost:8545 --broadcast -v 4

sleep 1

make show-result
