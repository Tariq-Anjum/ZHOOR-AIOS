# PRO-01 implementation handoff

## Status

`second review correction applied; canonical locked validation PASS; focused Luna re-review pending`

## Task identity

- RUN_ID: `ZHOOR-20260929-PRO01-PROTOCOL-01`
- TASK_ID: `ZHOOR-20260929-PRO01-PROTOCOL-01-IMPLEMENTER-PI-01`
- Role: implementation-low-cost
- Harness: Pi 0.87.1; isolated runtime, no shell tool; all validation and Git operations are controller-owned
- Provider/model: OpenRouter / Z.AI GLM 5.3 Flash (intended identity `z-ai/glm-5.3-flash`; verified by the controller, not introspected by this session)
- Branch: `work/pro-01-decision-protocol`
- Starting SHA: `eff8db3bb811e2943da17b95394aaa4e8c7df6ea` (clean status)
- Worktree: `/tmp/zhoor-pro01/ZHOOR-20260929-PRO01-PROTOCOL-01/worker`

## Success condition (restated)

Versioned v1 request/response/evidence/confidence/disposition/result-kind/deadline/error contracts under `schemas/protocol/**`; deterministic Rust semantic validation in `zhoor-protocol` under `crates/protocol/**`; positive/negative fixtures under `fixtures/protocol/valid/**` and `fixtures/protocol/invalid/**`; and a `protocol_contract` integration target enumerating every committed fixture with canonical Draft 2020-12 schema expectation, Rust expectation, and accept/reject agreement. Controller source audit corrections and implementation integration are complete. Canonical package validation, the standalone protocol contract target, and full workspace validation passed offline with the root lockfile locked and stable. The first exact-SHA Luna session returned BLOCKED after finding two schema/Rust semantic gaps; the blocker about attachment was based on interpreting `git show --decorate` incorrectly. Controller verified the checkout was detached and clean, confirmed both semantic findings, and applied bounded corrections. Canonical validation and focused exact-SHA re-review are pending. No transport, server, model integration, execution grant, policy engine, registry, trace implementation, backend API, database, or async framework.

## Schema test dependency resolution

The test-only Draft 2020-12 validator is `jsonschema = { version = "=0.33.0", default-features = false }`.

- The controller confirmed the cached crate source and crate archive for 0.33.0.
- The controller inspected the local source API: `jsonschema::draft202012::new(&schema)` returns a reusable `Validator`, which provides `is_valid`; `jsonschema::draft202012::meta::is_valid` validates schema documents against the Draft 2020-12 meta-schema.
- An isolated scratch probe compiled and ran with Rust 1.98.1 and `--offline`; the full 99-package dependency tree resolved from the local cache. `default-features = false` excludes HTTP/file reference resolvers; these canonical schemas are self-contained and use no external `$ref`.
- No network fetch or install was performed. The controller added the dependency as dev-only and exact-pinned it.

The contract integration test now compiles both canonical schemas with the Draft 2020-12 API, validates the schemas against the Draft 2020-12 meta-schema, enumerates every committed fixture in `valid/` and `invalid/`, and asserts that schema and Rust acceptance both match the fixture category. Cross-envelope semantic cases (foreign evidence, candidate membership, request/deadline linkage) are separate in-memory Rust tests; no fixture under `valid/` or `invalid/` is exempted from schema/Rust parity.

## Design decisions and assumptions

- **Envelope semantics.** `resolved` requires a result; `abstained` and `escalated` carry neither result nor error (never a fabricated answer); `denied` requires a typed error. This is enforced identically in the response schema (if/then) and the Rust validator.
- **Bounded enums, fail closed.** v1 `worker_kind` is limited to `skill_router` and `risk_judge`; new workers require a version change, not silent extension. Unknown operation/worker/result kinds, dispositions, trust tiers, backends, and error codes all produce typed errors.
- **Wire error codes.** Mirror the typed Rust errors plus the wire-only `denied_by_policy`. There is deliberately no `unsupported_version` wire code: an unsupported version cannot be represented inside a v1 envelope; the validator reports it as a typed error outside the envelope.
- **Identifiers.** Canonical lowercase `8-4-4-4-12` UUID strings, validated without a `uuid` crate (no dependency needed).
- **Token budget.** `estimated_input_tokens` serializes the typed request as canonical JSON, removes only the declared `input_token_budget` field, counts the remaining Unicode characters, divides by four, and rounds up. This includes bounded metadata and payload strings instead of only selected user text. Serialization failure returns an unbounded estimate and fails closed. The valid boundary fixture equals the estimate exactly. This is a deterministic v1 heuristic, not a provider tokenizer.
- **Cross-envelope rules.** Response request linkage, evidence containment, candidate containment, extract-field evidence linkage, verdict evidence obligations (`supported`/`unsupported` require non-empty evidence refs), and latency/deadline accounting (`queue_ms <= request.queue_ms`; total latency <= queue + execution window) are checked by Rust against the paired request. Focused in-memory tests cover foreign evidence, candidate membership, and queue-deadline overrun; committed wire fixtures all maintain schema/Rust acceptance parity.
- **Score scale.** v1 fixes one scale, `unit_interval`, with score and threshold in `[0, 1]`.
- **Extract fields.** Scalar values only (string/number/boolean/null), each linked to an evidence reference that must exist in the request; strings bounded to 4096 characters. This keeps transcripts and unbounded payloads out of the protocol.
- **Schema layout.** Each canonical schema is self-contained (inline `$defs`) so it is standalone-resolvable offline without cross-file registry machinery; the shared definitions are intentionally duplicated between the two files and the contract test pins both files as the complete canonical set.
- **Rust validation flow.** Value-level fail-closed field checks (unknown/missing) → typed extraction → semantic validation, so every fixture failure class maps to a specific typed error instead of a raw serde message. The typed error set is `#[non_exhaustive]`-free but classified through `ErrorKind` for contract pinning.
- **No authority anywhere.** No field, type, or code path grants execution, authorization, or capability; confidence and trust tier are separate types with no conversion path.

