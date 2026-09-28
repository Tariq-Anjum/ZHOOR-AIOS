# SEC-01 — aiosd contract and attack fixtures


**Depends on:** PRO-01
**Recommended:** security-reviewer
**Review:** independent-challenger in a separate session
**Timebox:** one contract/security session; no daemon

## Objective

Specify versioned `aiosd` request, short-lived single-operation grant, normalized argument binding, capability descriptor, and receipt contracts with positive and hostile fixtures. Do not implement execution.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Use OpenSpec only if already available; do not install it.
- Read ZHOOR architecture section 8, protocol contracts, and legacy threat categories only as checklist input.
- Karpathy application: pin only v1 `capabilities.describe`; do not design shell/service/package/network capabilities.

## Write scope

- `schemas/aiosd/**` — canonical aiosd/grant wire schemas owned only by this card.
- `crates/aiosd-contracts/**` — `zhoor-aiosd-contracts` package and contract tests only; no daemon, server, executor, grant issuer, or transport implementation.
- `fixtures/aiosd/**` and `docs/contracts/aiosd/**` for hostile fixtures and contract/threat notes.
- `docs/planning/zhoor-adoption/handoffs/SEC-01.md`.
- Do not edit protocol, grant, enforcement, root manifest, or lockfile paths.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-aiosd-contracts` and `cargo test --offline --locked -p zhoor-aiosd-contracts --test aiosd_contract`.

Full: `./scripts/validate.sh`.

## Acceptance

- Grant binds ID, expiry, policy version, authenticated client identity, exact capability, normalized argument hash, resource scope, originating trace, and decision/evidence hashes.
- Receipt records validated capability, caller, grant, argument hash, outcome, safe result reference, and timestamps.
- Fixtures reject expiry/replay, caller mismatch, capability substitution, argument tampering, unknown fields/version, path traversal/symlink escape representation, oversized inputs, and missing provenance.
- Contract is local Unix-socket oriented but transport framing is explicit.
- Only `capabilities.describe` is eligible and non-mutating.
- Before any execution-capable workflow is eligible for promotion, SEC-01 must define the required postcondition schema, verification owner, typed verdict/status, deterministic evidence, evidence linkage, failure/unknown behavior, and how verification affects later policy/state. An `aiosd` execution receipt records what was executed and observed; it does not itself prove required postconditions. Semantic output may be evidence/advice only and cannot replace required deterministic safety checks.

## Stop

Any unresolved cryptographic/key, privilege, multi-user, or path-canonicalization decision requires a security ADR/review. After two attempts, stop and record the exact ambiguity.
