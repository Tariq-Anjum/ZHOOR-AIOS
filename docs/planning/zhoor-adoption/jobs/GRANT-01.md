# GRANT-01 — Policy grant issuer and verifier

**Depends on:** SEC-01, SHADOW-01  
**Recommended:** Qwen3.8 Max  
**Review:** GPT-5.6 Sol  
**Timebox:** one security implementation session

## Objective

Implement the deterministic Policy Engine path that issues and verifies short-lived, single-operation grants for only `capabilities.describe`. Establish trust-root ownership, expiry/replay state, exact normalized argument binding, and negative tests without executing capabilities.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read architecture section 8, SEC-01 contracts/handoff, shadow audit evidence, accepted policy interfaces, and toolchain ADR.
- Karpathy application: one capability and one local trust design; no generic identity, RBAC, secret platform, or mutation permissions.

## Write scope

- Policy grant issuer/verifier implementation and tests.
- Local development/test trust-root handling and operator documentation.
- `docs/planning/zhoor-adoption/handoffs/GRANT-01.md`.
- ARC-03 must replace this description with exact path globs before `READY`.

## Acceptance

- Policy—not a model, worker, client, or `aiosd`—owns grant issuance.
- Grant binds issuer, authenticated client identity, capability, normalized argument hash, resource scope, policy version, expiry, nonce/grant ID, trace ID, and decision/evidence hashes.
- Verification rejects expiry, replay, issuer/key mismatch, caller mismatch, substitution, argument tampering, and version mismatch.
- Only `capabilities.describe` is issuable; issuing a grant does not execute it.
- Key/test-root handling is explicit, local, and never hard-coded as a production secret.

## Stop

After two trust-root/replay implementation failures, stop for security review. Never self-sign inside `aiosd` or accept fixture grants in deployable paths.
