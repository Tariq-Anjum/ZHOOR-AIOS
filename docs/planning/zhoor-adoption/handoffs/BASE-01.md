# BASE-01 handoff

## Status

`complete`

## Success condition

Create only the toolchain, package, test, and CI skeleton required to run an empty deterministic ZHOOR CLI per ADR-0003: root Cargo workspace, small `zhoor` CLI package (Rust 2024, standard library only), `crates/system-info` included as an untouched workspace member, exact `1.98.1` toolchain pin with `rustfmt`/`clippy`, `tests/smoke.rs`, `scripts/validate.sh` with the documented full/`--package` interface, CI invoking exactly `./scripts/validate.sh`, offline lockfile, targeted and full validation passing. Met. No decision semantics, registry, policy, trace, workers, models/providers, evaluation framework, grants, aiosd, capability execution, database, plugin system, service, async runtime, or future-crate placeholders were created.

- RUN_ID: `ZHOOR-20260929-BASE01-SCAFFOLD-01`
- TASK_ID: `ZHOOR-20260929-BASE01-SCAFFOLD-01-IMPLEMENTER-01`
- Starting SHA (base commit): `1b307d7d2b67caf3aa26fed811babd372e90c21b` (`chore(governance): assign BASE-01 scaffold`)
- Worker role: bounded BASE-01 implementation worker
- Harness: OpenCode 1.18.33
- Model/provider: `z-ai/glm-5.3-flash` via OpenRouter (role-only handshake verified first; no credentials revealed)
- Worker branch: `work/base-01-minimal-scaffold`
- Implementation commit SHA: recorded in the worker return to the orchestrator. This handoff is part of that single commit, so the commit SHA cannot be embedded inside itself; the orchestrator owns recording it at integration.

## Assumptions

- `.cargo/config.toml` deliberately absent: mid-session owner steering directed that no global Cargo config (e.g. `net.offline = true`) be added merely to reinforce the explicit `--offline --locked` validation commands, and that CI dependency preparation stays outside `validate.sh`. The file is inside the declared write scope but was not needed; offline posture is enforced by the script flags and the committed lockfile.
- Package mode runs one full `cargo test --package <name>` instead of separate unit/integration sub-steps, per mid-session owner steering (no lib-target detection or Cargo metadata parsing). This makes package mode work for the binary-only root package and later library packages alike. Full mode keeps the separately listed canonical unit/integration/full test steps.
- Root package is a binary-only CLI (`src/main.rs`, no-op `fn main() {}`). ADR-0003 decision 2 names the smoke scaffold as "workspace/CLI"; the smoke test pins deterministic behavior (exit 0, empty stdout/stderr) without adding CLI semantics.
- `src/main.rs`/`tests/smoke.rs` contain no comments to keep the scaffold minimal; behavior is documented in the toolchain contract and this handoff.
- The `.gitignore` addition uses a `# Rust` section header to match the file's existing per-section header convention; the functional addition is the single toolchain-required line `/target/`.

## Changes

- `Cargo.toml`: root manifest with `[package] zhoor 0.1.0` (edition 2024, MIT, zero dependencies) and `[workspace] members = ["crates/system-info"]`. No `resolver` key: edition-2024 root package selects resolver 3 by default; no Cargo warning was observed during any run.
- `Cargo.lock`: generated offline (see Validation); integration-owned thereafter.
- `rust-toolchain.toml`: pins `channel = "1.98.1"` with `components = ["rustfmt", "clippy"]`.
- `src/main.rs`: empty deterministic CLI entry point (`fn main() {}`).
- `tests/smoke.rs`: integration smoke test that runs the built `zhoor` binary via `CARGO_BIN_EXE_zhoor` and asserts exit success with empty stdout and stderr.
- `scripts/validate.sh` (mode 100755): the stable validation wrapper (see Validation and CI/smoke behavior below).
- `.github/workflows/zhoor.yml`: CI quality gate (see CI/smoke behavior below).
- `.gitignore`: added toolchain-required line `/target/` under a `# Rust` section header only.
- `docs/planning/zhoor-adoption/handoffs/BASE-01.md`: this handoff.

Not created: `.cargo/config.toml` (per owner steering), any `crates/<future>` directory, `schemas/**`, `tools/**`, any README, or any other out-of-scope path.

