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

### Solidity

```bash
# Initialize the dependencies
forge install && npm install

# Build the contracts
forge build

# Run the solidity tests. alias: `make test`
forge test
```

> You can also use `make build` to build the contracts, bindings, and components.

## Rust

```bash
# Generate new bindings from your contract(s) alias: `make build`
make bindings

# Run rust tests
make test
```

## WAVS

### Start Anvil, WAVS, and Deploy Eigenlayer

```bash
# copy over the .env file
cp .env.example .env

# MacOS Docker:
# Docker Engine -> Settings -> Resources -> Network -> 'Enable Host Networking'
# or
# brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect
make start-all
```

### Upload your WAVS Service Manager

```bash
# Required for `forge script`
sudo chmod 0666 .docker/cli/deployments.json
alias wavs-cli="docker run --network host --env-file ./.env -v $(pwd):/data ghcr.io/lay3rlabs/wavs:0.3.0-alpha5 wavs-cli"

# Deploy contracts
export SERVICE_MANAGER=`jq -r '.eigen_service_managers.local | .[-1]' .docker/cli/deployments.json`
export FOUNDRY_ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
forge script ./script/Deploy.s.sol ${SERVICE_MANAGER} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast

# Get deployed contracts
export SERVICE_HANDLER_ADDR=`jq -r '.service_handler' "./.docker/cli/script_deploy.json"`
export TRIGGER_ADDR=`jq -r '.trigger' "./.docker/cli/script_deploy.json"`
```

### Build WASI components

> Install `cargo binstall cargo-component` if you have not already. -- https://github.com/bytecodealliance/cargo-component#installation

```bash
make wasi-build

# TODO: currently broken upstream
# Verify execution works as expected without deploying
# wavs-cli exec --component $(pwd)/compiled/eth_price_oracle.wasm --input `cast format-bytes32-string 1`
```

## Deploy Service

```bash
# Contract trigger function signature to listen for
trigger_event=$(cast sig-event "NewTrigger(bytes)"); echo "Trigger Event: $trigger_event"

wavs-cli deploy-service --log-level=error --data /data/.docker/cli --home /data \
    --component /data/compiled/eth_price_oracle.wasm \
    --trigger-event-name ${trigger_event:2} \
    --trigger eth-contract-event \
    --trigger-address ${TRIGGER_ADDR} \
    --submit-address ${SERVICE_HANDLER_ADDR} \
    --service-config '{"fuel_limit":100000000,"max_gas":5000000,"host_envs":[],"kv":[],"workflow_id":"default","component_id":"default"}'
```

## Submit Request and Verify

```bash
# Submit request -> chain
cast send ${TRIGGER_ADDR} "addTrigger(bytes)" `cast format-bytes32-string 1` --rpc-url http://localhost:8545 --private-key $FOUNDRY_ANVIL_PRIVATE_KEY

# Verify
ID=`cast call ${TRIGGER_ADDR} "nextTriggerId()" --rpc-url http://localhost:8545`; echo "ID: $ID"
cast --to-ascii $(cast decode-abi "getData(uint64)(bytes)" `cast call ${SERVICE_HANDLER_ADDR} "getData(uint64)" $ID`)
```
