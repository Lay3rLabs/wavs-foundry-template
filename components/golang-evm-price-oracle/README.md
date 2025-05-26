# Golang Ethereum Price Oracle

A WAVS component that fetches the price of a crypto currency from CoinMarketCap and returns it to the Ethereum contract, in Go.

## System Setup

### Mac

```bash docci-os=mac
brew tap tinygo-org/tools
brew install tinygo
```

### Arch Linux

```bash docci-ignore
sudo pacman -Sy tinygo
```

### Ubuntu Linux

```bash docci-os=linux docci-if-not-installed="tinygo"
# https://tinygo.org/getting-started/install/linux/
wget --quiet https://github.com/tinygo-org/tinygo/releases/download/v0.37.0/tinygo_0.37.0_amd64.deb
sudo dpkg -i tinygo_0.37.0_amd64.deb && rm tinygo_0.37.0_amd64.deb
```

## Core Packages

```bash docci-if-not-installed="cast"
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

```bash
make setup
```

```bash docci-if-not-installed="cargo-binstall"
cargo install cargo-binstall
```

```bash docci-if-not-installed="wasm-tools"
cargo binstall wasm-tools --no-confirm
```

<!-- matches the value in the wavs-wasi for generation of the bindings -->
```bash occi-if-not-installed="wit-bindgen-go"
go install go.bytecodealliance.org/cmd/wit-bindgen-go@ecfa620df5beee882fb7be0740959e5dfce9ae26
wit-bindgen-go --version
```

## Verify installs

```bash
tinygo version
wkg --version
```

## Build Component

Build all wasi components from the root of the repo. You can also run this command within each component directory.

```bash
# Builds only this component, not all.
WASI_BUILD_DIR=golang-evm-price-oracle make wasi-build
```

## Execute Component

Run the component with the `wasi-exec` command in the root of the repo

```bash docci-output-contains="LTC"
COMPONENT_FILENAME=golang_evm_price_oracle.wasm COIN_MARKET_CAP_ID=2 make wasi-exec
```

---

## Run main README

Run through the main readme, but use `export COMPONENT_FILENAME=golang_evm_price_oracle.wasm` instead of the default.
