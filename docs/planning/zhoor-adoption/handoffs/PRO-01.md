# PRO-01 implementation handoff

## Status

`architecture/security adjudication complete; candidate 209d1fbb38fd7f23b763ad3c3048f46f998bf697 validated offline and exact-SHA reviewed PASS; board closure and publication pending`

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

Versioned v1 request/response/evidence/confidence/disposition/result-kind/deadline/error contracts under `schemas/protocol/**`; deterministic Rust semantic validation in `zhoor-protocol` under `crates/protocol/**`; positive/negative fixtures under `fixtures/protocol/valid/**` and `fixtures/protocol/invalid/**`; and a `protocol_contract` integration target enumerating every committed fixture with canonical Draft 2020-12 schema expectation, Rust expectation, and accept/reject agreement. Controller source audit corrections and implementation integration are complete. The fifth Luna exact-SHA review returned `CHANGES_REQUIRED` for typed error behavior on exponent-overflow score/confidence inputs. The correction is applied but unvalidated; work is paused for architecture/security review after two validation-design failures. No transport, server, model integration, execution grant, policy engine, registry, trace implementation, backend API, database, or async framework.

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

The second Luna re-review at `d3d8dabb2a17c8d51bc75c7b08491ed9f0215dbd` returned `CHANGES_REQUIRED` for one narrower numeric-parity issue: parsing through `f64` could round `800.000000000000001` to an integer. The reviewer accepted the max-choice clarification and the documented Rust-only duplicate-evidence-identity semantic check. The reviewer’s report and exact-SHA evidence are preserved under the run root; its emitted `RUN_TASK` string omitted `PRO01`, and the capture records the exact assigned task ID alongside that raw report text.

The final numeric correction enables serde_json `arbitrary_precision` for the protocol crate, parses the preserved decimal token exactly for unsigned integer fields, and adds invalid fixture `request-fractional-near-integer-budget.json`. Unit tests cover integral decimal/exponent forms, fractional forms, negative values, and out-of-range values. The cached `jsonschema 0.33.0` crate implements `type: integer` via `f64::fract()`, so its standalone validator accepts this one high-precision fractional token after rounding even with serde_json arbitrary precision. To keep the contract result faithful to Draft 2020-12 without replacing the full schema validator, the fixture suite supplements only exact mathematical integrality for schema nodes typed `integer`; the cached library continues to validate all other schema keywords.

The updated source passed canonical locked validation: `./scripts/validate.sh --package zhoor-protocol` (16 unit tests and 10 contract tests PASS), `cargo test --offline --locked -p zhoor-protocol --test protocol_contract` (10 tests PASS), and `./scripts/validate.sh` (full workspace PASS). The root `Cargo.lock` SHA-256 was `517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40` before and after the full locked sequence. Task-owned `git diff --check` passed. A new candidate and focused Luna re-review are pending.

## Fourth exact-SHA review and correction

The fourth fresh Pi/Luna review at `4cd48269b5c7312fdceaaae3131bf325b8d6ca2e` returned `CHANGES_REQUIRED`. It confirmed the exact unsigned-integer correction and test-only integer-schema supplement, but found the same `f64` boundary-rounding class in score and confidence fields: values such as `1.00000000000000001` and tiny negative decimals could round onto the inclusive `[0,1]` endpoints before Rust checked their bounds. The full sanitized report is preserved under the run root; the reviewed SHA matched exactly. The reviewer process was stopped with the verified Pi exit key, and no Pi process remained in the reviewer worktree (three idle Herdr shell panes remain).

Controller correction adds exact decimal-token comparison against `[0,1]` before score/confidence values are converted into the typed `f64` fields. Four invalid response fixtures cover score and confidence just above one and just below zero. The contract suite supplements only the schema's current `type: number`, `minimum: 0`, `maximum: 1` nodes because cached `jsonschema 0.33.0` uses `f64` for those comparisons; full Draft 2020-12 validation remains enabled. Unit tests cover endpoints, integral exponent forms, high-precision fractions, tiny negatives, and extreme exponents. Canonical package validation passed with 17 unit and 10 contract tests; standalone contract validation passed with 10 tests; full workspace validation passed. The root lock SHA remained `517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40`. The controller is preparing a new candidate for focused re-review. No dependency or schema-language change was made.

## Fifth exact-SHA review and correction

