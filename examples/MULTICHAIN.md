# Multi-chain Example

* Start 2 networks (chain 1 and chain 2)
* Create a contract that WAVS watches on chain 2

```bash
# start chain 2 first since WAVS needs to watch it
anvil --chain-id 8645 --port 8645 &

# start chain 1 and deploy Eigen contracts
make start-all
```

```bash
# == Submission Chain (Ethereum) ==
export CHAIN_A=local
export CHAIN_A_RPC_URL=http://localhost:8545

# == Trigger Chain ==
# NOTE: wavs.toml must be configured to watch this chain
#       with `active_trigger_chains`
export CHAIN_B=local2
export CHAIN_B_RPC_URL=http://localhost:8645
```

```bash
# == Submission Contract (Chain 1) ==
export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy`
forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} ${CHAIN_A} true --sig "run(string,string,bool)" --rpc-url ${CHAIN_A_RPC_URL} --broadcast
export SERVICE_HANDLER=`make get-service-handler-from-deploy`

#  == Trigger Contract (Chain 2) ==
forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} ${CHAIN_B} false --sig "run(string,string,bool)" --rpc-url ${CHAIN_B_RPC_URL} --broadcast
export SERVICE_TRIGGER_ADDR=`CHAIN=${CHAIN} make get-trigger-from-deploy`
```

```bash
# Build your service JSON
# operators submit to chain A's service handler
export SUBMIT_CHAIN=${CHAIN_A}
export SUBMIT_ADDRESS=${SERVICE_HANDLER}
# while WAVS watches for actions on chain B's trigger address
export TRIGGER_CHAIN=${CHAIN_B}
export TRIGGER_ADDRESS=${SERVICE_TRIGGER_ADDR}
COMPONENT_FILENAME=eth_price_oracle.wasm sh ./script/build_service.sh

# Deploy the service JSON
SERVICE_CONFIG_FILE=.docker/service.json make deploy-service
```

```bash
# Execute on Chain B (trigger chain), which is picked up by WAVS
export COIN_MARKET_CAP_ID=1
forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig "run(string,string)" --rpc-url ${CHAIN_B_RPC_URL} --broadcast -v 4
```

```bash
# Get the latest TriggerId on the trigger chain
RPC_URL=${CHAIN_B_RPC_URL} make show-trigger-id

# grab the data from the submitted data on chain A
TRIGGER_ID=1 RPC_URL=${CHAIN_A_RPC_URL} SERVICE_SUBMISSION_ADDR=$SERVICE_HANDLER make show-result
```