## Dependencies

- Production: `serde` (derive) and `serde_json`; already cached and locked from BASE-01 (lockfile pins `serde 1.0.229`, `serde_json 1.0.151`). Required for JSON parsing and deterministic serialization.
- Test-only: exact-pinned `jsonschema 0.33.0`, with default features disabled, for canonical Draft 2020-12 schema validation. Its source and full dependency tree compiled in an offline scratch probe; no network was used.
- The worker worktree's root lock diff added the `zhoor-protocol` member and 88 dependency packages (829-line diff). The worker did not hand-edit, stage, or commit the lockfile. The controller generated the canonical root lock offline under Rust 1.98.1 and committed it separately as `b6b4419e2f56c0efb77a0974c71f1c0b1ead384d` (`chore(protocol): reconcile workspace lockfile`). The delta is the package and the pinned test-only `jsonschema 0.33.0` dependency graph; no `reqwest` or production schema runtime is present.
- No other direct dependency. No async runtime, framework, database, model SDK, HTTP/file schema reference resolver, or production JSON Schema runtime.

## Validations

**Worker-session validation:** not run because this contained Pi session had no shell tool. The controller ran the following offline commands from the worker worktree; all passed after controller-owned audit corrections:

1. `cargo fmt --all -- --check` — PASS
2. `cargo clippy -p zhoor-protocol --all-targets --offline -- -D warnings` — PASS
3. `cargo check -p zhoor-protocol --all-targets --offline` — PASS
4. `cargo test -p zhoor-protocol --offline` — PASS (15 unit tests, 9 contract tests)
5. `cargo test --offline -p zhoor-protocol --test protocol_contract` — PASS (9 tests)
6. `cargo build -p zhoor-protocol --offline` — PASS
7. `cargo build -p zhoor-protocol --release --offline` — PASS

Controller corrections after the worker returned: added the test-only schema dependency and strict per-fixture schema/Rust parity; moved request-paired semantic checks to focused Rust tests so every committed valid/invalid wire fixture agrees with its schema; fixed Rust nested result-object parsing; repaired valid evidence references and request/response pairing; bounded over-maximum budgets as typed `OverBudget`; and changed the input estimate to cover the serialized request envelope. Formatting was applied only to `crates/protocol` Rust files.

**Controller integration and canonical validation:** the implementation commit `6420ec8544382fd4a89ce2954df0bd5670a546d2` is a fast-forward child of assignment SHA `eff8db3bb811e2943da17b95394aaa4e8c7df6ea`. Controller lock reconciliation is commit `b6b4419e2f56c0efb77a0974c71f1c0b1ead384d`. The final locked validation sequence passed:

1. `./scripts/validate.sh --package zhoor-protocol` — PASS (format, Clippy, check, unit/contract tests, debug and release package builds).
2. `cargo test --offline --locked -p zhoor-protocol --test protocol_contract` — PASS (9 tests).
3. `./scripts/validate.sh` — PASS (full workspace format, Clippy, checks, tests, docs, and debug/release builds).

The root `Cargo.lock` SHA-256 was `517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40` before and after the locked sequence.

## Scope check

- Files outside the declared write scope changed: no.
- Direct production dependencies: `serde`/`serde_json` (already locked and cached). Direct test-only dependency: exact-pinned `jsonschema 0.33.0`, default features disabled; full graph verified offline.
- No network access, installs, credentials, telemetry, background services, or Git operations performed.

## Incident entry (sanitized)