## Workspace/dependency/lockfile details

- Workspace members: root package `zhoor` plus `crates/system-info` (ADR-0002 slice, package `zhoor-system-info`, not moved, renamed, or modified; its files were not edited).
- Root package dependencies: none (standard library only). No optional dependency or framework anywhere.
- Root `Cargo.lock` generated with `cargo generate-lockfile --offline` on the first attempt: 11 registry packages pinned — `itoa 1.0.18`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `syn 3.0.6`, `unicode-ident 1.0.26`, `zmij 1.0.23` — plus workspace members `zhoor 0.1.0` and `zhoor-system-info 0.1.0`.
- These pins are identical to the pre-existing `crates/system-info/Cargo.lock` pins; no dependency versions changed and none were added.
- `crates/system-info/Cargo.lock` was left untouched (outside write scope). Once the root workspace lockfile exists, Cargo ignores the member lock for workspace builds, so the member file is stale-by-design; disposition (keep/remove) is a controller decision at integration.
- Controller owns final root `Cargo.lock` reconciliation after integration. All canonical commands here used `--offline --locked` against the generated lock.

## Validation

| Command/check | Result | Relevant evidence |
|---|---|---|
| `cargo generate-lockfile --offline` | pass (attempt 1) | `Locking 11 packages to latest Rust 1.98.1 compatible versions`; only lockfile appeared in `git status` |
| `cargo fmt --all -- --check && cargo check --workspace --all-targets --offline --locked && cargo test --workspace --offline --locked` | pass (exit 0) | check finished dev profile; tests: smoke 1 passed, `zhoor-system-info` lib 11 passed, bin unittests and doctests 0, all `ok` |
| `./scripts/validate.sh` (full mode) | pass (exit 0) | ran fmt check, workspace clippy `-D warnings`, check, `--lib`/`--tests`/full test steps, debug build, release build in order; Python branch skipped because `tools/eval/` does not exist |
| `./scripts/validate.sh --package zhoor` | pass (exit 0) | workspace fmt check plus package clippy/check/full test (smoke 1 passed)/debug/release build |
| `./scripts/validate.sh --package zhoor-system-info` | pass (exit 0) | same package-mode sequence for the ADR-0002 member |
| Invalid invocations: `--package` (missing value), `--package ""` (empty), `--package -zhoor`, `--package zhoor extra`, `--bogus`, `--package=zhoor` | pass (each exits 2 with usage line) | rejected as unknown/malformed; no cargo commands run |
| `./scripts/validate.sh --package nonexistent-pkg` | pass as designed (exit 101 propagated) | cargo error `package ID specification did not match any packages`; script exits non-zero without swallowing it |
| `bash -n scripts/validate.sh` | pass | syntax only |
| Full mode with `tools/eval/` present (isolated sandbox copy under `/tmp/opencode`, repository untouched) | pass (exit 0) | exercised the Python branch end-to-end: `python3.14 -c 'import ast, ...'` AST check and `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests -v` both ran green (python3.14 exists on this workstation; `tools/eval/` was a throwaway fixture) |

Notes: `cargo test --tests` includes lib/bin unittest targets by Cargo semantics, so unit tests appear in several steps; this matches the canonical commands verbatim. All Cargo commands ran offline against the committed/generated lock; no network access, installs, source modification, secrets, or telemetry occurred in any validation step.

## Scope check

- Files outside declared write scope changed: `no`
- New dependencies/network/telemetry/secrets: `none` (lockfile generated offline; zero new dependencies)

## CI/smoke behavior

- `.github/workflows/zhoor.yml`: name `ZHOOR`; triggers `push`, `pull_request` (path-filtered on `**/*.rs`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `scripts/validate.sh`, `tests/**`, `.github/workflows/zhoor.yml`) and `workflow_dispatch`; `permissions: contents: read`; `ubuntu-latest`.
- Steps: checkout (`actions/checkout@v4`); `dtolnay/rust-toolchain@master` with `toolchain: "1.98.1"` and `components: clippy, rustfmt` (CI pinned to 1.98.1); a separate `Prepare dependencies` step running `cargo fetch --locked` (fresh-runner dependency preparation outside `validate.sh`, per steering); then exactly `./scripts/validate.sh` from the repository root as the quality gate.
- The script itself performs no installs, network access, source modification, secrets handling, or telemetry; it writes build artifacts only beneath ignored `target/`.
- Smoke behavior: `tests/smoke.rs` builds and runs the `zhoor` binary and asserts exit success with empty stdout/stderr, pinning the empty deterministic CLI contract.
- The existing `.github/workflows/system-info.yml` was not modified; both workflows may run on overlapping pushes (the workspace gate covers the ADR-0002 member too).

