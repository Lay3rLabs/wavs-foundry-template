
<https://enarx.dev/docs/Quickstart>

<!-- TODO; I think i have to maybe build from source? vs the wget https://github.com/enarx/enarx/releases/download/v0.7.1/enarx-x86_64-unknown-linux-musl-->
<!-- this required an SGX machine. Also does not seem their stack is updated to use wasm32-wasip1 instead of the old wasm32-wasi package, so may have to downgrade rust version to use -->

```bash
# https://enarx.dev/docs/WebAssembly/Rust
cargo build --release --target=wasm32-wasip1

# Something with WAVS breaks this when I tried to do Eth Price Oracle instead
enarx run ../../target/wasm32-wasip1/release/fibonacci.wasm
````
Error: failed to link module
Caused by:
    unknown import: wasi:http/types@0.2.4::[resource-drop]future-incoming-response has not been defined
````

sudo rm -rf ../../target/wasm32-wasip1
```
