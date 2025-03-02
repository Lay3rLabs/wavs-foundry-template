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

# verify installs
tinygo version
wkg --version
```

## Build Component

```bash
cd components/golang-eth-price-oracle

make wasi-build

(cd ../../; make wasi-exec COMPONENT_FILENAME=golang_eth_price_oracle.wasm COIN_MARKET_CAP_ID=2)
```

## Run in a local environment

```bash
cd ../../
make start-all

# new tab
sh script/go.sh
```
