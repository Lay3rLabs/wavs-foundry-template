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
| Staged design memos for `#[wavs_component]` proc-macro and `wavs` CLI | 4c7759f | +3 memos |
| Dropped dead refs: `deploy:create-aggregator` script, README Go/JS language links, `metadata.json`, `commitlint` + `lint-staged` deps (no husky to run them) | 1de16f1 | -1 file, -823 LOC lockfile, -3 npm deps |
| Stopped vendoring `wit/deps/` + `wit-aggregator/deps/`; added `task wit:fetch` (wkg-driven) wired into `task setup` | (this commit) | -28 .wit files (~316 KB), +1 task recipe |

**Net file delta**: ~39 fewer files in tree; ~2,000 fewer LOC + 316 KB of vendored WIT. Public task surface preserved (`task build:forge`, `task build:wasi`, `task config:funded-key`, `task config:service-manager-address`, etc.).

## Audit corrections (2026-05-09 memo was wrong on these)

1. **`bindings.rs` is NOT 20k+ lines of generated WIT bindings.** The actual file is 8 lines of `wit_bindgen::generate!{ world, path, ... }`. The macro emits bindings at compile time, not into the file. Gitignoring would be incorrect and unnecessary. The audit's "highest-urgency fix" was based on a misread.
2. **`infra/wavs-1/wavs.toml` is NOT checked-in duplicate config.** `.gitignore:20` has `infra/*`, so the duplication is a working-tree artifact (generated on first run). The audit's "pick one location" recommendation doesn't apply.
3. **`test_utils/` doesn't exist** in this template. The `make validate-component` recipe in the deleted Makefile pointed at a missing script. Removed both the recipe and the CLAUDE.md reference.
4. **Template's vendored `wit/deps/` was in *source* form, not *bundle* form.** Upstream WAVS commits its deps as single `package.wit` files per dep (the format `wkg wit fetch` materializes). The template instead vendored the **multi-file source layout** from upstream's `wit-definitions/<pkg>/wit/` directories (e.g. `wavs-types-2.7.0/{chain,core,events,lib,service}.wit`). That's why drift accumulated — upstream evolved the source files while the template's frozen copy didn't. `wkg wit fetch` produces the bundle form, which is more stable and matches upstream's `deps/` convention.

## Deferred (intentionally not in this PR)

### Architectural — need separate PR(s)
- `#[wavs_component]` proc-macro implementation → see `2026-05-12-component-proc-macro-spec.md`
- `wavs` CLI binary (Phases A/B/C) → see `2026-05-12-wavs-cli-spec.md`
- `wavs.workflows.toml` rollout → already specced in `/workspace/memory/2026-05-09-components-json-design.md` + `/workspace/memory/2026-05-09-workflows-schema-v1.json`; needs implementation PR after CLI Phase B
- Removing `deploy/*.ts` (12 files, 2,304 LOC) → depends on CLI Phase B
- Removing per-component `Makefile` + `trigger.rs` + `solidity.rs` → depends on proc-macro
- Removing `components/aggregator/` from template → depends on the framework supplying it as a default workflow (unclear status; needs upstream decision)
- **WIT Tier 2** (delete `wit/operator.wit` + `wit-aggregator/aggregator.wit`; reference `wavs:operator/wavs-world@2.7.0` directly) → depends on:
  - verifying `wavs:types`, `wavs:operator`, `wavs:aggregator` are actually published to `wa.dev` (run `wkg get wavs:operator@2.7.0` against the registry)
  - upstream CI publishing on `vX.Y.Z` tag push (the `just wit-publish` target exists in `WAVS/justfile:289-292` but isn't wired to CI yet)
  - confirming `wasi:tls@0.2.0-draft` resolves via the registry (draft packages historically had spotty coverage)

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
2. **Solhint glob coverage**: the consolidated `.solhint.json` uses `src/script/**/*.sol` and `script/**/*.sol`. Verify with `pnpm lint:check` locally that file path matching works as intended.
3. **`metadata.json` external consumer (now deleted)**: no in-repo consumer was found, so it was removed. If a template directory or registry outside this repo was consuming it, restore from `git show HEAD^^:metadata.json`.
4. **`task wit:fetch` blocked on registry — `wavs:types@2.7.0` not published.** Confirmed on host 2026-05-12: `wkg wit fetch --wit-dir wit` fails with `no release matching version requirement =2.7.0`. Resolution path documented in `2026-05-12-wit-publish-blocker.md` — Option B (roll upstream to 2.8.0 + publish + bump template's world files). Blocked on coordination with WAVS devs (publish creds + version-bump timing). Option C fallback (commit bundle-form deps from `/workspace/WAVS/wit-definitions/`) available if the publish path drags.

## Cross-references

- `/workspace/memory/2026-05-09-wavs-foundry-template-minimization.md` — original audit
- `/workspace/memory/2026-05-09-components-json-design.md` — workflows.toml v1 design
- `/workspace/memory/2026-05-09-workflows-schema-v1.json` — JSON Schema draft
- `/workspace/memory/2026-05-09-workflows-schema-notes.md` — templating grammar notes
- `/workspace/memory/wavs-component-patterns.md` — 14 security/design rules
- `./2026-05-12-component-proc-macro-spec.md` — proc-macro contract
- `./2026-05-12-wavs-cli-spec.md` — CLI surface
- `./2026-05-12-wit-publish-blocker.md` — WIT registry publish: pending dev confirmation, full resume plan

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
