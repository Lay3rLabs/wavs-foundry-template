# Typescript Ethereum Price Oracle

A WAVS component that fetches the price of a crypto currency from CoinMarketCap and returns it to the Ethereum contract, in Typescript.

## System Setup

Follow the main [README.md](../../README.md) to install all the necessary dependencies.

## Core Packages

```bash docci-if-not-installed="cast"
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

```bash docci-if-not-installed="wasm-tools"
cargo binstall wasm-tools --no-confirm
```

```bash
make setup
```

```bash
npm --prefix ./components/js-evm-price-oracle/ install
```

## Build Component

Build all wasi components from the root of the repo. You can also run this command within each component directory.

```bash docci-output-contains="Successfully written"
# Builds only this component, not all.
warg reset
WASI_BUILD_DIR=js-evm-price-oracle make wasi-build
```

## Execute Component

Run the component with the `wasi-exec` command in the root of the repo

```bash docci-output-contains="LTC"
COMPONENT_FILENAME=js_evm_price_oracle.wasm COIN_MARKET_CAP_ID=2 make wasi-exec
```

---

## Run main README

Run through the main readme, but use `export COMPONENT_FILENAME=js_evm_price_oracle.wasm` instead of the default.
