# BASE-01 — Minimal project scaffold


**Depends on:** ARC-03  
**Recommended:** GLM 5.3 Flash  
**Review:** GPT-5.6 Luna  
**Timebox:** one implementation session

## Objective

Create only the toolchain, package, test, and CI skeleton required to run an empty deterministic ZHOOR library/CLI according to the accepted toolchain ADR.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read the ARC-03-accepted toolchain ADR, `docs/development/ZHOOR_TOOLCHAIN.md`, `TARGET_SCAFFOLD.md`, and `AGENT_OPERATING_RULES.md`.
- Karpathy application: do not create future subsystem directories or add optional dependencies.

## Write scope

- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`.
- `src/**` and `tests/smoke.rs` for the minimal root CLI/package smoke only.
- `.github/workflows/zhoor.yml`, `scripts/validate.sh`, and toolchain-required `.gitignore` lines only.
- `docs/planning/zhoor-adoption/handoffs/BASE-01.md`.
- The root workspace uses `crates/*` members so component cards do not edit root manifests. The committed lockfile is integration-owned; component workers use only already-locked dependencies and `--locked` validation.


## Path ownership and validation

Targeted: `cargo fmt --all -- --check && cargo check --workspace --all-targets --offline --locked && cargo test --workspace --offline --locked`.

Full: `./scripts/validate.sh`.

CI must invoke exactly `./scripts/validate.sh` from repository root. The wrapper implements all canonical full checks in `docs/development/ZHOOR_TOOLCHAIN.md`.

## Acceptance

- Clean install/build from declared lock state.
- Format, lint, type/static check, unit test, and package/build commands pass.
- No provider, model, database, web, plugin, workflow, or execution dependencies.
- CI runs the same canonical commands.

## Stop

After two failures of the same setup/check, record evidence and stop. Do not change the ARC-03 decision, switch toolchains, or add workaround frameworks.
