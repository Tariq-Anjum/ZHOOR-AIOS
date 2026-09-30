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
- cargo test -p zhoor-trace --offline — PASS (4 unit, 5 contract, 0 doctests)
- cargo test --offline -p zhoor-trace --test trace_contract — PASS (5 contract tests)
- cargo build -p zhoor-trace --offline — PASS
- cargo build -p zhoor-trace --release --offline — PASS
- git diff --check — PASS
- Schema fixtures are checked against Draft 2020-12 and the Rust parser; replay checks normalized-input equality, response linkage, sanitization, and error-code-only denial traces.

## Root lockfile

Cargo mechanically updated only the isolated worktree Cargo.lock while resolving the new workspace member and SHA-256 dependency. The observed delta adds zhoor-trace, sha2 0.10.9, and its six transitive packages. The worktree lock SHA-256 is 4585fbd4546d6b79045c87a83f4796dafe5144ff628cb4cb5fd297bc4f1a2150; the assignment baseline was 517b2bdab93379075eceb07fb1170360e2d7af7cb2c0ce2469fedc9179af2c40. Cargo.lock remains unstaged and uncommitted for controller reconciliation after source integration.

## Risks and stop conditions

- Plain SHA-256 digests are integrity/equality identifiers, not anonymization; low-entropy inputs can be guessed offline. Raw inputs remain outside the trace and are caller-supplied for replay.
- Replay requires the original request JSON to be available to the caller; a trace alone intentionally cannot recover free-form input.
- No STOP condition was encountered. No network access, download, install, credential access, external MCP, or background process was used.

## Next action

Commit only card-owned implementation and handoff paths, fast-forward integrate the source, reconcile the canonical root lockfile mechanically offline, then run the exact locked targeted and full validation commands before moving the board to REVIEW.
