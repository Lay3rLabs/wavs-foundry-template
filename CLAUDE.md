# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

A monorepo template for building **WAVS** (WebAssembly-based Actively Validated Services) applications. The stack combines:
- **Rust WASM components** (compiled to WASI) that run off-chain as AVS operators
- **Solidity contracts** (Foundry) for on-chain triggers and submission
- **TypeScript deployment scripts** that orchestrate the full pipeline

The sample service is a price oracle: a `SimpleTrigger` contract emits events, the WAVS node runs the Rust WASM component to fetch prices, and results are posted on-chain via `SimpleSubmit`.

## Commands

### Install & Setup
```bash
task setup           # Install pnpm + forge dependencies
cp .env.example .env # First-time setup
```

### Build
```bash
task build:forge     # Build Solidity contracts (forge build)
task build:wasi      # Build all WASI components

# Build a single component
WASI_BUILD_DIR=components/evm-price-oracle task build:wasi
```

### Test
```bash
forge test           # Run all Solidity tests
pnpm test            # Same via pnpm
pnpm test:unit       # Only unit tests (--match-contract Unit)
pnpm test:integration # Only integration tests (--match-contract Integration)

# Test a WASM component locally
INPUT_DATA="1" COMPONENT_FILENAME=evm_price_oracle.wasm task wasi:exec
```

### Lint & Format
```bash
pnpm lint:check      # Check Solidity linting + forge fmt
pnpm lint:fix        # Auto-fix linting issues
forge fmt            # Format Solidity only
cargo fmt            # Format Rust only
```

### Local Development (Full Stack)
```bash
# Terminal 1 — start anvil + IPFS + WARG registry (keep running)
task start-all-local

# Terminal 2 — deploy everything (POA contracts, WASM upload, service registration)
task deploy-full
```

### Validate a Component Before Building
```bash
make validate-component COMPONENT=your-component-name
```

## Architecture

### Data Flow
1. User calls `SimpleTrigger.addTrigger(data)` → emits `NewTrigger(bytes)` event
2. WAVS operator (Docker) detects the event, runs the WASM component
3. WASM component fetches external data, encodes result as ABI bytes
4. Aggregator collects operator signatures, calls `SimpleSubmit.handleSignedEnvelope()`
5. Result is stored on-chain, queryable via `SimpleSubmit.getData(triggerId)`

### Key Directories
- `src/contracts/` — `SimpleTrigger.sol` (emit triggers), `SimpleSubmit.sol` (receive results)
- `src/interfaces/ITypes.sol` — shared Solidity types (`TriggerInfo`, `DataWithId`) imported by both Solidity and Rust via `sol!` macro
- `components/evm-price-oracle/` — reference Rust WASM component (copy this as a starting point)
- `components/aggregator/` — aggregator WASM component (rarely modified)
- `config/components.json` — maps WASM filenames to trigger/submit contracts and config; controls what `deploy-full` deploys
- `deploy/` — TypeScript scripts (`tsx`) for the deployment pipeline
- `taskfile/` — task definitions included by `Taskfile.yml`
- `.docker/` — runtime output: `deployment_summary.json`, `service.json`, compiled component digests
- `.nodes/` — generated operator/aggregator infra configs

### Environment Modes
- **`dev`** (default): local anvil (chain `evm:31337`), local IPFS, local WARG registry (`http://localhost:8090`)
- **`prod`**: Sepolia (`evm:11155111`), Pinata IPFS — set `DEPLOY_ENV=prod` in `.env`

`DEPLOY_ENV` drives `deploy/env.ts` which selects `DevEnv` or `ProdEnv` with appropriate RPC URLs, IPFS endpoints, and chain IDs.

### Service Configuration (`config/components.json`)
This file defines which WASM components to deploy and how:
- `trigger.event.contract_json_path` — reads the trigger contract address from `.docker/deployment_summary.json`
- `submit.contract_json_path` — same for the submit contract
- `config.values` supports `${VAR_NAME}` (env var), `${get(json.path)}` (deployment summary), `${getEnv(field)}` (deploy env fields)
- `env_variables` — secrets passed as `WAVS_ENV_*` prefixed vars (must be prefixed `WAVS_ENV_` in `.env`)

## Important Files

| File | Purpose |
|---|---|
| `wavs.toml` | WAVS node config: chain endpoints, ports (WAVS=8041, aggregator=8040), signing mnemonic |
| `config/components.json` | Defines which components to deploy and their trigger/submit bindings |
| `.docker/deployment_summary.json` | Generated after deploy; contains all contract addresses and service ID |
| `src/interfaces/ITypes.sol` | Shared types imported by both Solidity and Rust (`sol!` macro in `trigger.rs`) |
| `deploy/env.ts` | `DevEnv`/`ProdEnv` classes; controls RPC URLs, IPFS, chain IDs per environment |
| `components/evm-price-oracle/` | Reference component — copy this when creating new components |
