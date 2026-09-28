# ARC-03 toolchain adjudication handoff

## State

- Run: `ZHOOR-20260928-ARC03-ADJUDICATION-01`
- Canonical repository: `/mnt/MD/AI-tools/ZHOOR-AIOS`
- Branch/base: `integration/zhoor-adoption` at `dba0367a7320688a090296e9fda566d5f02b008e`
- Candidate SHA: pending candidate commit; freeze and record the full SHA before reviewer work.
- ARC-03 remains `READY` until explicit owner acceptance. Later cards remain blocked.
- Push: not authorized before owner acceptance; none performed.

## Assumptions

- The accepted ARC-02 run is bounded evidence from a tiny JSONL workload on the target workstation, not a general performance claim.
- ARC-03 may replace only downstream card write-scope/path sections and add exact validation sections; it does not implement their systems or change their board status.
- Worker crates use a root Cargo workspace wildcard and committed locked dependencies. Component workers do not edit root manifests or lockfiles. Isolated worktrees keep Cargo build outputs under each checkout's ignored `target/`.
- JSON Schema files are the canonical wire-contract source. Rust alone implements production contract/domain validation. Python can read schemas/fixtures and prepare or report offline data but cannot become a second validator or production authority.
- Review reports are read-only findings on the frozen candidate SHA; the controller records received reports using the existing review-handoff naming convention.

## Evidence consumed

- Accepted evidence: `docs/planning/zhoor-adoption/evidence/ARC-02/{results.md,methodology.md,environment.md}` and `docs/planning/zhoor-adoption/handoffs/ARC-02.md`.
- The common bounded corpus had 14 valid and 16 invalid requests, including signed-i64 endpoints and overflow/underflow cases; all 84 outputs matched after the independent review correction.
- The local Rust cold-start samples were lower than Python in the reported runs; Python remained convenient for offline evaluation; the Python-parent/Rust-child option added a process boundary and had the slowest observed cold start. Variability and the narrow workload limit conclusions.
- The independent ARC-02 exact-SHA re-review passed at `e0edd78ed8dd6628a01b346c101a7a618e6d686c`.
- Machine/toolchain evidence: 16 GB-class AMD Ryzen 7 7840U workstation; Rust/Cargo 1.98.1; Python 3.14.7. No broad rerun or new spike was performed.
- Accepted architecture read: ADR-0001; ADR-0002; `docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md`; `docs/planning/zhoor-adoption/TARGET_SCAFFOLD.md`.

## Selected architecture

- Primary production stack: Rust 2024, exact `rustc`/Cargo 1.98.1 pin.
- Canonical layout: one root Cargo workspace and small CLI package; owned components live in distinct packages under `crates/`. The existing `crates/system-info` remains the narrow ADR-0002 slice and is not `aiosd`.
- Same production language for protocol validation, registry, risk policy, trace/replay, deterministic workers, contracts, grants, and future enforcement packages. Semantic rules are not duplicated in Python.
- Python 3.14.7 standard library is development-only for offline evaluation, fixture/dataset preparation, benchmark/report work, and utilities. It is absent from the production request path.
- Mixed-language production process boundary: none.
- JSON Schema under `schemas/` is the language-neutral wire-contract authority. Rust contract checks remain the sole production validation authority; derived bindings do not become a second schema source.
- No model, provider, backend, database, daemon framework, hosted dependency, or production subsystem was added.
- Normative commands and the future `scripts/validate.sh` interface are in `docs/development/ZHOOR_TOOLCHAIN.md`. The root wrapper belongs to BASE-01 and is not created here.

## Rejected alternatives

- Python-first production: convenient for tooling, but the bounded local exercise showed higher/noisier startup and RSS; dynamic production validation would need explicit controls. This is supporting evidence, not a guarantee.
- Mixed Rust/Python production: no measured request-path need justified a second runtime and subprocess lifecycle/failure boundary.
- Other languages or frameworks: ARC-02 supplied no requirement or evidence warranting another production stack or framework.

