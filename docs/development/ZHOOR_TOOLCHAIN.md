# ZHOOR production toolchain and validation

**Status:** proposed with ADR-0003; pending explicit owner acceptance. Commands below become available after `BASE-01` creates the root workspace and `scripts/validate.sh`. ARC-03 does not create a production scaffold.

## Toolchain policy

- Production implementation: Rust 2024, toolchain `1.98.1`, pinned in root `rust-toolchain.toml` with `rustfmt` and `clippy`. Do not use a floating stable channel for canonical checks. Upgrade the pin through a reviewed change.
- Workspace/package layout: root `zhoor` CLI plus one package per owned component in `crates/`. `crates/system-info` remains the separate ADR-0002 slice; do not call it `aiosd`.
- Contract authority: JSON Schema under `schemas/` (with protocol schemas in `schemas/protocol/**` and trace schema solely in `schemas/trace/**`). Rust owns the only production contract and domain validation. Generated bindings are derived outputs. Python tools may inspect schemas and fixture data, but must delegate normative validation and all policy/domain decisions to Rust.
- Python boundary: Python 3.14.7 standard library only, for offline evaluation analysis, fixture/dataset generation, benchmark/report tooling, and developer utilities. It is development-only, is not packaged with production code, and must not be on the request path. No third-party Python package is approved. Keep scripts small and cover them with `compileall` and `unittest`; Rust production lint/type checks remain the blocking canonical lint/static checks.
- Cargo dependency changes are reviewed and locked. Canonical validation is offline and uses the committed lockfile. No validation command downloads tools, models, or dependencies.

## Root lockfile integration and validation timing

`Cargo.lock` is integration-owned. Component cards must not edit or commit it. A component worker may run its pre-integration package checks with the exact package commands below, retaining `--offline` and omitting `--locked`; Cargo may update the local lockfile while resolving a newly added workspace member or an approved dependency. The worker reports that generated diff and does not stage it. After integrating the component source and manifest, the controller reviews and reconciles the root lockfile once on the integration candidate. The controller then runs the card's canonical locked targeted command and `./scripts/validate.sh`; a card is not complete until both pass on that reconciled integration candidate. CI and all post-integration validation always use the committed lockfile with `--locked`.

## Stable command interface

`BASE-01` implements `scripts/validate.sh` with this exact interface:

```sh
./scripts/validate.sh
./scripts/validate.sh --package zhoor-protocol
```

No argument validates the full Rust workspace and all committed offline Python tooling. `--package <workspace-package>` runs format plus all lint, static, test, and build checks for one Rust package; workspace formatting still checks all files. The script rejects unknown arguments and propagates the first failing command. It uses `--offline --locked` for Cargo. CI runs `./scripts/validate.sh` from the repository root.

## Canonical Rust commands

Run from repository root after `BASE-01`:

