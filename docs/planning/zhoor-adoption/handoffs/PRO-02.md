# PRO-02 handoff — registry identity/version contract

## Run state

- Starting local SHA: `f006fbea9df6efe8eaa252debd53d55d8a4dc115`
- Starting remote SHA: `2e73ef44181bc74b55fad25e7e2fd5f1dac9eaa1`
- Owner contract authorization SHA: `366577a238cfb2f8500c981ab98e8f7824b5797b`
- Assignment SHA: `a2bc3ea1fc17bb5b9bed26052d4dfe64ea303f94`
- Implementation worktree: `/tmp/zhoor-pro-02/ZHOOR-20260930T101500Z-PRO-02/worker`, branch `work/pro-02-20260930-resumed`
- Current status: `REVIEW`; implementation and lockfile reconciliation have been integrated
- Isolated implementation commit: `7f37e3e00530e1f78ffe49b5c4f936671633a5ac`
- Integrated implementation SHA: `0d55e9db596608c6ce47e2ea063615092c0c4662`
- Cargo.lock reconciliation commit: `c81bdb3aca1d924ba05dac87c3d1acdf387f5e2f`
- Prior exact-SHA architecture challenge: `OWNER_CONTRACT_DECISION_REQUIRED` at prior assignment `2cb8d54560079d1a012ee5014e1b96dd9f15f1a0`; the owner resolved format placement in D-021
- Prior challenge lane: contained Pi / OpenAI-Codex / GPT-6 Luna xhigh / read-only; `AGENT_CONTAINMENT=PASS`; stopped after result capture
- Prior challenge evidence: `/tmp/zhoor-pro-02/ZHOOR-20260930T081737Z-PRO-02/architecture-challenge-result.md`

## Accepted meanings

- **Descriptor format version:** the descriptor field contract version. Every entry in a v1 snapshot uses format `"1"`; the value is carried once at the registry root. The artifact's release/version is separate and not defined in v1.
- **Artifact hash:** `artifact_sha256` identifies exact raw bytes of the `SKILL.md` file only. It does not cover descriptor metadata or companion files.
- **Hash algorithm and grammar:** SHA-256, represented exactly as `sha256:<64 lowercase hexadecimal characters>`. Hashing performs no newline, Unicode, or serialization normalization. A hash is a content integrity identifier; it is not authentication, trust, authorization, anonymization, or a digital signature.
- **Registry version:** an opaque snapshot label at the registry root, distinct from descriptor format and artifact version. It must not be reused for different snapshot contents within the same configured canonical registry source. The label is not globally unique.
- **Registry identity/scope evidence:** the local canonical registry documentation states that the version-controlled `my-ai-skills` repository is the source of truth and local installed catalogs are runtime copies. Its origin is `https://github.com/Tariq-Anjum/my-ai-skills.git`. Therefore the minimum scope is the selected canonical registry source; local/runtime copies of that source share the same identity. This adds no global namespace or registry identity wire field.
- **Mismatch behavior:** when a caller supplies an expected `registry_version`, a different loaded root value is an explicit `RegistryVersionMismatch`; no snapshot substitution or count/timestamp/hash inference occurs.
- **Fail-closed format handling:** missing, non-string/malformed, unknown, or unsupported root `descriptor_format_version` is rejected by protocol metadata extraction before registry entry handling. Mixed versions are not represented.
- **Compatibility and ownership:** protocol-v1 request and response schemas, Rust envelope types, and fixtures remain unchanged; request candidates remain strings. The complete registry schema, descriptor fields, unknown-root closure, ordering, and lookup remain REG-01-owned.

## Implementation

The additive protocol API provides `DescriptorFormatVersion`, `RegistryVersion`, `ArtifactSha256`, `RegistrySnapshotMetadata`, and `RegistryContractError`. `RegistrySnapshotMetadata::from_root_value` extracts and validates the two protocol-owned root fields before entries are processed; the registry package remains responsible for closing and validating the complete root object. `ArtifactSha256` parses, computes, and verifies exact `SKILL.md` bytes. The schema fragment is composable so REG-01 can add registry-owned properties and close the complete root schema.

