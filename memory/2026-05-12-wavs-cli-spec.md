# `wavs` CLI surface — design spec

> Status: spec only, no implementation. Companion to the file-minimization audit (`/workspace/memory/2026-05-09-wavs-foundry-template-minimization.md`), the components.json design (`/workspace/memory/2026-05-09-components-json-design.md`), and the v1 workflows schema (`/workspace/memory/2026-05-09-workflows-schema-v1.json` + notes).
> Last updated: 2026-05-12.

## Problem

`/workspace/wavs-foundry-template/deploy/` ships 12 TypeScript files totalling **2,304 LOC** that every WAVS adopter clones and ships in their own repo. The pipeline doesn't materially differ between projects — `forge` doesn't ship a per-project `forge.ts`; `cargo` doesn't ship a per-project `cargo-build.sh`. WAVS shouldn't either.

Today's `deploy/` files:

| File | LOC | Purpose |
|---|---|---|
| `build-service.ts` | 432 | Reads `config/components.json`, produces `.docker/service.json` |
| `config.ts` | 103 | Config loader + validation |
| `constants.ts` | 14 | Magic strings |
| `create-deployer.ts` | 127 | Generates deployer wallet |
| `create-operator.ts` | 165 | Generates POA operator wallets |
| `deploy-contracts.ts` | 160 | Runs `forge script` for solidity contract deploys |
| `deploy-script.ts` | 333 | Top-level orchestrator |
| `env.ts` | 287 | `DevEnv` / `ProdEnv` classes (RPC URLs, chain IDs, IPFS endpoints) |
| `types.ts` | 100 | Shared TS types |
| `upload-components.ts` | 230 | Uploads `.wasm` files to WARG / Pinata |
| `upload-service.ts` | 121 | Uploads `service.json` to IPFS / Pinata |
| `utils.ts` | 232 | Shared helpers |

## Relationship to the existing `wavs-cli`

`/workspace/WAVS/packages/cli/` already builds a binary named `wavs-cli` with subcommands:
- `UploadComponent` — uploads compiled `.wasm` to a WAVS endpoint
- `DeployService` — deploys a service from a service URI
- `Exec` — runs a single component locally
- `ExecAggregator` — runs the aggregator component locally
- `Service` — service.json management

Some of these overlap with `deploy/upload-components.ts` and `deploy/upload-service.ts`. The rest of `deploy/*.ts` (build-service, deploy-contracts, deploy-script, create-deployer, create-operator, env, config) **has no Rust counterpart today**.

**Recommendation**: extend `wavs-cli` rather than create a separate binary. Rename to `wavs` at distribution time (or as the cargo package name) for the user-facing surface. Keep `wavs-cli` as the crate name for backwards-compat.

## Proposed subcommand surface

```
wavs <subcommand> [args]

Lifecycle
  wavs init [template]              Scaffold a new project (Phase C)
  wavs build [component]            Build one or all components (replaces task build:wasi)
  wavs build-service                Read components.json/wavs.workflows.toml, emit service.json
  wavs deploy-full                  Orchestrator: build-service → upload-component(s) → deploy-contracts → upload-service → deploy-service

Component / service
  wavs upload-component <file>      Upload a .wasm to WARG (LOCAL) or registry (TESTNET/PROD). [exists]
  wavs upload-service <json>        Upload service.json to IPFS (LOCAL) or Pinata (TESTNET/PROD)
  wavs deploy-service --uri <url>   Deploy a service to a running WAVS node. [exists]
  wavs deploy-contracts             Run forge scripts to deploy Trigger.sol + Submit.sol

Operators (POA)
  wavs create-deployer              Generate deployer wallet, fund on local
  wavs create-operator [N]          Generate N POA operator wallets, write infra/wavs-N/.env

Inspection
  wavs service get                  Read deployed service config from a WAVS node
  wavs service set-uri              Update on-chain service URI
  wavs operator register / verify   Operator lifecycle (currently in taskfile/poa-operator.yml)
```

## Configuration sources

Two files drive everything:

1. **`wavs.toml`** — WAVS node config (chain endpoints, ports, signing mnemonic source). Already exists; not changing.
2. **`config/components.json`** (today) → **`wavs.workflows.toml`** (target; see v1 schema). Workflows / aggregators / templating.

The CLI reads both. `wavs.toml` describes the *runtime*; `wavs.workflows.toml` describes the *service to deploy*.

## Env modes — replaces `deploy/env.ts`

Today `deploy/env.ts` (287 LOC) defines `DevEnv` and `ProdEnv` classes. CLI absorbs this:

- `WAVS_ENV=dev` (default) → local anvil, chain 31337, local IPFS, local WARG (`http://localhost:8090`)
- `WAVS_ENV=prod` → Sepolia (chain 11155111), Pinata, configured per-chain RPC endpoints

