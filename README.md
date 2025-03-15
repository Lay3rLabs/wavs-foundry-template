# [WAVS](https://docs.wavs.xyz) Monorepo Template

**Template for getting started with developing WAVS applications**

A template for developing WebAssembly AVS applications using Rust and Solidity, configured for Windows *WSL*, Linux, and MacOS. The sample oracle service fetches the current price of a cryptocurrency from [CoinMarketCap](https://coinmarketcap.com) and saves it on chain.

## System Requirements

<details>
<summary>Core (Docker, Compose, Make, JQ, Node v21+)</summary>

### Docker
- **MacOS**: `brew install --cask docker`
- **Linux**: `sudo apt -y install docker.io`
- **Windows WSL**: [docker desktop wsl](https://docs.docker.com/desktop/wsl/#turn-on-docker-desktop-wsl-2) & `sudo chmod 666 /var/run/docker.sock`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

### Docker Compose
- **MacOS**: Already installed with Docker installer
- **Linux + Windows WSL**: `sudo apt-get install docker-compose-v2`
- [Compose Documentation](https://docs.docker.com/compose/)

### Make
- **MacOS**: `brew install make`
- **Linux + Windows WSL**: `sudo apt -y install make`
- [Make Documentation](https://www.gnu.org/software/make/manual/make.html)

### JQ
- **MacOS**: `brew install jq`
- **Linux + Windows WSL**: `sudo apt -y install jq`
- [JQ Documentation](https://jqlang.org/download/)

### Node.js
- **Required Version**: v21+
- [Installation via NVM](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)
</details>

<details>

<summary>Rust v1.84+</summary>

### Rust Installation

```bash docs-ci-ignore
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install stable
rustup target add wasm32-wasip2
```

### Upgrade Rust

```bash docs-ci-ignore
# Remove old targets if present
rustup target remove wasm32-wasi || true
rustup target remove wasm32-wasip1 || true

# Update and add required target
rustup update stable
rustup target add wasm32-wasip2
```

</details>

<details>
<summary>Cargo Components</summary>

### Install Cargo Components

```bash docs-ci-ignore
# Install required cargo components
# https://github.com/bytecodealliance/cargo-component#installation
cargo install cargo-binstall
cargo binstall cargo-component warg-cli wkg --locked --no-confirm

# Configure default registry
wkg config --default-registry wa.dev
```

</details>

## Create Project

<!-- TODO: this would be unignored and run from an external CI, but not for now -->
```bash docs-ci-ignore
# If you don't have foundry: `curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup`
forge init --template Lay3rLabs/wavs-foundry-template my-wavs --branch main
```

> [!TIP]
> Run `make help` to see all available commands and environment variable overrides.

### Solidity

Install the required packages to build the Solidity contracts. This project supports both [submodules](./.gitmodules) and [npm packages](./package.json).

```bash
# Install packages (npm & submodules)
make setup

# Build the contracts
forge build

# Run the solidity tests
forge test
```

### Build WASI components

Now build the WASI rust components into the `compiled` output directory.

> [!WARNING]
> If you get: `error: no registry configured for namespace "wavs"`
>
> run, `wkg config --default-registry wa.dev`

```bash
make wasi-build # or `make build` to include solidity compilation.
```

### Execute WASI component directly

Test run the component locally to validate the business logic works. An ID of 1 is Bitcoin. Nothing will be saved on-chain, just the output of the component is shown.

```bash
COIN_MARKET_CAP_ID=1 make wasi-exec
```