The fifth fresh Pi/Luna review at `480d4e801e53480beced1a609622a027d3806828` returned `CHANGES_REQUIRED`. It found score and confidence validators called `f64_field` before checking exact bounds. Inputs such as `1e400` and `-1e400` therefore failed as `MalformedJson` when conversion was non-finite, instead of the protocol's typed `InvalidResult` or `InvalidConfidence` bound errors. The exact comparator itself passed review for signs, signed zero, decimal positions, trailing zeros, and exponent behavior. The sanitized report is preserved under the run root; its emitted RUN_TASK omitted `PRO01`, while the exact assigned task ID was `ZHOOR-20260929-PRO01-PROTOCOL-01-REVIEWER-LUNA-05`. The Pi process was stopped using the verified pane exit key and then verified gone; old idle Herdr shell panes remain.

The follow-up checks exact unit-interval bounds before converting accepted values to `f64`, preserving malformed-type errors and returning typed bounds errors for exponent-overflow numbers. Two additional invalid fixtures exercise overflow in calibrated confidence and score threshold, pinning `InvalidConfidence` and `InvalidResult`. The exact comparator unit test covers positive and negative overflow exponents. No new package or root lock change is expected. Canonical locked validation and focused Luna re-review are pending.

### Validation-design stop and architecture adjudication

The follow-up correction was uncommitted and had not passed canonical validation when the stop was entered. The first attempt at `./scripts/validate.sh --package zhoor-protocol` reached `protocol_contract` and panicked inside cached `jsonschema 0.33.0` numeric handling (`numeric.rs:27`, “Always valid”) on exponent-overflow JSON. After adding a test-only normalization path so the full validator could inspect other keywords, the focused contract test failed because `response-confidence-exponent-overflow.json` also contained a malformed evidence hash (58 hexadecimal characters instead of the schema-required 64). The adjudication-authorized retest exposed this independent fixture defect; the two exponent-overflow fixtures and three existing near-boundary score/confidence fixtures now use valid 64-character synthetic evidence hashes so each exact numeric invalid fixture has no unrelated schema violation. The full validation sequence for the correction was not completed at the stop checkpoint. `Cargo.lock` remains at SHA-256 `517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40`.

Independent architecture/security adjudication was completed by fresh contained Pi/GPT-6 Luna xhigh lane `pro01-arch-review` at `ff434f70d64bf9bb9f0728e0f36a1d1361d98955`. Evidence: `/tmp/zhoor-pro01/ZHOOR-20260929-PRO01-PROTOCOL-01/architecture-adjudication-luna.md`. It concluded that Draft 2020-12 numeric assertions apply to mathematical values; `jsonschema 0.33.0` can compile/meta-validate schemas and validate where its numeric behavior is reliable, but is not an arbitrary-precision oracle and a panic is not rejection. A sound minimal test design is an explicit inventory of numeric schema nodes with pinned type/bounds and exact checks on original fixture values. For finite exact violations, the library may still be called but its rounded numeric result cannot override the exact check. For non-finite values it cannot represent, replace only inventoried numeric leaves with in-range sentinels in a clone, require full schema validation of the clone, and separately assert exact numeric rejection and Rust's typed error against the original. This is a hybrid test result, not direct jsonschema validation of the original overflow value. The previous generic partial schema walker has been replaced; a dedicated test pins the numeric schema inventory and bounds.

The reviewer noted that accepted in-range score/confidence values are represented as Rust `f64`, so some tiny or near-endpoint decimals can round after exact raw-value bounds validation. The controller resolves this as the existing v1 typed representation: exact wire-value preservation is not a protocol requirement, while exact inclusive `[0,1]` checking before conversion is. No second challenger is required. Any future exact-decimal downstream requirement needs an explicit protocol change. No production validator, schema, acceptance expectation, dependency, or network behavior is being changed.

The current correction retains the two exponent-overflow invalid fixtures and typed expectations: confidence overflow → `InvalidConfidence`; score/threshold overflow → `InvalidResult`. The validator supplement now has an explicit inventory of bounded numeric schema nodes with pinned types/ranges; finite exact violations are decided by the exact check, and non-finite values are tested through a clone that replaces only inventoried leaves with safe sentinels and must pass full schema validation. The originals remain subject to exact checks and Rust typed errors. All five exact numeric invalid fixtures now have valid 64-character evidence hashes so the intended numeric bound/integrality violation is their only schema failure.

## Post-adjudication validation checkpoint

Canonical validation passed after the test-only numeric design correction:

