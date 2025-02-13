# WAVS Monorepo Template

<!-- ![Rust](https://github.com/gakonst/foundry-rust-template/workflows/Rust/badge.svg)
![Solidity](https://github.com/gakonst/foundry-rust-template/workflows/Solidity/badge.svg)
[![Telegram Chat][tg-badge]][tg-url]

[tg-badge]:
  https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Ffoundry_rs
[tg-url]: https://t.me/foundry_rs -->

## Base Required
- [docker](https://docs.docker.com/get-started/get-docker/)
  - MacOS: `brew install --cask docker`
  - Ubuntu: `sudo apt -y install docker.io`
- compose
  - Linux: `sudo apt-get install docker-compose-v2`
  - MacOS: already installed with docker installer
- [jq](https://jqlang.org/download/)
  - MacOS: `brew install jq`
  - Ubuntu: `sudo apt -y install jq`
- [Node v21+](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)


## Rust v1.84+
- rust
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Upgrade Rust
```
rustup target remove wasm32-wasi
rustup target remove wasm32-wasip1

# everyone will need this
rustup update stable
rustup target add wasm32-wasip2
```

## Carggo Components / wa.dev
- cargo install warg-cli wkg
- wkg config --default-registry wa.dev


## Specific
- [forge](https://github.com/foundry-rs/foundry)
  - `curl -L https://foundry.paradigm.xyz | bash`




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

> Install [`cargo install cargo-component --locked`](https://github.com/bytecodealliance/cargo-component#installation) if you have not already.

```bash
make wasi-build
```

> You can also use `make build` to build the contracts and components in one command

### Execute WASI component directly

```bash
make wasi-exec
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

> The `start-all` command must remain running in your terminal. Use another terminal to run other commands.
>
> You can stop the services with `ctrl+c`.

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
# Trigger contract via `script/Trigger.s.sol` for BTC
COIN_MARKET_CAP_ID=1 make trigger-service
```

## Show the result

```bash
# Get the latest TriggerId and show the result via `script/ShowResult.s.sol`
make show-result
```
