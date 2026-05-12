# WIT registry publish — pending dev confirmation

> Status: **BLOCKED on coordination with WAVS devs.** Owner: Jake.
> Created: 2026-05-12. Resumable.

## Where we left off

Commit `2bfb230` on `better-template` (wavs-foundry-template) removed vendored `wit/deps/` + `wit-aggregator/deps/` and added a `task wit:fetch` recipe that calls `wkg wit fetch --wit-dir wit` / `wkg wit fetch --wit-dir wit-aggregator`.

Running `task wit:fetch` on host fails with:

```
Error: component registry package `wavs:types` has no release matching version requirement `=2.7.0`
```

**Registry (`wa.dev`) does not carry `wavs:types@2.7.0`.** The template references `@2.7.0` in its world files; nothing matches.

## Why we picked Option B

Upstream version state (as of 2026-05-12):
- `/workspace/WAVS/Cargo.toml`: `version = "2.8.0"`
- `/workspace/WAVS/wit-definitions/**/*.wit`: still declare `@2.7.0`

Someone bumped Cargo to 2.8.0 without running `just set-version v2.8.0` to roll the WIT package decls. Publishing 2.7.0 now (Option A from the conversation) would put a version on the registry that's already historical. Option B rolls everything to 2.8.0 first.

Option C (commit bundle-form deps as a stopgap) stays available if Option B drags.

## Resumption plan

### Step 1 — Confirm with WAVS devs

Before publishing, check:
1. Is anyone mid-flight on a separate `2.8.0` work item? (Bumping the WIT version mid-PR could collide.)
2. Who has `warg` publishing credentials configured for `wavs:*` packages on `wa.dev`? Jake may not be on that ACL.
3. Should we publish to `wa.dev` or a private/staging registry first? (`wkg config --default-registry` setting in the template assumes `wa.dev`.)
4. Is `wasi:tls@0.2.0-draft` already on `wa.dev`? If not, the fetch will still fail on that draft dep even after `wavs:*` is published. (`wkg get wasi:tls@0.2.0-draft` confirms.)

### Step 2 — Roll upstream to 2.8.0

```bash
cd /workspace/WAVS
just set-version v2.8.0        # rewrites @2.7.0 → @2.8.0 across wit-definitions/
cargo build                     # sanity
just wit-build                  # produces wit-definitions/<pkg>/<pkg>.wasm artifacts
just wit-publish                # pushes to default registry
```

Commit the version roll upstream as its own PR. `just set-version` touches:
- `Cargo.toml` workspace version
- Every `.wit` file under `wit-definitions/` (package decl + `use` clauses)

### Step 3 — Update the template

In `/workspace/wavs-foundry-template/`:

1. **`wit/operator.wit`** — line 1 `package wavs:operator@2.7.0;` → `@2.8.0`. Lines 3-6 `use wavs:types/{core,service,chain,events}@2.7.0` → `@2.8.0`.
2. **`wit-aggregator/aggregator.wit`** — line 1 `package wavs:aggregator@2.7.0;` → `@2.8.0`. Lines 3-8 (same four `wavs:types` uses + two `wavs:operator/{input,output}@2.7.0` uses) → `@2.8.0`.

Then:

```bash
task wit:fetch                  # should succeed
task build:wasi                 # smoke-test components rebuild
```

Optionally also bump `wavs-wasi-utils` workspace dep in `Cargo.toml` to whatever the 2.8.0-aligned version is — check `WAVS/Cargo.toml` for the published version.

### Step 4 — Commit the version bump

One commit on `better-template`:

```
template: bump WIT deps to wavs:*@2.8.0

Aligns with WAVS workspace version 2.8.0 (set-version was run upstream).
Registry now carries the matching packages.
```

### Step 5 — Optional Tier 2 follow-up

Once the registry is verified working, revisit Tier 2 (delete `wit/operator.wit` + `wit-aggregator/aggregator.wit`; reference `wavs:operator/wavs-world@2.8.0` directly via `wit_bindgen::generate!`). See `2026-05-12-template-minimization-progress.md` § Deferred.

## Files touched on resume

- `/workspace/WAVS/Cargo.toml` (workspace version)
- `/workspace/WAVS/wit-definitions/**/*.wit` (version rolls, via `just set-version`)
- `/workspace/wavs-foundry-template/wit/operator.wit`
- `/workspace/wavs-foundry-template/wit-aggregator/aggregator.wit`

## CI/automation followup

The `just wit-publish` target exists upstream but isn't wired to CI. To prevent this exact blocker from recurring after the next version bump, add a step to the WAVS release workflow that runs `just wit-publish` on tag push. Captured separately so it's not lost — track in upstream WAVS, not in the template.

## Fallback if devs say "don't publish yet"

Execute Option C: revert `2bfb230`, instead commit bundle-form deps copied from `/workspace/WAVS/wit-definitions/<pkg>/wit/deps/<dep>/package.wit`. Each dep becomes one file (~17 total in tree, vs the 28 source-form files we removed). Build works offline. Drift bounded to per-dep diffs on re-sync.

Steps if going to C:
```bash
cd /workspace/wavs-foundry-template
git revert 2bfb230               # undoes the gitignore + wit:fetch task
# then for each dep, copy bundle-form:
mkdir -p wit/deps wit-aggregator/deps
# wit/ needs wavs:types + 8 wasi-* + wasi-tls
# wit-aggregator/ needs wavs:types + wavs:operator + 8 wasi-* + wasi-tls
for dep in $(ls /workspace/WAVS/wit-definitions/operator/wit/deps); do
  mkdir -p "wit/deps/$dep" "wit-aggregator/deps/$dep"
  cp "/workspace/WAVS/wit-definitions/operator/wit/deps/$dep/package.wit" "wit/deps/$dep/"
  cp "/workspace/WAVS/wit-definitions/aggregator/wit/deps/$dep/package.wit" "wit-aggregator/deps/$dep/" 2>/dev/null || true
done
# wit-aggregator also needs wavs:operator
cp -r /workspace/WAVS/wit-definitions/aggregator/wit/deps/wavs-operator-* wit-aggregator/deps/
```

## Cross-references

- `2026-05-12-template-minimization-progress.md` — overall progress log; Open Question §4 references this blocker
- `/workspace/WAVS/justfile:289-292` — `wit-publish` target
- `/workspace/WAVS/justfile:294-322` — `set-version` target
- `wkg` docs: `wkg wit fetch --wit-dir <path>` requires the **parent** of the wit dir as cwd, not the wit dir itself (lesson from the first attempt)