| Check | Exact command |
|---|---|
| Format | `cargo fmt --all` |
| Format check | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --workspace --all-targets --offline --locked -- -D warnings` |
| Static/type check | `cargo check --workspace --all-targets --offline --locked` |
| Unit tests | `cargo test --workspace --lib --offline --locked` |
| Integration/contract tests | `cargo test --workspace --tests --offline --locked` |
| Full test suite (unit, integration, doctest) | `cargo test --workspace --offline --locked` |
| Build/package | `cargo build --workspace --offline --locked` |
| Release build | `cargo build --workspace --release --offline --locked` |
| CI-equivalent/full validation | `./scripts/validate.sh` |

`cargo fmt --all` is for intentional formatting edits; validation and CI use the `--check` form. Package-targeted post-integration checks use `./scripts/validate.sh --package <name>`. For pre-integration worker checks, use the corresponding Cargo commands with the package selector (`-p <name>`), retain `--offline`, and omit `--locked`; do not stage the resulting root lockfile diff. The corresponding direct contract-test command for PRO-01 is `cargo test --offline --locked -p zhoor-protocol --test protocol_contract`; each implementation card names its own contract-test target below. If a card has no Rust package, its targeted Python or report check is stated in that card.

## Python offline tooling commands

For the EVAL-00 Python harness and later dataset/report tooling, run from repository root:

```sh
python3.14 -c 'import ast, pathlib; [ast.parse(p.read_text(), filename=str(p)) for p in pathlib.Path("tools/eval").rglob("*.py")]'
PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests -v
```

The full-validation script uses `PYTHONDONTWRITEBYTECODE=1` and runs both commands when `tools/eval/` exists. Its Rust sequence is format check, Clippy, static check, unit tests, integration/contract tests, full tests/doctests, debug build, and release build; it writes build artifacts only beneath ignored `target/`. The Python AST and unittest checks write no checkout artifacts. Python tests consume canonical schema/fixture data and may call the Rust validator; they do not reimplement Rust protocol, registry, policy, worker, grant, or enforcement behavior. Python-only dataset cards name a narrower unittest discovery directory and still run `./scripts/validate.sh` for full CI-equivalent validation.

## Targeted/full validation by card

| Card | Targeted validation | Full validation |
|---|---|---|
| BASE-01 | `cargo fmt --all -- --check && cargo check --workspace --all-targets --offline --locked && cargo test --workspace --offline --locked` | `./scripts/validate.sh` |
| PRO-01 | `./scripts/validate.sh --package zhoor-protocol` and `cargo test --offline --locked -p zhoor-protocol --test protocol_contract` | `./scripts/validate.sh` |
| REG-01 | `./scripts/validate.sh --package zhoor-registry` and `cargo test --offline --locked -p zhoor-registry --test registry_contract` | `./scripts/validate.sh` |
| POL-01 | `./scripts/validate.sh --package zhoor-risk-policy` and `cargo test --offline --locked -p zhoor-risk-policy --test risk_policy_contract` | `./scripts/validate.sh` |
| TRC-01 | `./scripts/validate.sh --package zhoor-trace` and `cargo test --offline --locked -p zhoor-trace --test trace_contract` | `./scripts/validate.sh` |
| SEC-01 | `./scripts/validate.sh --package zhoor-aiosd-contracts` and `cargo test --offline --locked -p zhoor-aiosd-contracts --test aiosd_contract` | `./scripts/validate.sh` |
| WRK-01 | `./scripts/validate.sh --package zhoor-skill-router` and `cargo test --offline --locked -p zhoor-skill-router --test skill_router_contract` | `./scripts/validate.sh` |
| WRK-02 | `./scripts/validate.sh --package zhoor-risk-judge` and `cargo test --offline --locked -p zhoor-risk-judge --test risk_judge_contract` | `./scripts/validate.sh` |
| EVAL-00 | `python3.14 -c 'import ast, pathlib; [ast.parse(p.read_text(), filename=str(p)) for p in pathlib.Path("tools/eval").rglob("*.py")]' && PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/harness -v` | `./scripts/validate.sh` |
| EVAL-01 | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/skill_router_batch_01 -v` | `./scripts/validate.sh` |
| EVAL-02 | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/risk_judge_batch_01 -v` | `./scripts/validate.sh` |
| EVAL-03 | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/skill_router_batch_02 -v` | `./scripts/validate.sh` |
| EVAL-04 | `PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests/risk_judge_batch_02 -v` | `./scripts/validate.sh` |
| AUD-01 | `git diff --check -- docs/planning/zhoor-adoption/handoffs/reviews/AUD-01-*.md` plus validation responsibilities in the review report | `./scripts/validate.sh` |
| BENCH-01 | `./scripts/validate.sh --package zhoor-backend-adapter` and `cargo test --offline --locked -p zhoor-backend-adapter --test backend_adapter_contract` | `./scripts/validate.sh` |
| BENCH-02 | `git diff --check -- eval/benchmarks/configs/ eval/benchmarks/results/ eval/reports/benchmarks/` plus recorded frozen-run reproduction command | `./scripts/validate.sh` |
| SHADOW-01 | `./scripts/validate.sh --package zhoor-shadow` and `cargo test --offline --locked -p zhoor-shadow --test shadow_contract` | `./scripts/validate.sh` |
| GRANT-01 | `./scripts/validate.sh --package zhoor-grant` and `cargo test --offline --locked -p zhoor-grant --test grant_contract` | `./scripts/validate.sh` |
| AIOSD-01 | `./scripts/validate.sh --package zhoor-aiosd-core` and `cargo test --offline --locked -p zhoor-aiosd-core --test aiosd_core_contract` | `./scripts/validate.sh` |
| AIOSD-02 | `./scripts/validate.sh --package zhoor-aiosd-unix` and `cargo test --offline --locked -p zhoor-aiosd-unix --test unix_transport_contract` | `./scripts/validate.sh` |
| AUD-02 | `git diff --check -- docs/planning/zhoor-adoption/handoffs/reviews/AUD-02-*.md` plus validation responsibilities in the review report | `./scripts/validate.sh` |
| CANARY-01 | `git diff --check -- docs/architecture/adr/ADR-CANARY-01-*.md docs/evaluations/canary-01/ docs/planning/zhoor-adoption/handoffs/CANARY-01.md` | `./scripts/validate.sh` |

Review-only cards do not gain implementation ownership by running full validation. Their reports identify the exact candidate SHA and the checks they executed.
