# WAVS Monorepo Template

**Template for quickly getting started with developing WAVS Rust applications**

A comprehensive template for developing WAVS (WebAssembly AVS) applications using Rust and Solidity. This template provides a pre-configured development environment with integrated testing frameworks for both Rust and Solidity components.

## Developer Tools Installation

<details>
<summary>System Requirements (Docker, Compose, Make, JQ, NodeJS v21+)</summary>

### Docker
- **MacOS**: `brew install --cask docker`
- **Ubuntu**: `sudo apt -y install docker.io`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

### Docker Compose
- **Linux**: `sudo apt-get install docker-compose-v2`
- **MacOS**: Already installed with Docker installer
- [Compose Documentation](https://docs.docker.com/compose/)

### Make
- **Linux**: `sudo apt -y install make`
- **MacOS**: `brew install make`
- [Make Documentation](https://www.gnu.org/software/make/manual/make.html)

### JQ
- **MacOS**: `brew install jq`
- **Ubuntu**: `sudo apt -y install jq`
- [JQ Documentation](https://jqlang.org/download/)

### Node.js
- **Required Version**: v21+
- [Installation via NVM](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)
</details>

<details>

<summary>Rust v1.84+</summary>

### Initial Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Upgrade Process

```bash
# Remove old targets
rustup target remove wasm32-wasi
rustup target remove wasm32-wasip1

# Update and add required target
rustup update stable
rustup target add wasm32-wasip2
```

</details>

<details>
<summary>Cargo Components & wa.dev</summary>

```bash
# Install required cargo components
cargo install warg-cli wkg

# Configure default registry
wkg config --default-registry wa.dev
```

</details>

## New Project

Create a new project using this template:

```bash
# If you don't have foundry or forge: `curl -L https://foundry.paradigm.xyz | bash`
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
> You can stop the services with `ctrl+c`. Some MacOS terminals require pressing this twice.

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
