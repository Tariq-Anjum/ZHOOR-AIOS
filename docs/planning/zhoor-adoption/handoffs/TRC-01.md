# TRC-01 implementation handoff

## Run and assignment

- RUN_ID: ZHOOR-20260930T043222Z-TRC-01
- Card: TRC-01
- Route: controller-authored by Codex / GPT-6 Luna / high in an isolated implementation worktree.
- Starting and assignment SHA: 3d1c52c52d3d1a6f94deb0e1b48c0813c6b6b049
- Work branch: work/trc-01-20260930043222
- Worktree: /tmp/zhoor-trc-01/ZHOOR-20260930T043222Z-TRC-01/worker
- Worker TASK_ID: none; no implementation subordinate was created.
- Board status at start: TRC-01 = ASSIGNED; PRO-01 = DONE; other Wave-5 cards remained BLOCKED.

## Implementation

- Added zhoor-trace with validated trace creation from protocol Request / Response values.
- Added the sole canonical trace v1 schema, valid/invalid contract fixtures, and a replay request fixture.
- Trace payloads retain typed operation/worker/model/policy/registry versions, route, result kind, confidence, sanitized evidence references, separate queue/load/inference latency, disposition, typed error code, optional content-addressed receipt reference, timestamp, and domain-separated input/response/calibration/policy hashes.
- Raw goal text, provenance/source labels, evidence bodies, result values, uncertainty text, calibration references, and error messages are omitted. Policy references are stored as a domain-separated hash.
- Replay takes the original request JSON from the caller, validates it through zhoor-protocol, and checks trace version, request identity, worker/operation, policy-reference hash, and normalized input hash before returning the typed request. The trace itself does not persist replay text.
- Assumptions: the caller provides version-token metadata from trusted runtime context; timestamp is supplied by the caller in Unix milliseconds; receipts use receipt://sha256/<64 lowercase hex>.
- Dependency: sha2 = 0.10.9 for the required SHA-256 contract. It was already cached locally; all Cargo commands remained offline.
- Scope: implementation changes are confined to crates/trace/**, schemas/trace/**, fixtures/trace/**, and this handoff. Protocol files are read-only. No root manifest or unrelated component path changed.

## Pre-integration validation

All commands used the installed Rust 1.98.1 toolchain and --offline; pre-integration commands omit --locked as required.

- cargo fmt --all -- --check — PASS
- cargo clippy -p zhoor-trace --all-targets --offline -- -D warnings — PASS
- cargo check -p zhoor-trace --all-targets --offline — PASS
- cargo test -p zhoor-trace --offline — PASS (4 unit, 9 contract, 0 doctests)
- cargo test --offline -p zhoor-trace --test trace_contract — PASS (9 contract tests)
- cargo build -p zhoor-trace --offline — PASS
- cargo build -p zhoor-trace --release --offline — PASS
- git diff --check — PASS
- Schema fixtures are checked against Draft 2020-12 and the Rust parser; replay checks normalized-input equality, response linkage, sanitization, and error-code-only denial traces. Contract tests also check exact confidence-decimal bounds, the timestamp u64 boundary, required nullable fields, operation/result compatibility, and revalidation of public protocol structs.

## Independent review and correction

- First substantive exact-SHA review: Pi / OpenAI-Codex / GPT-6 Luna / xhigh, read-only, on candidate `aab713dc60d54239d38deea2338c547a38e80827`.
- Result: CHANGES_REQUIRED. The reviewer found that out-of-range confidence decimals could round into range during f64 parsing, and that the timestamp schema lacked the Rust u64 maximum.
- Corrections: `from_json_str` now checks the original arbitrary-precision confidence decimals against [0, 1] before f64 deserialization; the schema caps timestamps at 18446744073709551615. Added boundary contract coverage for confidence and timestamps.
- Second exact-SHA review: fresh contained Pi / OpenAI-Codex / GPT-6 Luna / xhigh reviewer REV3 reviewed `e69911b8f764a6ec1440ad0b7d5cc18e9580c02a` and returned CHANGES_REQUIRED. Findings: absent nullable fields were accepted by Rust despite schema-required keys; operation/result-kind compatibility was not enforced by trace parsing/schema; public protocol structs could be mutated to invalid values before `create`.
- Additional corrections: `from_json_str` now requires the nullable schema keys to be present; Rust and JSON Schema enforce the operation/result-kind mapping; `create` round-trips typed request/response values through the protocol `from_value` validators before cross-envelope validation and hashing. Added regression coverage for all three findings.
- The initial reviewer session returned BLOCKED because its persistent system prompt prohibited source inspection; it was stopped before any source review. A fresh contained reviewer session performed the substantive review. No source changes were made from the blocked session.
- First correction commit `77c1b1d` passed canonical locked validation. Second correction commit `1c54cb0` is integrated and also passed canonical locked validation; a fresh exact-SHA review of the second correction is pending.

## Root lockfile

Cargo mechanically updated only the isolated worktree Cargo.lock while resolving the new workspace member and SHA-256 dependency. The observed delta adds zhoor-trace, sha2 0.10.9, and its six transitive packages. The worktree lock SHA-256 is 4585fbd4546d6b79045c87a83f4796dafe5144ff628cb4cb5fd297bc4f1a2150; the assignment baseline was 517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40. Cargo.lock remains unstaged and uncommitted for controller reconciliation after source integration.

## Risks and stop conditions

- Plain SHA-256 digests are integrity/equality identifiers, not anonymization; low-entropy inputs can be guessed offline. Raw inputs remain outside the trace and are caller-supplied for replay.
- Replay requires the original request JSON to be available to the caller; a trace alone intentionally cannot recover free-form input.
- No STOP condition was encountered. No network access, download, install, credential access, external MCP, or background process was used.

## Pre-integration action (completed)

The controller committed only card-owned implementation and handoff paths, fast-forward integrated the source, reconciled the canonical root lockfile mechanically offline, and passed the exact locked targeted and full validation commands.

## Canonical integration checkpoint

- Initial source integration: fast-forward from assignment SHA 3d1c52c52d3d1a6f94deb0e1b48c0813c6b6b049 to implementation SHA 286e42580d547add2bf62f64094c8ef5e80b0cb2.
- Root lock reconciliation: controller commit 9dff79a692b6e1dc50cc2699073202fe9343f89f; mechanical offline update only. Cargo.lock SHA-256 is 4585fbd4546d6b79045c87a83f4796dafe5144ff628cb4cb5fd297bc4f1a2150.
- Correction commit `77c1b1d` fixed exact confidence decimal validation and bounded the trace timestamp schema at `u64::MAX`. Correction commit `1c54cb0` requires nullable keys, validates operation/result-kind mappings in Rust and schema, and revalidates public protocol structs before trace creation.
- Final `./scripts/validate.sh --package zhoor-trace` — PASS; 4 unit and 9 contract tests, locked clippy/check, debug and release package builds.
- Final `cargo test --offline --locked -p zhoor-trace --test trace_contract` — PASS; 9 tests.
- Final `./scripts/validate.sh` — PASS; full locked workspace validation exit 0, including 17 protocol unit + 11 protocol contract, 11 system-info unit, 4 trace unit + 9 trace contract tests, clippy/check, and debug/release builds.
- Cargo.lock SHA-256 before and after corrected locked validation: 4585fbd4546d6b79045c87a83f4796dafe5144ff628cb4cb5fd297bc4f1a2150 (unchanged).
- Protected baseline reconciliation: PASS; all 28 status/path/SHA-256 records still match.
- Repository-wide git diff --check reports trailing whitespace in pre-existing protected dirty job-card files, including jobs/TRC-01.md. Those files were preserved. git show --check on the TRC-01 implementation and lock reconciliation commits passes.
- Board state remains TRC-01 = REVIEW; PRO-01 = DONE; REG-01/POL-01/SEC-01 = BLOCKED. No later card has started.

## Review handoff

- The first substantive exact-SHA review was performed by a fresh, contained, read-only Pi / OpenAI-Codex / GPT-6 Luna / xhigh session against candidate `aab713dc60d54239d38deea2338c547a38e80827` and returned CHANGES_REQUIRED for confidence decimal rounding and the timestamp schema bound.
- A preceding reviewer session returned BLOCKED because its persistent system prompt prohibited source inspection; it was stopped before inspecting source. A fresh session performed the substantive review.
- The first corrected source candidate was `77c1b1d`; the second correction is integrated as `1c54cb0` and canonical validation passes. After this handoff-only commit, freeze the resulting HEAD as the next exact review candidate.
- Next: create a fresh contained reviewer lane for that exact candidate. The reviewer must not claim to execute Cargo checks; canonical validation evidence is controller-owned. Close TRC-01 only after exact-SHA review PASS and all other gates pass; do not start a later card.
