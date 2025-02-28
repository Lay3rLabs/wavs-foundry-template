```bash
wkg get wasi:cli@0.2.0
# move it into wit/deps/NAME-version/package.wit

wkg get wasi:filesystem@0.2.0
wkg get wasi:sockets@0.2.0
wkg get wasi:random@0.2.0
```


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

# TODO: we should also include these in the release process so that we can just curl down?
wit-bindgen-go generate -o internal/ /home/reece/Desktop/Programming/Rust/wavs/sdk/wavs:worker@0.3.0-beta.wasm


go mod tidy
tinygo build -target=wasip2 -o ../../compiled/golang-wavs-example.wasm --wit-package /home/reece/Desktop/Programming/Rust/wavs/sdk/wavs:worker@0.3.0-beta.wasm --wit-world wavs:worker/layer-trigger-world main.go

go build

(cd ../../; make wasi-exec COMPONENT_FILENAME=golang-wavs-example.wasm)
```
