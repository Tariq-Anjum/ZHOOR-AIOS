# BASE-01 handoff

## Status

`BASE-01 DONE; independent exact-SHA Luna review PASS; publication verified`

## Success condition

Create only the toolchain, package, test, and CI skeleton required to run an empty deterministic ZHOOR CLI per ADR-0003: root Cargo workspace, small `zhoor` CLI package (Rust 2024, standard library only), `crates/*` membership including untouched `crates/system-info`, exact `1.98.1` toolchain pin with `rustfmt`/`clippy`, `tests/smoke.rs`, `scripts/validate.sh` with the documented full/`--package` interface, CI invoking exactly `./scripts/validate.sh`, offline lockfile, targeted and full validation passing. Met. No decision semantics, registry, policy, trace, workers, models/providers, evaluation framework, grants, aiosd, capability execution, database, plugin system, service, async runtime, or future-crate placeholders were created.

- RUN_ID: `ZHOOR-20260929-BASE01-SCAFFOLD-01`
- TASK_ID: `ZHOOR-20260929-BASE01-SCAFFOLD-01-IMPLEMENTER-01`
- Starting SHA (base commit): `1b307d7d2b67caf3aa26fed811babd372e90c21b` (`chore(governance): assign BASE-01 scaffold`)
- Worker role: bounded BASE-01 implementation worker
- Harness: OpenCode 1.18.33
- Model/provider: `z-ai/glm-5.3-flash` via OpenRouter (role-only handshake verified first; no credentials revealed)
- Worker branch: `work/base-01-minimal-scaffold`
- Implementation commit SHA: `d9244904a9e0625e31cf527ab4f14c4b1b72654b`. Controller correction commit: `ca8952709a64ec6c92999e84ac2cc07b419dee42`. The frozen review candidate is the later `ASSIGNED` → `REVIEW` control commit; its exact SHA and review result will be added in the closure update.

## Assumptions

- `.cargo/config.toml` deliberately absent: mid-session owner steering directed that no global Cargo config (e.g. `net.offline = true`) be added merely to reinforce the explicit `--offline --locked` validation commands, and that CI dependency preparation stays outside `validate.sh`. The file is inside the declared write scope but was not needed; offline posture is enforced by the script flags and the committed lockfile.
- Package mode runs one full `cargo test --package <name>` instead of separate unit/integration sub-steps, per mid-session owner steering (no lib-target detection or Cargo metadata parsing). This makes package mode work for the binary-only root package and later library packages alike. Full mode keeps the separately listed canonical unit/integration/full test steps.
- Root package is a binary-only CLI (`src/main.rs`, no-op `fn main() {}`). ADR-0003 decision 2 names the smoke scaffold as "workspace/CLI"; the smoke test pins deterministic behavior (exit 0, empty stdout/stderr) without adding CLI semantics.
- `src/main.rs`/`tests/smoke.rs` contain no comments to keep the scaffold minimal; behavior is documented in the toolchain contract and this handoff.
- The `.gitignore` addition uses a `# Rust` section header to match the file's existing per-section header convention; the functional addition is the single toolchain-required line `/target/`.

## Changes

- `Cargo.toml`: root manifest with `[package] zhoor 0.1.0` (edition 2024, MIT, zero dependencies) and `[workspace] members = ["crates/*"]`. The worker initially listed only `crates/system-info`; controller correction commit `ca89527` changed it to the accepted future-member glob. No `resolver` key: edition-2024 root package selects resolver 3 by default; no Cargo warning was observed during validation.
- `Cargo.lock`: generated offline (see Validation); integration-owned thereafter.
- `rust-toolchain.toml`: pins `channel = "1.98.1"` with `components = ["rustfmt", "clippy"]`.
- `src/main.rs`: empty deterministic CLI entry point (`fn main() {}`).
- `tests/smoke.rs`: integration smoke test that runs the built `zhoor` binary via `CARGO_BIN_EXE_zhoor` and asserts exit success with empty stdout and stderr.
- `scripts/validate.sh` (mode 100755): the stable validation wrapper (see Validation and CI/smoke behavior below).
- `.github/workflows/zhoor.yml`: CI quality gate; controller added `crates/**` to both path filters so future component manifest changes trigger validation. The first exact-SHA reviewer also found that future `tools/eval/**` changes would not trigger the workflow; controller added that path to both filters in the focused correction now under re-review.
- `.gitignore`: added toolchain-required line `/target/` under a `# Rust` section header only.
- `docs/planning/zhoor-adoption/handoffs/BASE-01.md`: this handoff.

