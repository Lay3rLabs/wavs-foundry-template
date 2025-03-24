# JS

```bash
cd components/js-eth-price-oracle

npm install @bytecodealliance/componentize-js @bytecodealliance/jco

make wasi-build # `too many arguments` error is fine for now

(cd ../../ && make wasi-exec COMPONENT_FILENAME=js_eth_price_oracle.wasm)
```