Env selection lives in one place (`wavs.toml [env.<name>]` sections), readable by both the node and CLI. No per-template TS env definitions.

## Templating engine

Must match the formal grammar specified in `/workspace/memory/2026-05-09-workflows-schema-notes.md`:

```
${env:VAR_NAME}                     # from .env / shell
${deploy:contracts.trigger.address} # from .docker/deployment_summary.json (post-deploy state)
${ctx:chain.id}                     # from current env context (chain ID, RPC URL, etc.)
${this.workflow.name}               # self-reference inside a workflow block
```

With optional filters: `${env:FOO | default:"bar"}`, `${deploy:x | as_address}`.

Replaces the ad-hoc `${VAR}`, `${get(json.path)}`, `${getEnv(field)}`, `${array[]}` substitutions in today's `build-service.ts`.

## Phasing

### Phase A — Parity (subcommands that already exist in `wavs-cli`)
- Audit `wavs upload-component` vs `deploy/upload-components.ts` for parity gaps
- Audit `wavs deploy-service` vs `deploy/deploy-script.ts:deployService()` for parity gaps
- Fill gaps; mark these subcommands stable

### Phase B — New orchestration
- Implement `wavs build-service` (port `build-service.ts` logic to Rust, consume both today's `components.json` and the new `wavs.workflows.toml` schema)
- Implement `wavs deploy-full` orchestrator
- Implement `wavs upload-service` (Pinata + local IPFS paths)
- Implement `wavs deploy-contracts` (wraps forge script invocations)
- Implement `wavs create-deployer` / `wavs create-operator`

After Phase B, the template can delete `deploy/*.ts` entirely, drop the `tsx`/typescript deps from `package.json`, and replace the Taskfile `deploy-full` task body with `wavs deploy-full`.

### Phase C — Scaffolding
- Implement `wavs init` that generates a fresh template directory (no clone + customize ritual)
- Distribute via `cargo install wavs` from crates.io once stable

## Test plan

End-to-end equivalence: starting from a clean checkout, `wavs deploy-full` against local anvil + IPFS produces an identical `.docker/deployment_summary.json` shape (modulo wallet addresses) to today's `task deploy-full`.

Specifically:
- Component digests in `service.json` match between TS and Rust pipelines
- Contract deployment addresses are deterministic (use the same deployer key)
- Operator registration succeeds with the same parameters
- The price-oracle workflow triggers and submits identical results

## Coexistence with `@wavs/mcp`

`@wavs/mcp` (at `/workspace/WAVS/packages/wavs-mcp/`) is an MCP server exposing WAVS operations to LLMs. It currently shells out to `wavs-cli` internally. After the CLI extension lands, `@wavs/mcp` should still shell out — but to the expanded subcommand surface. Same binary, more verbs.

No changes needed to `@wavs/mcp` for Phase A; minor wrapper additions for Phase B (new tools: `wavs_build_service`, `wavs_deploy_full`).

## Open questions

1. **TS interop during transition**: Phase B introduces a Rust pipeline alongside the TS one. Do we ship both for a release? Or hard-cut over once parity is established? Recommendation: hard-cut once Phase B passes the equivalence test — fewer code paths to maintain.
2. **`wavs init` template registry**: where do templates live? In `WAVS/` repo? Separate `wavs-templates` repo? GitHub starter templates? Defer to Phase C planning.
3. **Cosmos / SVM**: today's `deploy/` is EVM-only. CLI should be chain-aware. How do we model that without duplicating the EVM logic for each chain? Recommendation: subcommands take a `--chain` flag; the deploy-contracts dispatch routes to forge / wasmd / anchor accordingly. Concrete shape TBD.
4. **`pnpm run deploy:*` script compatibility**: `package.json` exposes `deploy:full`, `deploy:contracts`, etc. as pnpm scripts. After Phase B, do those scripts shell out to `wavs ...`? Or get deleted? Likely delete; users invoke `wavs deploy-full` directly.

## Cross-references

- `/workspace/memory/2026-05-09-wavs-foundry-template-minimization.md` — audit
- `/workspace/memory/2026-05-09-components-json-design.md` — config replacement design
- `/workspace/memory/2026-05-09-workflows-schema-v1.json` — JSON Schema draft
- `/workspace/memory/2026-05-09-workflows-schema-notes.md` — templating grammar spec
- `/workspace/WAVS/packages/cli/` — current CLI to extend
- `/workspace/WAVS/packages/wavs-mcp/` — MCP server (shells out to CLI)
- `/workspace/WAVS/packages/types/src/service.rs` — canonical `Service` type

## Sequencing

Phase A can land in parallel with the proc-macro work. Phase B is multi-week and gates the `deploy/*.ts` deletion. Phase C is post-stabilization.

The file-minimization PR (today) doesn't touch any of this; it just clears the field.
