```bash
git clone git@github.com:bytecodealliance/go-modules.git
cd go-modules
go install ./cmd/wit-bindgen-go

cargo binstall wasm-tools

wit-bindgen-go
```

```bash
# https://component-model.bytecodealliance.org/language-support/go.html
# wit-bindgen-go generate ../wasi-cli/wit

# https://tinygo.org/getting-started/install/
sudo pacman -Sy tinygo

# TODO: move to use docker only instead?
# docker run --rm -v $(pwd):/src tinygo/tinygo:0.35.0 tinygo build -o wasm.wasm -target=wasm examples/wasm/export



tinygo version
wasm-tools -V
# TOTO: ensure wkg is installed

cd /home/reece/Desktop/Programming/Rust/wavs/sdk/

wkg wit build

cd components/golang-eth-price-oracle

# cp /home/reece/Desktop/Programming/Rust/wavs/sdk/wavs:worker@0.3.0-beta.wasm .

go get go.bytecodealliance.org/cmd/wit-bindgen-go
go run go.bytecodealliance.org/cmd/wit-bindgen-go generate -o internal/ ./example:component@0.1.0.wasm

# TODO: i had to manually modify the internal/example/component to return an int32 ?? why
tinygo build -target=wasip2 -o add.wasm --wit-package wit/ --wit-world example main.go


# this is only for testing, we will use the wavs-cli wasi-exec command as the real impl vs this
git clone git@github.com:bytecodealliance/component-docs.git
cd component-docs/component-model/examples/example-host
cargo run --release -- 1 2 /home/reece/Desktop/Programming/Rust/wavs-foundry-template/components/golang-eth-price-oracle/add.wasm



```
