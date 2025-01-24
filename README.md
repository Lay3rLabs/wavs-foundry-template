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
(cd lib/WAVS; cargo install --path ./packages/cli)
```

```bash
cp .env.example .env

# [!] Get your key from: https://openweathermap.org/
# Update the WAVS_ENV_OPEN_WEATHER_API_KEY in the .env file with your key`

cp ./lib/WAVS/packages/wavs/wavs.toml .
cp ./lib/WAVS/packages/cli/cli.toml .

# start the WAVS network
docker compose up --build
```

Deploy Eigenlayer and upload your WAVS Service contract

```bash
docker_cmd="docker exec -it wavs bash -c"
export CLI_EIGEN_CORE_DELEGATION_MANAGER=`${docker_cmd} 'jq -r .eigen_core.local.delegation_manager ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_REWARDS_COORDINATOR=`${docker_cmd}  'jq -r .eigen_core.local.rewards_coordinator ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_AVS_DIRECTORY=`${docker_cmd}  'jq -r .eigen_core.local.avs_directory ~/wavs/cli/deployments.json' | tr -d '\r'`
export FOUNDRY_ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

forge script ./script/WavsServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast

# set this in the your terminal from the script output
export SERVICE_MANAGER_ADDRESS=0x851356ae760d987E095750cCeb3bC6014560891C
```

Build WAVS WASI component(s)

```bash
# build all components/*
make wasi-build

# Verify execution works as expected without deploying
wavs-cli exec --component $(pwd)/compiled/eth_trigger_weather.wasm --input Nashville,TN
```

Deploy service and verify with adding a task

```bash
sudo chmod 0666 .docker/cli/deployments.json

v=$(cast sig-event "NewTrigger(bytes)"); v=${v:2}; echo $v

wavs-cli deploy-service --data ./.docker/cli --component $(pwd)/compiled/eth_trigger_weather.wasm \
  --trigger-event-name ${v} \
  --trigger eth-contract-event \
  --submit-address ${SERVICE_MANAGER_ADDRESS} \
  --service-config '{"fuelLimit":100000000,"maxGas":5000000,"hostEnvs":["WAVS_ENV_OPEN_WEATHER_API_KEY"],"kv":[],"workflowId":"default","componentId":"default"}'

wavs-cli add-task --input "Nashville,TN" --data ./.docker/cli --service-id <Service-ID>

# Where the call address is the service manager in ./.docker/cli/deployments.json
hex_bytes=$(cast decode-abi "getData(uint64)(bytes)" `cast call ${SERVICE_MANAGER_ADDRESS} "getData(uint64)" 1`)
echo `cast --to-ascii $hex_bytes`
```
