# wavs-foundry-template minimization — progress log

> Living document. Tracks what's shipped, what's deferred, and corrections to the 2026-05-09 audit.
> Last updated: 2026-05-12.

## Shipped (this PR — branch `better-template`)

| Change | Commit | Net |
|---|---|---|
| Deleted `Makefile`, `PLAN.md`, `old-wavs-skill-claude.md` | 74279c1 | -1,161 LOC, +3 LOC (CI fix) |
| Merged `README_SETUP.md` into `README.md` under a "Prerequisites" section | ebb494b | -1 file |
| Consolidated 4 `.solhint.json` files into one with `overrides` (now also covers `src/script/**`) | ba4329c | -3 files |
| Collapsed `taskfile/` from 6 files to 2 (inlined build.yml + services.yml; merged config.yml into env.yml preserving `config:*` namespace) | 36a980e | -3 files |
| Staged design memos for `#[wavs_component]` proc-macro and `wavs` CLI | (this commit) | +3 memos |

**Net file delta**: ~10 fewer files at the repo root; ~1,170 fewer LOC. Public task surface preserved (`task build:forge`, `task build:wasi`, `task config:funded-key`, `task config:service-manager-address`, etc.).

## Audit corrections (2026-05-09 memo was wrong on these)

1. **`bindings.rs` is NOT 20k+ lines of generated WIT bindings.** The actual file is 8 lines of `wit_bindgen::generate!{ world, path, ... }`. The macro emits bindings at compile time, not into the file. Gitignoring would be incorrect and unnecessary. The audit's "highest-urgency fix" was based on a misread.
2. **`infra/wavs-1/wavs.toml` is NOT checked-in duplicate config.** `.gitignore:20` has `infra/*`, so the duplication is a working-tree artifact (generated on first run). The audit's "pick one location" recommendation doesn't apply.
3. **`test_utils/` doesn't exist** in this template. The `make validate-component` recipe in the deleted Makefile pointed at a missing script. Removed both the recipe and the CLAUDE.md reference.

## Deferred (intentionally not in this PR)

### Architectural — need separate PR(s)
- `#[wavs_component]` proc-macro implementation → see `2026-05-12-component-proc-macro-spec.md`
- `wavs` CLI binary (Phases A/B/C) → see `2026-05-12-wavs-cli-spec.md`
- `wavs.workflows.toml` rollout → already specced in `/workspace/memory/2026-05-09-components-json-design.md` + `/workspace/memory/2026-05-09-workflows-schema-v1.json`; needs implementation PR after CLI Phase B
- Removing `deploy/*.ts` (12 files, 2,304 LOC) → depends on CLI Phase B
- Removing per-component `Makefile` + `trigger.rs` + `solidity.rs` → depends on proc-macro
- Removing `components/aggregator/` from template → depends on the framework supplying it as a default workflow (unclear status; needs upstream decision)
- Publishing `wit/` + `wit-aggregator/` via `wkg` registry → depends on registry workflow stabilization (Lay3rLabs/wavs-taskfiles already publishes via `wkg`; verify channel)

### Low priority — defer
- Move `telemetry/` to a docs recipe → already opt-in (commented-out reference in services taskfile), low pressure
- Consolidate `src/script/` and `script/` into one location → cosmetic, not blocking anything
- Move `script/Common.s.sol` into `src/script/Common.s.sol` (same as above)

## Out-of-scope upstream changes

These would compound the gains but require coordination with `/workspace/WAVS/`:

- A `wavs-macros` crate adjacent to `wavs-wasi-utils` (proc-macro home)
- Subcommand additions to `wavs-cli` (the orchestration verbs)
- A canonical `Service`-type validator exposed for CLI use (already exists in `wavs_types::Service`; needs lighter wrapper for `wavs build-service`)

## Open questions that emerged from execution

1. **YAML/task linting in CI**: today there's no `task --dry-run` or equivalent step in CI. After collapsing `taskfile/`, a typo wouldn't be caught until a user ran the command. Worth adding a `task --list-all` smoke test step to `.github/workflows/contracts.yml`?
2. **`pnpm run deploy:create-aggregator`**: `package.json` references `deploy/create-aggregator.ts` which was deleted in commit 4512cf7 ("Clean up?"). That script entry is dead. Worth fixing in a follow-up commit.
3. **Solhint glob coverage**: the consolidated `.solhint.json` uses `src/script/**/*.sol` and `script/**/*.sol`. Verify with `pnpm lint:check` locally that file path matching works as intended.

## Cross-references

- `/workspace/memory/2026-05-09-wavs-foundry-template-minimization.md` — original audit
- `/workspace/memory/2026-05-09-components-json-design.md` — workflows.toml v1 design
- `/workspace/memory/2026-05-09-workflows-schema-v1.json` — JSON Schema draft
- `/workspace/memory/2026-05-09-workflows-schema-notes.md` — templating grammar notes
- `/workspace/memory/wavs-component-patterns.md` — 14 security/design rules
- `./2026-05-12-component-proc-macro-spec.md` — proc-macro contract
- `./2026-05-12-wavs-cli-spec.md` — CLI surface

## What "done" looks like

Adopting the template should put a new user at the audit's first-principles 7-file minimum:

```
my-wavs-service/
├── component/
│   ├── Cargo.toml
│   └── src/lib.rs          # #[wavs_component] handler
├── contracts/
│   ├── Trigger.sol
│   └── Submit.sol
├── wavs.workflows.toml     # workflow definitions
├── wavs.toml               # node config (could ship as wavs init default)
└── README.md
```

We're not there yet. This PR clears cruft and stages the contracts for the next two PRs. The 7-file target requires the proc-macro and CLI to land.