The protocol crate uses the already locked/cached `sha2 = 0.10.9`, already used by `zhoor-trace`; it is a new direct package dependency but not a new resolved package or network fetch. Unlocked offline resolution changed only the `zhoor-protocol` dependency list in the worktree's generated `Cargo.lock`. The implementation commit must not include that root lockfile change; the controller reconciles it during integration.

## Fixtures and tests

The new identity fixtures cover valid root metadata, absent/malformed/unsupported root format, malformed/missing registry version, valid artifact hash, wrong algorithm, wrong length, uppercase text, a known SHA-256 vector (`hello`), changed bytes, expected-version match, and explicit mismatch. Schema/Rust parity is checked for these fixtures. The metadata schema intentionally leaves registry-owned root keys open for composition; the final closed root and unknown-field behavior belong to REG-01. Existing request-v1/response-v1 fixtures are unchanged.

## Validation evidence

- Pre-integration unlocked/offline package sequence: PASS — `cargo +1.98.1 fmt --all -- --check`; `cargo +1.98.1 clippy --offline -p zhoor-protocol --all-targets -- -D warnings`; `cargo +1.98.1 check --offline -p zhoor-protocol --all-targets`; `cargo +1.98.1 test --offline -p zhoor-protocol` (21 unit tests, 14 contract tests, doctests).
- Canonical post-integration targeted validation: PASS — `./scripts/validate.sh --package zhoor-protocol`; `cargo +1.98.1 test --offline --locked -p zhoor-protocol --test protocol_contract` (14 tests).
- Canonical full validation: PASS — `./scripts/validate.sh` (workspace lint, check, tests, docs, debug/release builds).
- `git diff --check` on the complete PRO-02 change range from assignment `a2bc3ea1fc17bb5b9bed26052d4dfe64ea303f94` through current implementation/lock commits: PASS. The unscoped working-tree `git diff --check` reports trailing spaces in pre-existing D-018-protected job-card edits; those paths remain unchanged and preserved.
- Root `Cargo.lock` SHA-256 before resolution: `4585fbd4546d6b79045c87a83f4796dafe5144ff628cb4cb5fd297bc4f1a2150`; post-reconciliation and before/after locked validation: `4447c1d03b40adb9e0217b7cf20c6845e30986555ae475a9b0fb074b19cc0e32` (unchanged by locked validation).
- D-018: PASS; all 28 status/path/SHA-256 records match, with zero extras.
- Exact-SHA independent review: PASS for candidate `7100cb6fff25cb49238e393cb55f540e143ff04d`; `AGENT_CONTAINMENT=PASS` before packet; report `docs/planning/zhoor-adoption/handoffs/reviews/PRO-02-herdr-review.md`.
- Reviewer route: fresh contained Pi session in separate Herdr tab; OpenAI/Codex route; GPT-6 Luna / xhigh; read-only tools; exact detached candidate worktree. The reviewer inspected source/schema/fixtures/test code but did not run Cargo. The lane is stopped/completed; only the controller remains active.
- Review outcome: no blocking findings. The reviewer confirmed the format/hash/snapshot distinctions, request-v1 compatibility, canonical-source scope, expected-version mismatch, protocol/registry ownership boundary, fixture coverage, and absence of security overclaims.
- D-018 closure recheck: PASS; 28/28 original status/path/SHA-256 records match and there are no extras after recording the review report.

## Closure

PRO-02 is DONE after the targeted and full offline validation passed, the exact-SHA independent review passed with no blockers, D-018 reconciled, and locked validation left the reconciled Cargo.lock hash unchanged. REG-01 remains BLOCKED pending a separate owner authorization. POL-01, SEC-01, WRK-01, and WRK-02 remain BLOCKED; no later card was started.

## Closure and stop

PRO-02 may move to DONE only after canonical targeted/full validation, exact-SHA independent review PASS, all findings resolved, D-018 reconciliation, and Cargo.lock policy pass. On closure, REG-01 remains BLOCKED until a separate owner authorization. POL-01, SEC-01, WRK-01, and WRK-02 remain BLOCKED and are not authorized by this run.
