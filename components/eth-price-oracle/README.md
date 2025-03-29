Better to reference the fibonacci for base

```bash
# normal
WASI_BUILD_DIR=components/eth-price-oracle make wasi-build # eth_price_oracle.wasm
COIN_MARKET_CAP_ID=1 make wasi-exec

# hacky BUT it does work and gets data into the wasm
# cargo build --release --target=wasm32-wasip1 # eth-price-oracle.wasm
echo 12 | enarx run target/wasm32-wasip1/release/eth_price_oracle.wasm
```
