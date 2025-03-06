
### Python

```bash
# create env
python3 -m venv venv

source venv/bin/activate

pip install componentize-py
pip freeze > requirements.txt

# initial bindings gen
componentize-py --wit-path add.wit --world example bindings .

# impl with the app
componentize-py --wit-path add.wit --world example componentize app -o add.wasm
```



## WAVS

```bash
# Get bindings
componentize-py --wit-path /home/reece/Desktop/Programming/Rust/wavs/sdk/wit --world wavs:worker/layer-trigger-world bindings wavs_py

# Create component
componentize-py --wit-path /home/reece/Desktop/Programming/Rust/wavs/sdk/wit --world wavs:worker/layer-trigger-world componentize main -o ../../compiled/python.wasm
```

## Run

```bash
(cd ../../; make wasi-exec COMPONENT_FILENAME=python.wasm COIN_MARKET_CAP_ID=2)
```
