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

```bash docci-ignore
# https://tinygo.org/getting-started/install/linux/
wget https://github.com/tinygo-org/tinygo/releases/download/v0.37.0/tinygo_0.37.0_amd64.deb
sudo dpkg -i tinygo_0.37.0_amd64.deb
```

## Verify installs

```bash
tinygo version
wkg --version
```

## Build Component

Build all wasi components from the root of the repo. You can also run this command within each component directory.

```bash docci-output-contains="component built"
make wasi-build
```

## Execute Component

Run the component with the `wasi-exec` command in the root of the repo

```bash docci-output-contains="LTC"
COMPONENT_FILENAME=golang_eth_price_oracle.wasm COIN_MARKET_CAP_ID=2 make wasi-exec
```

## Run in a local environment

Start all services

```bash docci-background docci-delay-after=5
make start-all
```

Build your smart contracts

```bash
forge build
```

Deploy the contracts

```bash docci-delay-after=1
export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy`

forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast
```

Deploy the component

```bash docci-delay-after=1
WAVS_SCRIPT_ACCEPT_ALL_DEFAULTS=true

DEFAULT_COMPONENT_FILENAME=golang_eth_price_oracle.wasm sh ./script.sh

SERVICE_CONFIG_FILE=service_config.json make deploy-service
```

Trigger the service

```bash docci-delay-after=1
export COIN_MARKET_CAP_ID=1
export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`

forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig "run(string,string)" --rpc-url http://localhost:8545 --broadcast -v 4
```

Show the result from the triggered service

```bash docci-output-contains="BTC"
make show-result
```
