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


# solc --abi ../../src/interfaces/ITypes.sol
# abigen --abi ../../out/ITypes.sol/ITypes.json --pkg main --out ./abi.go


# ! TODO: fork go-ethereum
# 1) just pull over the common/ and abi/ packages without anything WS related?
# 2) fix the upstream to support wasm with https://github.com/coder/websocket | https://godoc.org/nhooyr.io/websocket#hdr-Wasm
#
# ../../../../../../.gvm/pkgsets/go1.23.1/global/pkg/mod/github.com/gorilla/websocket@v1.4.2/client.go:16:2: package net/http/httptrace is not in std (/home/reece/.cache/tinygo/goroot-ef1d5cacd8081d4a638fc91d6f688024fdfc368b55b569b858b4e97acf58cf33/src/net/http/httptrace)

(cd ../../; solc --abi src/interfaces/ITypes.sol --bin -o ./components/golang-eth-price-oracle/output_dir/)


mkdir -p submit
abigen --abi output_dir/src_interfaces_ITypes_sol_ITypes.abi --pkg submitcontract --type SubmitContract --out submit/contract.go

solc --abi ../../src/interfaces/ITypes.sol| awk '/JSON ABI/{x=1;next}x' > Store.abi
solc --bin ../../src/interfaces/ITypes.sol | awk '/Binary:/{x=1;next}x' > Store.bin
abigen --bin=Store.bin --abi=Store.abi --pkg=store --out=Store.go
```
