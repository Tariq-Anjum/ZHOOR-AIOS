# AIOSD-02 — Unix transport, peer identity, and lifecycle hardening

**Depends on:** AIOSD-01  
**Recommended:** GLM 5.3 Flash  
**Review:** GPT-5.6 Sol  
**Timebox:** one transport/security session

## Objective

Wrap the reviewed `aiosd` enforcement core in the pinned local Unix-socket framing, authenticate peer credentials, enforce limits/timeouts, and prove cleanup/crash behavior for only `capabilities.describe`.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read AIOSD-01 handoff/core interface, SEC-01 framing contract, GRANT-01 verifier, and toolchain ADR.
- Karpathy application: no generic server framework, remote transport, privilege escalation, or additional capability.

## Write scope

- `crates/aiosd-unix/**` — `zhoor-aiosd-unix` Unix transport/peer adapter and lifecycle integration tests.
- `docs/contracts/aiosd-transport/**` and `docs/planning/zhoor-adoption/handoffs/AIOSD-02.md` for transport-specific operator docs.
- `crates/aiosd-core/**`, grants, schemas, and receipts are read-only dependencies. No additional capability or remote transport.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-aiosd-unix` and `cargo test --offline --locked -p zhoor-aiosd-unix --test unix_transport_contract`.

Full: `./scripts/validate.sh`.

Lockfile timing: component workers do not edit or commit the root `Cargo.lock`. Before integration, run the package checks from `docs/development/ZHOOR_TOOLCHAIN.md` without `--locked` (keep `--offline`) and report any generated lockfile diff without staging it. After integrating the package source/manifests, the controller reconciles and reviews the single root lockfile update, then runs the locked targeted and full commands above on the integration candidate. This card is complete only after those post-integration checks pass.

## Acceptance

- Derives peer identity from platform credentials, never client fields.
- Enforces frame/input/concurrency/time limits and fail-closed error behavior.
- Integrates reviewed grant verification and exact argument binding.
- Proves socket permission, stale-socket cleanup, shutdown, timeout, crash/restart, and receipt behavior.
- Receipt storage remains immutable/append-only; overwrite and truncation attempts fail.
- Only `capabilities.describe` is registered.

## Stop

After two peer-credential/lifecycle failures, stop for security review. Do not substitute a TCP socket or claimed identity.
