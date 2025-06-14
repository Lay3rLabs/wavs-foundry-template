
### Python

```bash
cd components/py-evm-price-oracle

python3 -m venv venv
source venv/bin/activate
# deactivate

pip install -r requirements.txt

# initial bindings gen
make check-package
make convert-wasm-to-wit
componentize-py --wit-path wavs:worker@0.4.0-beta.4.wit --world wavs:worker/layer-trigger-world bindings .
componentize-py --wit-path wavs:worker@0.4.0-beta.4.wit --world wavs:worker/layer-trigger-world bindings wavs_py
```



## WAVS

## Run

```bash
# compile
componentize-py --wit-path wavs:worker@0.4.0-beta.4.wit --world wavs:worker/layer-trigger-world componentize app -o ../../compiled/py_evm_price_oracle.wasm

# this must be run from the root of the repo to work, this is why we cd ../../
(cd ../../; make wasi-exec COMPONENT_FILENAME=py_evm_price_oracle.wasm COIN_MARKET_CAP_ID=2)
```
