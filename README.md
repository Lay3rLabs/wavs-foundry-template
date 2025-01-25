# WAVS Monorepo Template

<!-- ![Rust](https://github.com/gakonst/foundry-rust-template/workflows/Rust/badge.svg)
![Solidity](https://github.com/gakonst/foundry-rust-template/workflows/Solidity/badge.svg)
[![Telegram Chat][tg-badge]][tg-url]

[tg-badge]:
  https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ffoundry_rs
[tg-url]: https://t.me/foundry_rs -->

**Template for quickly getting started with developing WAVS Rust applications**

A comprehensive template for developing WAVS (WebAssembly AVS) applications using Rust and Solidity. This template provides a pre-configured development environment with integrated testing frameworks for both Rust and Solidity components.

## Installation

Create a new project using this template:

```bash
# If you don't have forge: `curl -L https://foundry.paradigm.xyz | bash`

forge init --template Lay3rLabs/wavs-foundry-template my-wavs
```

## Testing

Given the repository contains both Solidity and Rust code, there's 2 different
workflows.

### Setting up the Environment

Initialize the submodule dependencies:

```bash
forge install
```

Build the contracts:

```bash
forge build
```

Run the tests:

```bash
forge test
```

## Rust

Rust bindings to the contracts can be generated via `forge bind`, which requires
first building your contracts:

```bash
make bindings
```

Then, you can run the tests:

```bash
cargo test
```

## WAVS

Build the latest solidity:

```bash
make build
```

Install the WAVS CLI:

```bash
# MacOS: if you get permission errors: eval `ssh-agent -s`; ssh-add
(cd lib/WAVS; cargo install --path ./packages/cli)
```

```bash
cp .env.example .env

# [!] Get your key from: https://openweathermap.org/
# Update the WAVS_ENV_OPEN_WEATHER_API_KEY in the .env file with your key`

cp ./lib/WAVS/packages/wavs/wavs.toml .
cp ./lib/WAVS/packages/cli/cli.toml .

# MacOS Docker:
# Docker Engine -> Settings -> Resources -> Network -> 'Enable Host Networking'
# or
# brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect
make start-all
```

Upload your WAVS Service contract

```bash
# Grab Eigenlayer contracts
export CLI_EIGEN_CORE_DELEGATION_MANAGER=`jq -r .eigen_core.local.delegation_manager ".docker/cli/deployments.json"`
export CLI_EIGEN_CORE_REWARDS_COORDINATOR=`jq -r .eigen_core.local.rewards_coordinator ".docker/cli/deployments.json"`
export CLI_EIGEN_CORE_AVS_DIRECTORY=`jq -r .eigen_core.local.avs_directory ".docker/cli/deployments.json"`

# Deploy
export FOUNDRY_ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
forge script ./script/WavsServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast

# Grab deployed service manager address by name
BROADCAST_FILE=./broadcast/WavsServiceManager.s.sol/31337/run-latest.json
export SERVICE_MANAGER_ADDRESS=`jq -r '.transactions[] | select(.contractName == "WavsServiceManager") | .contractAddress' "${BROADCAST_FILE}"`
echo "Service Manager Address: $SERVICE_MANAGER_ADDRESS"
```

Build WAVS WASI component(s)

```bash
# build all wasi components/*
# https://github.com/bytecodealliance/cargo-component#installation / cargo binstall cargo-component
make wasi-build

# Verify execution works as expected without deploying
# TODO: currently broken upstream
# wavs-cli exec --component $(pwd)/compiled/eth_trigger_weather.wasm --input Nashville,TN
```

Deploy service and verify with adding a task

```bash
sudo chmod 0666 .docker/cli/deployments.json

# Contract trigger function signature to listen for
trigger_event=$(cast sig-event "NewTrigger(bytes)"); echo $trigger_event

service_info=`wavs-cli deploy-service --log-level=error --data ./.docker/cli --component $(pwd)/compiled/eth_trigger_weather.wasm \
  --trigger-event-name ${trigger_event:2} \
  --trigger eth-contract-event \
  --submit-address ${SERVICE_MANAGER_ADDRESS} \
  --service-config '{"fuelLimit":100000000,"maxGas":5000000,"hostEnvs":["WAVS_ENV_OPEN_WEATHER_API_KEY"],"kv":[],"workflowId":"default","componentId":"default"}'`

echo "Service info: $service_info"

# Submit AVS request -> chain
SERVICE_ID=`echo $service_info | jq -r .service[0]`; echo "Service ID: $SERVICE_ID"
wavs-cli add-task --input "Nashville,TN" --data ./.docker/cli --service-id ${SERVICE_ID}

# Grab data from the contract directly
hex_bytes=$(cast decode-abi "getData(uint64)(bytes)" `cast call ${SERVICE_MANAGER_ADDRESS} "getData(uint64)" 1`)
echo `cast --to-ascii $hex_bytes`
```
