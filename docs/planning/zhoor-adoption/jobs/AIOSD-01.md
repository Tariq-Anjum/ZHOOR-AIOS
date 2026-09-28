# AIOSD-01 — aiosd enforcement core and append-only receipts


**Depends on:** GRANT-01  
**Recommended:** Muse Spark 1.3  
**Review:** GPT-5.6 Sol  
**Timebox:** one enforcement-core session; Unix transport is AIOSD-02

## Objective

Implement a transport-independent `aiosd` enforcement core that accepts authenticated peer identity from a trusted adapter, verifies the policy-issued grant and normalized arguments, dispatches only `capabilities.describe`, and writes immutable append-only receipts. It performs no mutation or privilege escalation.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read architecture section 8, accepted aiosd contract/fixtures, GRANT-01 issuer/verifier interface, toolchain ADR, and SEC-01 handoff.
- Karpathy application: one non-mutating capability; no generic command runner, plugin loader, or privilege framework.

## Write scope

- `crates/aiosd-core/**` — `zhoor-aiosd-core` transport-independent enforcement core, fixture adapter, fixed capability registry, receipt store, and tests.
- `fixtures/aiosd-core/**` and `docs/contracts/aiosd-core/**` for core-only cases/contract notes.
- `docs/planning/zhoor-adoption/handoffs/AIOSD-01.md`.
- No Unix socket/server source; `crates/aiosd-unix/**` is reserved to AIOSD-02. SEC-01 schemas and GRANT-01 package are read-only.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-aiosd-core` and `cargo test --offline --locked -p zhoor-aiosd-core --test aiosd_core_contract`.

Full: `./scripts/validate.sh`.

## Acceptance

- Requires peer identity from a trusted adapter and never accepts identity from request fields.
- Rejects expired/replayed/mismatched/tampered grants and malformed/oversized requests.
- Revalidates normalized arguments and capability allowlist independently.
- Implements only `capabilities.describe`; no shell, filesystem mutation, network, package, service, or privilege operations.
- Emits immutable append-only receipts without secrets; overwrite/truncation attempts fail in tests.
- Fails closed on verifier, dispatch, receipt, crash, and timeout errors.
- Core contract/integration tests pass; Unix peer/socket lifecycle is deferred to AIOSD-02.

## Stop

If grant verification or append-only receipt behavior cannot be implemented safely after two attempts, stop and request security review. Do not self-issue grants or trust user-supplied identity fields.
