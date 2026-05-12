# `#[wavs_component]` proc-macro — design contract

> Status: spec only, no implementation. Companion to the file-minimization audit (`/workspace/memory/2026-05-09-wavs-foundry-template-minimization.md`) and security-pattern synthesis (`/workspace/memory/wavs-component-patterns.md`).
> Last updated: 2026-05-12.

## Problem

Every WAVS component in this template carries the same per-file boilerplate:

```
components/<name>/
├── Cargo.toml          # 8 dep entries, mostly identical
├── Makefile            # 12 lines, `cargo component build && cp` (identical across components)
└── src/
    ├── lib.rs          # business logic + Guest impl + trigger decode/encode glue (~50 LOC boilerplate)
    ├── bindings.rs     # 8-line `wit_bindgen::generate!{...}` wrapper (HAND-WRITTEN, not generated)
    ├── trigger.rs      # 65 LOC decode_trigger_event/encode_trigger_output/Destination enum
    └── solidity.rs     # `sol!("../../src/interfaces/ITypes.sol")` + addTrigger fn signature
```

A new component author copies `evm-price-oracle/`, replaces `lib.rs` business logic, and ships three near-identical support files alongside. The boilerplate doesn't change shape; only `lib.rs` does.

> **Note on the audit's bindings.rs claim:** the 2026-05-09 audit said `bindings.rs` was 20k+ lines of generated WIT bindings checked in. That was wrong — it's an 8-line caller of `wit_bindgen::generate!`. The macro emits bindings at compile time into the rustc output, not into a file. Still boilerplate, but cheap to keep in tree. Gitignoring it would be incorrect.

## Goal

A single attribute that absorbs `bindings.rs` + `trigger.rs` + `solidity.rs` + the `Guest` impl scaffold, so components shrink to one file with just the handler logic.

## Proposed surface

```rust
// components/evm-price-oracle/src/lib.rs (target shape after macro)
use wavs::prelude::*;

#[wavs_component(
    world = "wavs-world",
    wit = "../../wit",
    types = "../../src/interfaces/ITypes.sol",
)]
async fn handle(trigger: NewTrigger) -> Result<DataWithId, Error> {
    let id: u64 = String::abi_decode(&trigger.data)?.trim().parse()?;
    let price = fetch_price(id).await?;
    Ok(DataWithId {
        triggerId: trigger.triggerId,
        data: serde_json::to_vec(&price)?.into(),
    })
}
```

### Attribute arguments

| Arg | Required | Default | Purpose |
|---|---|---|---|
| `world` | yes | — | WIT world name (e.g. `"wavs-world"`, `"aggregator-world"`) |
| `wit` | yes | — | Path to WIT directory (relative to `Cargo.toml`) |
| `types` | no | — | Path to a `.sol` file consumed by `sol!` macro. If present, expanded types are re-exported via `pub mod solidity` |
| `mode` | no | `"event"` | `"event"` for EVM contract event triggers; `"raw"` for CLI/test triggers; `"any"` to expose both via enum |
| `aggregator` | no | `false` | When `true`, generates an aggregator-shaped Guest impl instead of operator-shaped |

### What the macro expands to

For the `mode = "event"` (default) case the macro emits:

1. `pub mod bindings { wit_bindgen::generate!{ ... } }` — re-exports `Guest`, `TriggerAction`, `WasmResponse`, world types
2. `pub mod solidity { use alloy_sol_macro::sol; sol!(<types path>); }` if `types` arg present
3. A `Destination` enum (`Ethereum`, `CliOutput`) — kept for `wasi-exec` local testing
4. A default `decode_trigger_event` fn that handles both `TriggerData::EvmContractEvent` and `TriggerData::Raw` cases
5. A default `encode_trigger_output` fn that wraps the user handler's return in a `WasmResponse` with `DataWithId` abi-encoded payload
6. `struct Component; export!(Component with_types_in bindings);`
7. An `impl Guest for Component { fn run(action: TriggerAction) -> Result<Vec<WasmResponse>, String> { ... } }` block that:
   - calls `decode_trigger_event(action.data)`
   - dispatches to the user-supplied `handle(...)` async fn via `block_on`
   - routes the output through `encode_trigger_output` for Ethereum destinations or raw `WasmResponse` for CLI

### What the user must still supply