- `./scripts/validate.sh --package zhoor-protocol` — PASS; 17 unit tests and 11 contract tests, clippy/check, debug and release package builds.
- `cargo test --offline --locked -p zhoor-protocol --test protocol_contract` — PASS; 11 contract tests.
- `./scripts/validate.sh` — PASS; full workspace tests, clippy/check, and debug/release builds, all offline and locked.
- `git diff --check` over the tracked PRO-01 correction files — PASS.
- Overflow fixture JSON parses and each evidence reference has exactly 64 hexadecimal characters — PASS.
- Root `Cargo.lock` SHA-256 before and after validation: `517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40` (unchanged).
- Protected baseline reconciliation: 28/28 status/path/SHA-256 records match. Nine non-protected dirty paths remain in the PRO-01 correction scope: three Rust files, five numeric fixtures, and the PRO-01 handoff.
- Board remains `PRO-01 = REVIEW`; no later card started.

Following this validation checkpoint, the controller committed the nine exact PRO-01 correction paths as candidate `209d1fbb38fd7f23b763ad3c3048f46f998bf697` without rewriting prior history. A fresh contained Pi/GPT-6 Luna xhigh reviewer returned PASS with no findings at that exact SHA. The controller independently verified the detached review checkout SHA because the reviewer guard correctly denied `.git/HEAD`; the reviewer did not rerun commands. Topology policy/containment commits are integrated in the candidate's ancestry through `ff434f70d64bf9bb9f0728e0f36a1d1361d98955`. A later controller-only auth-handoff helper commit `5f4f474757b8204abb8f6969ada9122c08beea71` is on the integration branch after the candidate and changed only `scripts/zhoor_herdr.py` and its tests. Herdr topology evidence is recorded in `/tmp/zhoor-herdr-topology/topology-test-evidence.md`.

## Final exact-SHA acceptance review

- Candidate: `209d1fbb38fd7f23b763ad3c3048f46f998bf697`; detached review checkout matched that exact SHA and was clean.
- Reviewer: fresh Pi 0.87.1 lane `pro01-review-luna-08`, controller-routed `openai-codex` / GPT-6 Luna / xhigh; `AGENT_CONTAINMENT=PASS` before the packet was sent.
- Herdr topology: tab `wG:tQ`, interactive pane `wG:p1N`, managed Sidebar `wG:p1P`; focus restored to controller tab `wG:tE` and remained there. Pi exited with ctrl+d and live agent absence was verified; the completed tab is retained.
- Verdict: `PASS`; findings `NONE`. The reviewer examined the numeric boundary/error behavior, exact test-only schema supplementation, fixtures, protocol semantics, and handoff.
- Review evidence: `/tmp/zhoor-pro01/ZHOOR-20260929-PRO01-PROTOCOL-01/review-result-209d1fb.md`; containment evidence: `/tmp/zhoor-pro01/ZHOOR-20260929-PRO01-PROTOCOL-01/review-containment-pass-209d1fb-08.md`.
- Limitation: the reviewer could not inspect `.git/HEAD` or rerun validation because it had read-only file tools. The controller independently verified the exact checkout SHA and supplied the canonical offline/locked validation results recorded above.

## Assumptions to confirm at review

- The bounded v1 `worker_kind` set (`skill_router`, `risk_judge`) matches the current worker roster; adding a worker is a deliberate protocol version event.
- The serialized-JSON character estimate divided by four is a deterministic v1 budget heuristic, not a provider tokenizer. It includes all request fields except the declared budget field.
- `session_ref` is a bounded opaque label (not forced to UUID form) because the architecture types it as a plain string.
- Wire error codes on a denied response are descriptive; policy evaluation itself belongs to the policy card, and this crate performs no policy decisions.

## Residual risks

- Canonical locked package/contract/full validation and `git diff --check` passed after the fifth correction at the exact candidate SHA. The independent exact-SHA review returned PASS with no findings. The property-level evidence identity check remains a documented deterministic Rust semantic rule because Draft 2020-12 cannot express unique-by-property semantics in an array; a focused test pins this behavior. The cached test validator's `f64` implementations of integer and unit-interval boundaries are supplemented by narrow exact-decimal checks in the contract suite.
- No dedicated Unicode boundary fixture exercises strings near schema length limits. The reviewer found no blocking issue; Rust counts Unicode scalar values. This remains a non-blocking coverage limitation.
- Cross-envelope consistency is intentionally a Rust semantic check because it depends on the paired request. Focused tests exercise foreign evidence, candidate membership, verdict evidence, and deadline accounting.
- Fixture request/response identifiers are synthetic and deterministic; real provenance values belong to later runtime cards.

## Recommended next action

Exactly one next action: record the exact-SHA PASS, transition PRO-01 from REVIEW to DONE in the board closure commit, reconcile the 28 protected records, verify the remote relationship, and publish only by normal fast-forward if the remote has not diverged. Stop after PRO-01; do not start a later card.
