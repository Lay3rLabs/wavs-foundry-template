
<https://enarx.dev/docs/Quickstart>

<!-- TODO; I think i have to maybe build from source? vs the wget https://github.com/enarx/enarx/releases/download/v0.7.1/enarx-x86_64-unknown-linux-musl-->
<!-- this required an SGX machine. Also does not seem their stack is updated to use wasm32-wasip1 instead of the old wasm32-wasi package, so may have to downgrade rust version to use -->

```bash
cd components/fibonacci

# https://enarx.dev/docs/WebAssembly/Rust
cargo build --release --target=wasm32-wasip1

# hacky BUT it does work and gets data into the wasm
echo 12 | enarx run ../../target/wasm32-wasip1/release/fibonacci.wasm
```
