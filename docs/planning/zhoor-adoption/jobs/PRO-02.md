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

## Provisional proposal for the pre-implementation architecture challenge

This proposal is a challenge target, not an accepted contract until the architecture challenge and implementation gates pass:

- `descriptor_format_version` identifies the descriptor field contract, not the version of the skill artifact. V1 accepts exactly the string `"1"`; unknown values fail closed.
- V1 does not add a separate `skill_artifact_version`. The frozen architecture and current compact index do not provide one stable artifact-version convention. A content digest identifies exact bytes and must not be presented as a release/version label.
- `artifact_sha256` identifies the exact raw bytes of the primary skill artifact file (`SKILL.md`), encoded as lowercase `sha256:<64 hex digits>`. No JSON or YAML canonicalization is involved. The registry loader need not read the body to load metadata; the digest is precomputed in the index and may be checked when the body is later fetched.
- `registry_version` is an opaque, non-empty ASCII snapshot label, separate from descriptor format and artifact digest. The index declares it; a caller that requires a particular snapshot supplies an expected value, and mismatch fails closed. It does not authenticate or sign the snapshot.
- The digest is content identity/integrity metadata only. It does not provide authentication, authorization, provenance, trust, or anonymization. V1 does not define a descriptor-metadata digest.
- Existing request-v1 and response-v1 envelopes remain byte-for-byte unchanged. In particular, request candidates remain `Vec<String>`.

The challenger must test this proposal against the bounded repository evidence and report ambiguity or concrete defects before protocol source changes.

## Write scope

- `schemas/protocol/**` — additive shared identity/version schema only; do not change request-v1 or response-v1.
- `crates/protocol/**` — additive typed primitives and tests only; do not change existing request/response semantics.
- `fixtures/protocol/**` — fixtures for the new contract only; existing request-v1/response-v1 fixtures remain unchanged.
- `docs/planning/zhoor-adoption/handoffs/PRO-02.md`.
- Controller-owned board/decision changes only for the PRO-02 lifecycle. Do not edit registry, trace, policy, `aiosd`, or worker paths.

## Acceptance

- Precisely distinguishes descriptor format version, skill artifact version, artifact content digest, and registry snapshot version.
- Defines the digest target and byte representation, algorithm, text grammar, and mismatch behavior without ambiguous canonicalization.
- Defines registry snapshot label location, meaning, expectation, and mismatch behavior.
- Adds only protocol-owned reusable primitives; the full descriptor and registry/index remain REG-01-owned.
- Existing request-v1/response-v1 schemas, Rust public envelope types, and fixtures are unchanged and continue to pass.
- New JSON Schema and Rust behavior agree on accepted/rejected identity metadata, unknown fields fail closed where applicable, and version/hash mismatches are covered.
- Architecture challenge is captured before source mutation; canonical targeted/full checks pass; exact-SHA independent review passes.

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
