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

- Unix-socket transport/peer adapter, lifecycle integration tests, operator docs, and `docs/planning/zhoor-adoption/handoffs/AIOSD-02.md`.
- ARC-03 must replace this description with exact path globs before `READY`.

## Acceptance

- Derives peer identity from platform credentials, never client fields.
- Enforces frame/input/concurrency/time limits and fail-closed error behavior.
- Integrates reviewed grant verification and exact argument binding.
- Proves socket permission, stale-socket cleanup, shutdown, timeout, crash/restart, and receipt behavior.
- Receipt storage remains immutable/append-only; overwrite and truncation attempts fail.
- Only `capabilities.describe` is registered.

## Stop

After two peer-credential/lifecycle failures, stop for security review. Do not substitute a TCP socket or claimed identity.
