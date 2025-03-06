forge build

make deploy-contracts
make deploy-service COMPONENT_FILENAME="golang_eth_price_oracle.wasm"

COIN_MARKET_CAP_ID=1 make trigger-service

sleep 1

make show-result