## Files changed by ARC-03

- `docs/architecture/adr/ADR-0003-rust-production-toolchain.md`
- `docs/development/ZHOOR_TOOLCHAIN.md`
- `docs/planning/zhoor-adoption/WORK_BOARD.md` — planned path column only; statuses/dependencies unchanged.
- `docs/planning/zhoor-adoption/jobs/{BASE-01,PRO-01,REG-01,POL-01,TRC-01,SEC-01,WRK-01,WRK-02,EVAL-00,EVAL-01,EVAL-02,EVAL-03,EVAL-04,AUD-01,BENCH-01,BENCH-02,SHADOW-01,GRANT-01,AIOSD-01,AIOSD-02,AUD-02,CANARY-01}.md` — write scope plus path-ownership/validation sections only.
- This handoff.

No source, schema, fixture, manifest, lockfile, CI workflow, runtime dependency, or production scaffold was created.

## Exact commands and results so far

- `git status --short --branch` — branch is `integration/zhoor-adoption`; initial HEAD was `dba0367a7320688a090296e9fda566d5f02b008e`.
- `git fetch origin integration/zhoor-adoption` — PASS; fetched tracking SHA remained `dba0367a7320688a090296e9fda566d5f02b008e`.
- `git worktree list --porcelain` — inspected; no unexpected ARC-03 worktree conflict. The requested reviewer-luna and reviewer-sol directories already existed at the base checkpoint.
- External `STATE.md` protected-state comparison — PASS before edits: all 28 recorded paths/statuses and SHA-256 values matched the live worktree.
- D-018 protected-card snapshots — PASS: current card content is identical to its pre-edit snapshot outside `## Write scope` and `## Path ownership and validation`. Unrelated protected files remain byte-identical. `SEC-01.md` was clean at startup and was separately snapshotted from HEAD before editing.
- Mechanical ownership-root overlap audit — PASS for `REG-01/POL-01/TRC-01/SEC-01`, `WRK-01/WRK-02`, `EVAL-01/EVAL-02`, `EVAL-03/EVAL-04`, and `GRANT-01/AIOSD-01/AIOSD-02`.
- `git diff --check -- docs/planning/zhoor-adoption/WORK_BOARD.md` — PASS.
- `git diff --no-index --check` from each saved pre-edit downstream-card snapshot to its current file — PASS; no task-owned whitespace errors.
- `git diff --no-index --check /dev/null <ADR/toolchain document>` — no whitespace diagnostics (exit 1 is expected because the files differ from `/dev/null`).
- Full `git diff --check` — reports trailing whitespace on pre-existing protected Herdr recommendation lines in downstream cards. These lines are present in the saved pre-edit snapshots; the task-relative check above is clean. They were preserved and not opportunistically edited.
- Rust source/format/lint/test/build commands — not run: the repository has no root Cargo workspace yet; creating it is owned by BASE-01.
- No source tests were added or run.

## Downstream path ownership and commands