- `Cargo.toml` (the macro can't author this; would need a separate `cargo wavs new` subcommand from the CLI)
- The handler fn signature: `async fn handle(trigger: T) -> Result<R, E>` where `T` is the decoded trigger type and `R` is the response payload type
- Any helper modules/functions for the business logic

## Defense-in-depth requirements (from `wavs-component-patterns.md`)

The macro should emit, **not just permit**, the safety patterns that have already produced internal audit findings:

1. **log.address verification** (rule from `feedback_wavs_component_log_address.md` in `/workspace/memory/`): when `mode = "event"`, the macro must emit code that checks `log.address` against an expected contract address. Path:
   - Default: pull the expected address from a component-scoped config value (e.g. `WAVS_ENV_TRIGGER_CONTRACT`)
   - Escape hatch: `#[wavs_component(verify_log_address = false)]` opt-out, with a `// SAFETY: ...` comment requirement
2. **abi.decode matching** — the user's handler signature is the source of truth for what type the trigger decodes to; the macro generates the decode call using the handler signature's type so there's no drift between "what we decoded" and "what we typed"

## Where the macro lives

New crate at `/workspace/WAVS/packages/wavs-macros/`:

```
WAVS/packages/wavs-macros/
├── Cargo.toml          # proc-macro = true
└── src/
    ├── lib.rs          # #[proc_macro_attribute] entry point
    ├── parse.rs        # syn parsing of attribute args + fn signature
    ├── expand.rs       # quote!-based code generation
    └── tests.rs        # trybuild compile-fail/pass cases
```

Re-export from a new `wavs` umbrella crate or directly from `wavs-wasi-utils`:

- Option A: `wavs-wasi-utils` re-exports the macro (`pub use wavs_macros::wavs_component`). User adds one dep.
- Option B: new `wavs` umbrella crate (`pub use wavs_macros::*; pub use wavs_wasi_utils::*;`). Cleaner imports (`use wavs::prelude::*;`). Adds a layer.

Recommended: **Option A**. One fewer crate, the macro composes with existing wavs-wasi-utils helpers transparently.

## Migration path

Existing component (e.g. `evm-price-oracle`) opts in by:

1. Add `wavs-macros` (or upgrade `wavs-wasi-utils`) dep in `components/<name>/Cargo.toml`
2. Replace `lib.rs` shell with the `#[wavs_component]` attribute + handler signature
3. Delete `trigger.rs`, `solidity.rs`, `bindings.rs`
4. Keep business logic functions (`get_price_feed`, struct defs) as-is

Per-component file count: 6 → 2 (Cargo.toml + lib.rs).

## Test plan

1. **Compile-time tests** (`trybuild`):
   - Valid attribute args produce compiling code
   - Missing required args produce specific error messages
   - Wrong handler signature (sync where async expected, wrong return type) errors clearly
2. **Behavior tests**: convert `components/evm-price-oracle/` to single-file form on a feature branch. Run:
   - `task build:wasi` succeeds
   - `INPUT_DATA="1" COMPONENT_FILENAME=evm_price_oracle.wasm task wasi:exec` produces same JSON output as before
   - `task deploy-full` end-to-end matches pre-macro behavior
3. **Cross-component test**: convert `components/aggregator/` (different `world`, different shape) to validate the `aggregator = true` mode works

## Open questions

1. **Sync vs async handler**: support both, or async-only? Async-only is simpler for the macro but forces all components to depend on `wstd::runtime::block_on`. Recommendation: support both, dispatch on the `async` keyword in the fn signature.
2. **Error type**: handler returns `Result<R, E>` where `E: ToString`? Or force `E = anyhow::Error`? Latter is friendlier; former is more flexible. Recommendation: `E: Into<anyhow::Error>` so users can choose.
3. **Multiple handlers per component**: WAVS workflows can have a single component handle multiple trigger types (e.g. event + cron). Does the macro support `#[wavs_component]` on multiple fns in one lib.rs, each with a different trigger type? Or one component = one trigger? Defer to a follow-up.
4. **Macro vs declarative `export_layer_trigger_world!`**: there's an existing 6-line declarative macro at `/workspace/WAVS/examples/components/_helpers/src/bindings/world.rs`. Does `#[wavs_component]` deprecate it? Or coexist (declarative for the helpers crate; attribute for user components)? Recommendation: keep declarative as the lower-level primitive; attribute-macro generates a call to it.
5. **CliOutput / Raw mode**: do users still need access to `Destination::CliOutput` for local testing, or does the macro handle both transparently? If the macro auto-dispatches, can users override? Lean toward auto-dispatch with no escape hatch — local testing uses `wasi:exec` which sets up the right TriggerData::Raw anyway.

## Cross-references

- `/workspace/memory/2026-05-09-wavs-foundry-template-minimization.md` — the audit that motivates this work
- `/workspace/memory/wavs-component-patterns.md` — 14 security/design rules the macro should bake in
- `/workspace/WAVS/examples/components/_helpers/src/bindings/world.rs` — existing declarative macro
- `/workspace/WAVS/packages/wasi-utils/` — guest-side helper crate (target re-export point)
- `feedback_wavs_component_log_address` and `feedback_signed_envelope_escape_hatch` memories in `/workspace/MEMORY.md`

## Sequencing

Land **after** the file-minimization PR (this PR) and **before** the `wavs` CLI work. Macro implementation is bounded (~1 week with tests); CLI is multi-week. Macro adoption unblocks per-component cleanup without depending on CLI delivery.
