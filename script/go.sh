forge build

make deploy-contracts
make deploy-service COMPONENT_FILENAME="golang-wavs-example.wasm"

COIN_MARKET_CAP_ID=1 make trigger-service

sleep 1

make show-result
