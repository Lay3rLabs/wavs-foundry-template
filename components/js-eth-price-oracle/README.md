# JS

```bash
cd components/js-eth-price-oracle

npm install @bytecodealliance/componentize-js @bytecodealliance/jco

# WAVS_PACKAGE=wavs:worker@0.3.0-rc1
# cp /home/reece/Desktop/Programming/Rust/wavs/sdk/${WAVS_PACKAGE}.wasm .



# # npx jco transpile add.wasm -o dist

# # convert the index.ts to js
# npx tsc --outDir out/ --target es6 --strict --module preserve index.ts

# npx jco componentize out/index.js --wit /home/reece/Desktop/Programming/Rust/wavs/sdk/wit/ --world-name wavs:worker/layer-trigger-world --out ../../compiled/js_example.wasm

make wasi-build

(cd ../../ && make wasi-exec COMPONENT_FILENAME=js_eth_price_oracle.wasm)
```
