forge build

make deploy-contracts
make deploy-service COMPONENT_FILENAME="js_eth_price_oracle.wasm"

COIN_MARKET_CAP_ID=1337 make trigger-service

sleep 1

make show-result
