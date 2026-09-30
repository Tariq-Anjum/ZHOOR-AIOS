# PRO-02 — Registry descriptor identity/version contract

**Depends on:** PRO-01
**Recommended:** Herdr Runtime Server implementation session
**Review:** independent architecture challenge before implementation, then exact-SHA review
**Timebox:** one bounded implementation session

## Objective

Define and implement only the protocol-owned identity/version/hash primitives that REG-01 needs. Keep the complete skill descriptor, registry/index schema, and lookup behavior owned by REG-01.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read ZHOOR architecture §§3, 4, 5, 9, and 10; PRO-01 card and handoff; REG-01 card and blocker handoff; protocol schemas/types/fixtures; and only representative compact local index entries.
- Do not treat legacy HERMES plugin conventions as normative.

## Owner-approved contract constraints

The owner resolved the prior architecture challenge in D-021. Implement the following bounded contract, subject to the registry identity/scope stop gate below:

- `descriptor_format_version` identifies the descriptor field contract, not the version of a skill artifact. One registry snapshot carries it once at the root, with exact v1 value `"1"`; entries do not repeat it. Reject the entire snapshot before entry processing when it is absent, malformed, unknown, or unsupported. Mixed descriptor formats within a snapshot are unsupported.
- V1 does not add a separate `skill_artifact_version`. The frozen architecture and current compact index do not provide one stable artifact-version convention. A content digest identifies exact bytes and must not be presented as a release/version label.
- `artifact_sha256` identifies the exact raw bytes of `SKILL.md`, encoded as lowercase `sha256:<64 lowercase hexadecimal characters>`. No descriptor metadata or canonicalized representation is hashed. Do not introduce canonical-JSON machinery. It is an integrity/content identity only and does not authenticate, authorize, establish trust, provide anonymization, or act as a signature.
- `registry_version` is a registry snapshot identity label, not a descriptor-format version or artifact version. It is carried once at the registry root. Within the already established registry identity/scope, a value must not be reused for different snapshot contents. A caller-supplied expected value that differs from the loaded snapshot must produce an explicit mismatch failure; do not silently substitute the current snapshot.
- Existing request-v1 and response-v1 envelopes remain byte-for-byte unchanged. In particular, request candidates remain `Vec<String>`.

## Registry identity/scope stop gate

Before protocol source mutation, inspect the accepted architecture and canonical registry ownership to identify the existing registry identity/scope that bounds `registry_version` non-reuse. Do not invent a global namespace. If the repository leaves two materially reasonable scopes, stop and return `OWNER_CONTRACT_DECISION_REQUIRED` with at most two options and their contract/API/compatibility/integrity consequences.

## Write scope

- `schemas/protocol/**` — additive shared identity/version schema only; do not change request-v1 or response-v1.
- `crates/protocol/**` — additive typed primitives and tests only; do not change existing request/response semantics.
- `fixtures/protocol/**` — fixtures for the new contract only; existing request-v1/response-v1 fixtures remain unchanged.
- `docs/planning/zhoor-adoption/handoffs/PRO-02.md`.
- Controller-owned board/decision changes only for the PRO-02 lifecycle. Do not edit registry, trace, policy, `aiosd`, or worker paths.

## Acceptance

- Precisely distinguishes descriptor format version, skill artifact version (not defined in v1), artifact content digest, and registry snapshot version.
- Defines the digest target and byte representation, algorithm, text grammar, and mismatch behavior without ambiguous canonicalization.
- Defines registry snapshot label location, meaning, scope/non-reuse rule, expectation, and mismatch behavior.
- Adds only protocol-owned reusable primitives; the full descriptor and registry/index remain REG-01-owned.
- Existing request-v1/response-v1 schemas, Rust public envelope types, and fixtures are unchanged and continue to pass.
- New JSON Schema and Rust behavior agree on accepted/rejected identity metadata, unknown fields fail closed where applicable, and version/hash mismatches are covered.
- The prior architecture challenge and D-021 owner decision are captured before source mutation; canonical targeted/full checks pass; exact-SHA independent review passes.

## Validation

Targeted:

```bash
./scripts/validate.sh --package zhoor-protocol
cargo +1.98.1 test --offline --locked -p zhoor-protocol --test protocol_contract
```

Full:

```bash
./scripts/validate.sh
git diff --check
```

All Cargo resolution remains offline. The controller owns root `Cargo.lock`; record its SHA-256 before and after locked validation. If a new dependency is necessary and not already cached and approved, stop.

## Stop

- Do not modify existing request-v1/response-v1 envelopes to carry descriptor objects.
- If repository evidence plus the independent architecture challenge leaves two materially different reasonable contracts without an authoritative choice, return `OWNER_CONTRACT_DECISION_REQUIRED` with at most two options and the specified impact summary.
- Do not resume REG-01 automatically when PRO-02 closes. A new explicit owner authorization is required.
- Do not start POL-01, SEC-01, WRK-01, WRK-02, or another card.
