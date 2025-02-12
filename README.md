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
# Install initial dependencies
make setup

# Build the contracts
forge build

# Run the solidity tests.
forge test
```

### Build WASI components

> Install `cargo binstall cargo-component` if you have not already. -- https://github.com/bytecodealliance/cargo-component#installation

```bash
make wasi-build

# TODO: currently broken upstream
# Verify execution works as expected without deploying
# wavs-cli exec --component $(pwd)/compiled/eth_price_oracle.wasm --input `cast format-bytes32-string 1`
```

> You can also use `make build` to build the contracts and components in one command

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

### Upload your Service's Trigger and Submission contracts

```bash
# Deploy submission and trigger contract's from `script/Deploy.s.sol`
make deploy-contracts
```

>
> You can see the deployed trigger address with `make get-trigger-from-deploy`
>
> You can see the deployed submission address with `make get-service-handler-from-deploy`

## Deploy Service

```bash
make deploy-service
```

## Trigger the Service

```bash
# Trigger contract via `script/Trigger.s.sol`
COIN_MARKET_CAP_ID=1 make trigger-service
```

## Show the result

```bash
# Get the latest TriggerId and show the result via `script/ShowResult.s.sol`
make show-result
```
