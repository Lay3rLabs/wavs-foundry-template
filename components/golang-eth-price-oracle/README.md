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

make wasi-build

(cd ../../; make wasi-exec COMPONENT_FILENAME=golang_eth_price_oracle.wasm)
```

```bash
cd ../../
make start-all

# new tab
sh script/go.sh
```