Before source assignment, the OpenCode worker lane auto-started its configured external MCP processes, and lifecycle process inspection exposed a credential-bearing remote URL in process metadata. No PRO-01 source or task packet had been sent to that lane. The lane was stopped, its child processes were verified gone, the affected credential was treated as exposed, and the owner completed rotation/revocation. The lane is quarantined for this milestone; routing moved to this isolated Pi/OpenRouter/GLM lane, and the `AGENT_CONTAINMENT=PASS` gate passed before this packet. No secret, URL, token fragment, header, or credential prefix/suffix is reproduced anywhere in this handoff or in any task artifact.

## First exact-SHA review and correction

The first Luna review session returned `BLOCKED` at candidate `ef89714337ba6f364b97eba1e06263fd430a3b71`; its in-session report is preserved under the assigned run root. It identified two real parity gaps: Rust accepted repeated `choice` values although the response schema requires unique items, and paired-response validation did not require the result kind to match the request operation. The report also incorrectly inferred an attached checkout from `git show --decorate`; controller verification showed `git branch --show-current` empty and `git worktree list --porcelain` marked the exact candidate detached. The reviewer session was stopped safely before corrections.

A separate fresh role-only reviewer launch was stopped before source because its profile referenced a shared multi-provider auth store. No repository source or review packet was sent to that session. The active review profile was corrected to expose only the selected OpenAI/Codex provider entry through an anonymous memory descriptor; no credential was copied into a prompt, packet, repository file, or persistent runtime file. The corrected session passed the containment gate before receiving source.

Controller corrections reject duplicate choices with a typed result error and enforce the `select`/`choice`, `score`/`score`, `extract`/`extract`, and `verify`/`verdict` mapping for resolved responses. A duplicate-choice invalid fixture and focused paired-envelope test cover those changes. Candidate `88459e89d409ebb3c5689e0b37db97e08aca3c8e` received `CHANGES_REQUIRED` from a contained Luna review. Its report and validation results are preserved under the run root. The two verified findings were Rust numeric parsing of schema-valid integer-valued decimals and an overstrict `max_choices <= candidates.len()` Rust condition. Controller correction now accepts finite nonnegative integral decimal representations within `u64`, removes the overstrict candidate-count condition because `max_choices` is an upper bound and returned choices must still belong to the supplied candidates, and adds valid fixtures for both cases.

The reviewer also noted that schema `uniqueItems` compares whole evidence objects, while Rust rejects repeated content-addressed `ref` identities even when trust/provenance metadata differs. This check is retained as an intentional Rust semantic safeguard against conflicting trust/provenance for the same content hash. Draft 2020-12 has no standard unique-by-property array constraint; the request schema now states this boundary, and a contract test demonstrates schema shape acceptance alongside typed Rust semantic rejection. The required committed valid/invalid fixture inventory still has schema/Rust accept/reject parity.

The updated source passed canonical locked validation: `./scripts/validate.sh --package zhoor-protocol` (15 unit tests and 10 contract tests PASS), `cargo test --offline --locked -p zhoor-protocol --test protocol_contract` (10 tests PASS), and `./scripts/validate.sh` (full workspace PASS). The root `Cargo.lock` SHA-256 was `517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40` before and after the full locked sequence. Task-owned `git diff --check` passed. A new candidate and focused Luna re-review are pending.

## Assumptions to confirm at review

- The bounded v1 `worker_kind` set (`skill_router`, `risk_judge`) matches the current worker roster; adding a worker is a deliberate protocol version event.
- The serialized-JSON character estimate divided by four is a deterministic v1 budget heuristic, not a provider tokenizer. It includes all request fields except the declared budget field.
- `session_ref` is a bounded opaque label (not forced to UUID form) because the architecture types it as a plain string.
- Wire error codes on a denied response are descriptive; policy evaluation itself belongs to the policy card, and this crate performs no policy decisions.

## Residual risks

- Worker-side schema/Rust parity and offline validation passed; canonical locked package/contract/full validation, controller lock reconciliation, and lock stability passed after both correction rounds. The reviewer-requested property-level evidence identity check remains a documented deterministic Rust semantic rule because Draft 2020-12 cannot express unique-by-property semantics in an array; a focused test pins this behavior. Independent exact-SHA PASS remains pending.
- No Unicode boundary fixture exercises strings near schema length limits; the Rust validator counts Unicode scalar values and this should be checked during exact-SHA review if it is material.
- Cross-envelope consistency is intentionally a Rust semantic check because it depends on the paired request. Focused tests exercise foreign evidence, candidate membership, verdict evidence, and deadline accounting.
- Fixture request/response identifiers are synthetic and deterministic; real provenance values belong to later runtime cards.

## Recommended next action

Exactly one next action: commit the validated correction round as a new candidate, then have the same contained Pi/GPT-6 Luna reviewer perform a focused re-review against a clean detached checkout of that exact SHA.