| Card | Exclusive writable paths | Targeted validation | Full validation |
|---|---|---|---|
| BASE-01 | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/**`, `.github/workflows/zhoor.yml`, `scripts/validate.sh`, `src/**`, `tests/smoke.rs`, its handoff | `cargo fmt --all -- --check && cargo check --workspace --all-targets --offline --locked && cargo test --workspace --offline --locked` | `./scripts/validate.sh` |
| PRO-01 | `schemas/protocol/**`, `crates/protocol/**`, `fixtures/protocol/**`, its handoff | `./scripts/validate.sh --package zhoor-protocol`; `cargo test --offline --locked -p zhoor-protocol --test protocol_contract` | `./scripts/validate.sh` |
| REG-01 | `schemas/registry/**`, `crates/registry/**`, `fixtures/registry/**`, its handoff | `./scripts/validate.sh --package zhoor-registry`; `cargo test --offline --locked -p zhoor-registry --test registry_contract` | `./scripts/validate.sh` |
| POL-01 | `schemas/risk-policy/**`, `crates/risk-policy/**`, `fixtures/risk-policy/**`, its handoff | `./scripts/validate.sh --package zhoor-risk-policy`; `cargo test --offline --locked -p zhoor-risk-policy --test risk_policy_contract` | `./scripts/validate.sh` |
| TRC-01 | `schemas/trace/**`, `crates/trace/**`, `fixtures/trace/**`, its handoff | `./scripts/validate.sh --package zhoor-trace`; `cargo test --offline --locked -p zhoor-trace --test trace_contract` | `./scripts/validate.sh` |
| SEC-01 | `schemas/aiosd/**`, `crates/aiosd-contracts/**`, `fixtures/aiosd/**`, `docs/contracts/aiosd/**`, its handoff | `./scripts/validate.sh --package zhoor-aiosd-contracts`; `cargo test --offline --locked -p zhoor-aiosd-contracts --test aiosd_contract` | `./scripts/validate.sh` |
| WRK-01 | `crates/skill-router/**`, `fixtures/skill-router/**`, its handoff | `./scripts/validate.sh --package zhoor-skill-router`; `cargo test --offline --locked -p zhoor-skill-router --test skill_router_contract` | `./scripts/validate.sh` |
| WRK-02 | `crates/risk-judge/**`, `fixtures/risk-judge/**`, its handoff | `./scripts/validate.sh --package zhoor-risk-judge`; `cargo test --offline --locked -p zhoor-risk-judge --test risk_judge_contract` | `./scripts/validate.sh` |
| EVAL-00 | `tools/eval/harness/**`, `tools/eval/tests/harness/**`, `docs/evaluations/harness/**`, its handoff | `python3.14 -c 'import ast, pathlib; [ast.parse(p.read_text(), filename=str(p)) for p in pathlib.Path("tools/eval").rglob("*.py")]'`; `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/harness -v` | `./scripts/validate.sh` |
| EVAL-01 | `eval/datasets/skill-router/batch-01/**`, `tools/eval/tests/skill_router_batch_01/**`, its handoff | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/skill_router_batch_01 -v` | `./scripts/validate.sh` |
| EVAL-02 | `eval/datasets/risk-judge/batch-01/**`, `tools/eval/tests/risk_judge_batch_01/**`, its handoff | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/risk_judge_batch_01 -v` | `./scripts/validate.sh` |
| EVAL-03 | `eval/datasets/skill-router/batch-02/**`, `eval/reports/skill-router/**`, `tools/eval/tests/skill_router_batch_02/**`, its handoff | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/skill_router_batch_02 -v` | `./scripts/validate.sh` |
| EVAL-04 | `eval/datasets/risk-judge/batch-02/**`, `eval/reports/risk-judge/**`, `tools/eval/tests/risk_judge_batch_02/**`, its handoff | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/risk_judge_batch_02 -v` | `./scripts/validate.sh` |
| AUD-01 | `docs/planning/zhoor-adoption/handoffs/reviews/AUD-01-*.md` only | Review report `git diff --check`; record candidate checks in report | `./scripts/validate.sh` (read-only) |
| BENCH-01 | `crates/backend-adapter/**`, its handoff | `./scripts/validate.sh --package zhoor-backend-adapter`; `cargo test --offline --locked -p zhoor-backend-adapter --test backend_adapter_contract` | `./scripts/validate.sh` |
| BENCH-02 | `eval/benchmarks/configs/**`, `eval/benchmarks/results/**`, `eval/reports/benchmarks/**`, its handoff | `git diff --check` on owned output trees and recorded frozen-run reproduction command | `./scripts/validate.sh` |
| SHADOW-01 | `crates/shadow/**`, `eval/reports/shadow/**`, its handoff | `./scripts/validate.sh --package zhoor-shadow`; `cargo test --offline --locked -p zhoor-shadow --test shadow_contract` | `./scripts/validate.sh` |
| GRANT-01 | `crates/grant/**`, `fixtures/grant/**`, `docs/contracts/grant/**`, its handoff | `./scripts/validate.sh --package zhoor-grant`; `cargo test --offline --locked -p zhoor-grant --test grant_contract` | `./scripts/validate.sh` |
| AIOSD-01 | `crates/aiosd-core/**`, `fixtures/aiosd-core/**`, `docs/contracts/aiosd-core/**`, its handoff | `./scripts/validate.sh --package zhoor-aiosd-core`; `cargo test --offline --locked -p zhoor-aiosd-core --test aiosd_core_contract` | `./scripts/validate.sh` |
| AIOSD-02 | `crates/aiosd-unix/**`, `docs/contracts/aiosd-transport/**`, its handoff | `./scripts/validate.sh --package zhoor-aiosd-unix`; `cargo test --offline --locked -p zhoor-aiosd-unix --test unix_transport_contract` | `./scripts/validate.sh` |
| AUD-02 | `docs/planning/zhoor-adoption/handoffs/reviews/AUD-02-*.md` only | Review report `git diff --check`; record threat/candidate checks in report | `./scripts/validate.sh` (read-only) |
| CANARY-01 | `docs/architecture/adr/ADR-CANARY-01-*.md`, `docs/evaluations/canary-01/**`, its handoff | `git diff --check` on those owned paths | `./scripts/validate.sh` |

The four required parallel wave pairs and the three later grant/enforcement cards have disjoint source, schema, fixture, test, documentation, report, and handoff roots. Root manifests/lockfile are not component write paths. Cargo artifacts live in per-worktree `target/`; Python validation disables bytecode output. Review-only AUD cards own report paths only.

## Runtime routing discrepancy

`MODEL_AND_SKILL_MATRIX.md` still records the older operational mapping of Sol/Pi for architecture-primary and Luna/Codex for independent-challenger. This run uses the owner-assigned mapping: Luna/Codex high as controller; Luna/Pi xhigh as challenger one; Sol/Codex-or-Pi high as challenger two. The matrix is left unchanged as directed. Current Herdr discovery shows only the controller's Codex pane; its listing confirms the harness but does not expose model or reasoning. No reviewer model has been started before candidate freeze. Both reviewer runtime states must be verified by fresh role-only handshakes before source review.

## Risks and unresolved points

- ARC-02's workload, latency samples, process RSS, and Linux peer-credential probe are narrow and noisy; they do not establish production SLOs, cgroup memory, concurrency, or service lifecycle behavior.
- Exact Rust 1.98.1 is the observed available toolchain; upgrades require explicit review. Offline builds depend on pre-cached, lockfile-pinned crates.
- JSON Schema-to-Rust binding generation is intentionally not selected here; its implementation must preserve the schema as sole contract authority and avoid an unjustified runtime dependency.
- Python's standard-library tooling receives syntax and unit checks but no separate formatter/type-checker dependency; it cannot implement production semantic rules.
- No currently visible Herdr reviewer pane is available. Review preparation must create/use a fresh isolated session only after candidate freeze and verify the actual requested model/reasoning. If the assigned lanes cannot be verified, stop under the run's stop conditions.
- Full source/build/CI validation cannot run until BASE-01 exists; ARC-03 validates the future command contract and current documentation only.

## Candidate and review results

- Candidate commit/SHA: pending.
- Luna exact-SHA review: pending.
- Sol exact-SHA review: pending.
- Reviewer reports: pending; use `docs/planning/zhoor-adoption/handoffs/reviews/ARC-03-<reviewer>.md` and identify the exact reviewed SHA.
- Owner acceptance: pending. No status transition, BASE-01 start, or push is authorized before explicit YES.

## Next action

Finish exact-scope validation and staging, create the single ARC-03 candidate commit, freeze and record its full SHA, then prepare and assign the Luna exact-SHA review.