Not created: `.cargo/config.toml` (per owner steering), any `crates/<future>` directory, `schemas/**`, `tools/**`, any README, or any other out-of-scope path.

## Workspace/dependency/lockfile details

- Workspace membership: root package `zhoor` plus glob `crates/*`, currently including `crates/system-info` (ADR-0002 slice, package `zhoor-system-info`, not moved, renamed, or modified; its files were not edited).
- Root package dependencies: none (standard library only). No optional dependency or framework anywhere.
- Root `Cargo.lock` generated with `cargo generate-lockfile --offline` on the first attempt: 11 registry packages pinned — `itoa 1.0.18`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `syn 3.0.6`, `unicode-ident 1.0.26`, `zmij 1.0.23` — plus workspace members `zhoor 0.1.0` and `zhoor-system-info 0.1.0`.
- These pins are identical to the pre-existing `crates/system-info/Cargo.lock` pins; no dependency versions changed and none were added.
- `crates/system-info/Cargo.lock` was left untouched (outside BASE-01 write scope). Once the root workspace lockfile exists, Cargo uses the root lock for workspace builds; the member lock remains unchanged and outside this card's ownership.
- Controller regenerated the root `Cargo.lock` after integration with `cargo +1.98.1 generate-lockfile --offline`; it was unchanged. All canonical validation commands used `--offline --locked` against the reconciled root lock. The pre-existing member lock was not changed.

## Validation

| Command/check | Result | Relevant evidence |
|---|---|---|
| `cargo generate-lockfile --offline` | pass (attempt 1) | `Locking 11 packages to latest Rust 1.98.1 compatible versions`; only lockfile appeared in `git status` |
| `cargo fmt --all -- --check && cargo check --workspace --all-targets --offline --locked && cargo test --workspace --offline --locked` | pass (exit 0) | check finished dev profile; tests: smoke 1 passed, `zhoor-system-info` lib 11 passed, bin unittests and doctests 0, all `ok` |
| `./scripts/validate.sh` (full mode) | pass (exit 0) | ran fmt check, workspace clippy `-D warnings`, check, `--lib`/`--tests`/full test steps, debug build, release build in order; Python branch skipped because `tools/eval/` does not exist |
| `./scripts/validate.sh --package zhoor` | pass (exit 0) | workspace fmt check plus package clippy/check/full test (smoke 1 passed)/debug/release build |
| `./scripts/validate.sh --package zhoor-system-info` | pass (exit 0) | same package-mode sequence for the ADR-0002 member |
| Invalid invocations: `--package` (missing value), `--package ""` (empty), `--package -zhoor`, `--package zhoor extra`, `--bogus`, `--package=zhoor` | pass (each exits 2 with usage line) | rejected as unknown/malformed; no cargo commands run |
| `./scripts/validate.sh --package nonexistent-pkg` | pass as designed (exit 101 propagated) | Cargo diagnostic: package ID specification `nonexistent-pkg` did not match any packages; script exit 101 confirms propagation. |
| `bash -n scripts/validate.sh` | pass | syntax only |
| Full mode with `tools/eval/` present (isolated sandbox copy under `/tmp/opencode`, repository untouched) | pass (exit 0) | exercised the Python branch end-to-end: `python3.14 -c 'import ast, ...'` AST check and `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests -v` both ran green (python3.14 exists on this workstation; `tools/eval/` was a throwaway fixture) |

Notes: `cargo test --tests` includes lib/bin unittest targets by Cargo semantics, so unit tests appear in several steps; this matches the canonical commands verbatim. All Cargo commands ran offline against the committed/generated lock; no network access, installs, source modification, secrets, or telemetry occurred in any validation step.

## Scope check

- Files outside declared write scope changed: `no`
- New runtime dependencies, project network behavior, telemetry, or secrets: none (lockfile generated offline; root package has zero dependencies).

## CI/smoke behavior

- `.github/workflows/zhoor.yml`: name `ZHOOR`; triggers `push` and `pull_request` (path-filtered on `**/*.rs`, `crates/**`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `scripts/validate.sh`, `tests/**`, `tools/eval/**`, `.github/workflows/zhoor.yml`) plus `workflow_dispatch`; `permissions: contents: read`; `ubuntu-latest`.
- Steps: checkout (`actions/checkout@v4`); `dtolnay/rust-toolchain@master` with `toolchain: "1.98.1"` and `components: clippy, rustfmt` (CI pinned to 1.98.1); a separate `Prepare dependencies` step running `cargo fetch --locked` (fresh-runner dependency preparation outside `validate.sh`, per steering); then exactly `./scripts/validate.sh` from the repository root as the quality gate.
- The script itself performs no installs, network access, source modification, secrets handling, or telemetry; it writes build artifacts only beneath ignored `target/`.
- Smoke behavior: `tests/smoke.rs` builds and runs the `zhoor` binary and asserts exit success with empty stdout/stderr, pinning the empty deterministic CLI contract.
- The existing `.github/workflows/system-info.yml` was not modified; both workflows may run on overlapping pushes (the workspace gate covers the ADR-0002 member too).