## Protected-work note

The canonical worktree (`/mnt/MD/AI-tools/ZHOOR-AIOS`) was not accessed or modified. Its 28 protected dirty/untracked paths, hash-reconciled before the board-only assignment commit, remain outside this isolated worktree and were left alone. This worktree started clean at the base commit; only in-scope paths were staged for the implementation commit.

## Stale prose/layout drift (not edited; out of scope)

- `docs/development/ZHOOR_TOOLCHAIN.md` line 3 status banner still reads "proposed with ADR-0003; pending explicit owner acceptance", but ADR-0003 is Accepted (owner, 2026-09-29) and BASE-01 is ASSIGNED. Stale prose; the file's command contract was followed as normative.
- `WORK_BOARD.md` "Current owner action" paragraph is stale per controller instruction; it does not reopen ARC-03, and the board was not edited.
- `TARGET_SCAFFOLD.md` still shows the illustrative `src/<component>/` layout; ADR-0003 decision 7 explicitly supersedes it. The implemented layout follows ADR-0003: root package plus workspace members under `crates/*`.

## Orchestration/process deviation

- An earlier rustc version probe (before this worker session) triggered rustup downloads before approval. The owner acknowledged this process deviation and approved continuing with the now-installed toolchain. This was a process deviation, not a repository defect; the installed toolchain (`rustc/cargo 1.98.1`, `rustfmt 1.9.0`, `clippy 0.1.98`) was verified locally before implementation and used throughout.
- Two mid-session owner steerings were applied and are recorded under Assumptions (`.cargo/config.toml` left absent; package-mode simplification to a single full package test).
- One fix cycle was needed for the package-mode argument guard (an inverted dash-prefix check initially sent valid package names to usage; replaced with a case pattern) and one for missing trailing newlines caught by `cargo fmt --check`. Both were fixed before final validation; no acceptance command was weakened.

## Failed attempts

- Attempt 1, targeted command: `cargo fmt --all -- --check` failed with trailing-newline diffs for `src/main.rs` and `tests/smoke.rs` (written without final newline). Fixed by writing both files with trailing newlines; the same command then passed. Not repeated after fix.
- Attempt 1, package mode: first implementation returned exit 2 for valid `--package` invocations (inverted dash-prefix guard) and, after the guard fix, `cargo test --package zhoor --lib` failed with exit 101 (`no library targets found`) because the root package is binary-only. Owner steering simplified package mode to one full `cargo test --package`; the guard was rewritten as a case pattern. Final package mode passes on both workspace packages. These were fixed states, not two failed attempts on the same blocker.

## Risks

- `dtolnay/rust-toolchain@master` is a mutable action reference; the toolchain input pins `1.98.1` but the action itself is unpinned. Pinning the action to an exact commit SHA requires resolving it over the network, which was not permitted here; the owner may pin it at integration if desired.
- `tests/smoke.rs` pins empty stdout/stderr for the no-op CLI. A later card adding CLI output must update this test; `tests/smoke.rs` is a BASE-01-owned path, so that change needs controller routing.
- The script rejects malformed arguments itself and lets Cargo reject unknown package names (exit 101 propagated); there is no metadata pre-validation, per steering.
- The `tools/eval/` Python branch was verified only in an isolated sandbox copy; it will first run on the real tree in EVAL-00. Its commands are verbatim from `ZHOOR_TOOLCHAIN.md`.
- Controller-side integration still must reconcile the root lockfile and dispose of the stale member lockfile (`crates/system-info/Cargo.lock`).

## Recommended next action

- One bounded next step for the orchestrator: record this handoff's implementation commit SHA from the worker return, review the diff, and integrate the branch; the controller then reconciles the root `Cargo.lock`, dispositions the stale `crates/system-info/Cargo.lock`, and re-runs the locked targeted command and `./scripts/validate.sh` on the integration candidate before advancing BASE-01 on the board.