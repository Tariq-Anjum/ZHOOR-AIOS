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

- `aiosd` enforcement core, fixture adapter, capability registry limited to `capabilities.describe`, append-only receipt store, tests, and `docs/planning/zhoor-adoption/handoffs/AIOSD-01.md`.
- ARC-03 must replace this description with exact path globs before `READY`; no Unix socket/server source.

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