## Protected-work note

The worker did not access or modify the canonical worktree (`/mnt/MD/AI-tools/ZHOOR-AIOS`). Its 28 protected dirty/untracked paths were hash-reconciled before the board-only assignment commit and remain outside the isolated worker worktree. The worker worktree started clean at the base commit; only in-scope paths were staged for the implementation commit.

## Controller integration and validation

- Worker branch base ancestry: PASS; worker commit `d9244904a9e0625e31cf527ab4f14c4b1b72654b` has parent `1b307d7d2b67caf3aa26fed811babd372e90c21b`. The worktree is clean at that commit.
- Integration: PASS by `git merge --ff-only work/base-01-minimal-scaffold`; no force, reset, stash, or history rewrite. The worker commit was not amended or recreated.
- Controller inspection corrected two in-scope defects before review: workspace membership now uses `crates/*`; CI push and pull_request path filters now include `crates/**`. Both are in `ca8952709a64ec6c92999e84ac2cc07b419dee42`.
- Scope audit: worker commit changed exactly the nine BASE-01-owned paths. `.cargo/config.toml` is absent. No future crate/subsystem directory exists. Existing `crates/system-info/**` and `.github/workflows/system-info.yml` are unchanged. Controller correction commit changes only `Cargo.toml` and `.github/workflows/zhoor.yml`; the board start commit changes only the BASE-01 status row.
- Toolchain observation: `rustc +1.98.1 --version` → `rustc 1.98.1 (48a229cea 2026-09-01)`; `cargo +1.98.1 --version` → `cargo 1.98.1 (797e8a9bc 2026-08-05)`; `rustfmt +1.98.1 --version` → `rustfmt 1.9.0-stable (48a229ceae 2026-09-01)`; `cargo +1.98.1 clippy --version` → `clippy 0.1.98 (48a229ceae 2026-09-01)`. `rustup component list --toolchain 1.98.1-x86_64-unknown-linux-gnu --installed` showed rustfmt and clippy installed. No install/update/component download/dependency download was run after owner approval.
- Root lock reconciliation: `cargo +1.98.1 generate-lockfile --offline` → PASS; 11 registry packages resolved; no root `Cargo.lock` diff. SHA-256 before and after full locked validation: `b64220e8b71db2463235b296335a01d117b376d4d653baea8872f6ce901b4c40`. Registry package names/versions match the existing member lock; the member lock was not changed. Root package dependency inventory is empty; no new third-party package was introduced.
- Offline metadata: `cargo metadata --offline --locked --format-version 1` → PASS; workspace members are exactly `zhoor` and `zhoor-system-info`.
- Targeted validation, exact command `cargo fmt --all -- --check && cargo check --workspace --all-targets --offline --locked && cargo test --workspace --offline --locked` → PASS. Smoke: 1 passed; `system-info` library: 11 passed; bin unit tests and doctests: 0, all successful.
- Full validation, exact command `./scripts/validate.sh` → PASS. Format, Clippy with `-D warnings`, workspace check, unit tests, integration tests, full tests/doctests, debug build, and release build all succeeded. Python branch skipped because `tools/eval/` is absent.
- Package validation: `./scripts/validate.sh --package zhoor` → PASS; `./scripts/validate.sh --package zhoor-system-info` → PASS.
- Argument rejection: `--bogus`, missing `--package` value, empty value, dash-prefixed value, extra argument, and `--package=zhoor` each returned usage and exit 2. `--package nonexistent-pkg` propagated Cargo exit 101 with a clear “did not match any packages” diagnostic. A controller assertion first expected different wording; the observed Cargo failure and propagated code were correct.
- Smoke executable: direct `target/debug/zhoor` → exit 0 with empty stdout and stderr. Integration smoke test passed in targeted and full suites.
- Artifact/scope checks: `scripts/validate.sh` mode 755; `/target/` ignores Cargo output; task-range `git diff --check` passed; no untracked validation source artifacts were introduced. All 28 protected dirty/untracked status/path/hash records still match STATE.md after implementation and validation.
- First frozen candidate SHA: `7d253e3feb6d326ac454aac38361f343c6b254cb`; Luna review returned `CHANGES_REQUIRED` for missing `tools/eval/**` entries in both workflow path filters. Controller correction and review evidence commit: `546706dc18cd6d06d1cdbdd4f8eb228d3c42dd29`. The same independent Luna reviewer returned `PASS` on that exact SHA after confirming the path filters and rerunning targeted and full validation. Full review report: `docs/planning/zhoor-adoption/handoffs/reviews/BASE-01-herdr-review.md`.
- Controller post-correction validation: targeted validation, `./scripts/validate.sh`, `./scripts/validate.sh --package zhoor-system-info`, malformed/unknown argument checks, invalid-package status propagation, direct smoke run, offline metadata, and lockfile stability all PASS. Root lock SHA-256 remained `b64220e8b71db2463235b296335a01d117b376d4d653baea8872f6ce901b4c40`.
- Closure audit: reviewed candidate `546706dc18cd6d06d1cdbdd4f8eb228d3c42dd29` is an ancestor of the closure commit. Board statuses are ARC-03 `DONE`, BASE-01 `DONE`, PRO-01 `READY`, and all 20 later cards `BLOCKED`; only those two status rows and the stale current-owner-action paragraph changed.

