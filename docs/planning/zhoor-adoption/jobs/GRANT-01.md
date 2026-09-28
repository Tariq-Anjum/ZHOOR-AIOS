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

- `crates/grant/**` — `zhoor-grant` issuer/verifier implementation and tests.
- `fixtures/grant/**` and `docs/contracts/grant/**` — grant fixtures, local test-root instructions, and operator contract docs.
- `docs/planning/zhoor-adoption/handoffs/GRANT-01.md`.
- `schemas/aiosd/**` from SEC-01 and AIOSD packages are read-only contracts. Do not edit enforcement/transport packages or root lockfiles.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-grant` and `cargo test --offline --locked -p zhoor-grant --test grant_contract`.

Full: `./scripts/validate.sh`.

Lockfile timing: component workers do not edit or commit the root `Cargo.lock`. Before integration, run the package checks from `docs/development/ZHOOR_TOOLCHAIN.md` without `--locked` (keep `--offline`) and report any generated lockfile diff without staging it. After integrating the package source/manifests, the controller reconciles and reviews the single root lockfile update, then runs the locked targeted and full commands above on the integration candidate. This card is complete only after those post-integration checks pass.

## Acceptance

- Policy—not a model, worker, client, or `aiosd`—owns grant issuance.
- Grant binds issuer, authenticated client identity, capability, normalized argument hash, resource scope, policy version, expiry, nonce/grant ID, trace ID, and decision/evidence hashes.
- Verification rejects expiry, replay, issuer/key mismatch, caller mismatch, substitution, argument tampering, and version mismatch.
- Only `capabilities.describe` is issuable; issuing a grant does not execute it.
- Key/test-root handling is explicit, local, and never hard-coded as a production secret.

## Stop

After two trust-root/replay implementation failures, stop for security review. Never self-sign inside `aiosd` or accept fixture grants in deployable paths.
