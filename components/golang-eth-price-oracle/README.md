## Install Wit Bindgen for Go

```bash
go install go.bytecodealliance.org/cmd/wit-bindgen-go@ecfa620df5beee882fb7be0740959e5dfce9ae26

wit-bindgen-go --version
```

## System Setup

```bash
# https://component-model.bytecodealliance.org/language-support/go.html

# https://tinygo.org/getting-started/install/

# macOS
brew tap tinygo-org/tools
brew install tinygo

# Arch (btw)
sudo pacman -Sy tinygo

# Ubuntu / WSL:
# TODO: .


# -------

# verify installs
tinygo version
wkg --version

# move into the golang oracle directory

cd components/golang-eth-price-oracle

# download the WAVS package bindings
export WAVS_PACKAGE=wavs:worker@0.3.0-rc1
# TODO: this is currently broken on this release, requires:
# TODO: https://github.com/Lay3rLabs/WAVS/pull/403 to fix `failed to resolve import `wasi:cli/environment@0.2.0::get-environment`

cp /home/reece/Desktop/Programming/Rust/wavs/sdk/wavs:worker@0.3.0-rc1.wasm .
# wkg get $WAVS_PACKAGE --overwrite --format wasm --output ${WAVS_PACKAGE}.wasm

# generate the Go/ bindings
wit-bindgen-go generate -o internal/ ${WAVS_PACKAGE}.wasm

# install
go mod tidy
tinygo build -target=wasip2 -o ../../compiled/golang-wavs-example.wasm --wit-package ${WAVS_PACKAGE}.wasm --wit-world wavs:worker/layer-trigger-world main.go

(cd ../../; make wasi-exec COMPONENT_FILENAME=golang-wavs-example.wasm)
```

```bash
cd ../../
make start-all

# new tab
sh script/go.sh
```