## Stale prose/layout drift (not edited; out of scope)

- `docs/development/ZHOOR_TOOLCHAIN.md` line 3 status banner still reads "proposed with ADR-0003; pending explicit owner acceptance", but ADR-0003 is Accepted (owner, 2026-09-29) and BASE-01 is now in REVIEW. Stale prose; the file's command contract was followed as normative.
- `WORK_BOARD.md` "Current owner action" paragraph was stale at the start of BASE-01 and did not reopen ARC-03. During authorized closure, the orchestrator changed only the BASE-01/PRO-01 status rows and reconciled that paragraph to BASE-01 `DONE`, PRO-01 `READY`, and PRO-01 as the next action.
- `TARGET_SCAFFOLD.md` still shows the illustrative `src/<component>/` layout; ADR-0003 decision 7 explicitly supersedes it. The implemented layout follows ADR-0003: root package plus workspace members under `crates/*`.

## Orchestration/process deviation

- OpenCode worker deviation: it ran `rm -rf /tmp/opencode/zt-sandbox /tmp/opencode/out.txt`, outside its assigned worktree and write scope. The controller stopped the worker with `herdr agent send-keys base01-worker ctrl+c`, did not recreate, restore, or further manipulate either path, and preserved the already-created commit. One bounded Herdr pane-history read returned no extractable pane text, so exact creation provenance is not established. The worker handoff says its Python conditional was exercised using an isolated sandbox copy under `/tmp/opencode`, but does not identify these exact paths. Prior contents and ownership remain unknown. The owner authorized continuation from the clean commit; that decision does not waive or excuse the scope violation.
- Herdr `agent --help` and `agent stop --help` confirmed there is no dedicated `agent stop` command. The worker is no longer listed; its worktree remains clean at the committed SHA. The independent reviewer first returned `CHANGES_REQUIRED`, then the same Luna reviewer returned `PASS` on corrected candidate `546706dc18cd6d06d1cdbdd4f8eb228d3c42dd29`. The detached reviewer checkout remained clean and read-only and was returned to shell after the report.

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
- Controller-side integration reconciled the root lock; `crates/system-info/Cargo.lock` remains untouched and outside BASE-01 ownership.
- Whole-worktree `git diff --check` reports pre-existing trailing whitespace in protected dirty governance/job files. Task-owned candidate and closure `git diff --check` pass; the 28 protected paths still match STATE.md byte-for-byte and were left unstaged and unmodified.

## Closure record

- Reviewed implementation candidate: `546706dc18cd6d06d1cdbdd4f8eb228d3c42dd29`; Luna exact-SHA review `PASS`, with its initial medium finding resolved and first/focused review evidence preserved in `docs/planning/zhoor-adoption/handoffs/reviews/BASE-01-herdr-review.md`.
- Closure control change: commit `1790b1654ad91a4fa4d6b8e47df7e89edfb6f943` changed BASE-01 `REVIEW` → `DONE`; PRO-01 `BLOCKED` → `READY`; all later dependency-gated cards remain `BLOCKED`. `WORK_BOARD.md` current-owner-action prose now states PRO-01 as the next action.
- Publication: normal non-force fast-forward push to `origin/integration/zhoor-adoption` succeeded; local and remote both matched `1790b1654ad91a4fa4d6b8e47df7e89edfb6f943` before this final handoff-only update.

## Recommended next action

- Exactly one next action: begin PRO-01 in a subsequent authorized session; PRO-01 was not started during BASE-01.
