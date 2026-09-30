# SEC-01 independent security review — Luna r5

## Review identity

- Role: independent exact-SHA security reviewer.
- Route: fresh Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only.
- Herdr role/name: `sec-01-final-independent-reviewer-r5` / `sec01reviewr5`.
- Containment: `AGENT_CONTAINMENT=PASS`; actual `! /usr/bin/printf ZHOOR_REVIEW_R5_SENTINEL` probe in this lane returned the fixed shell-prohibition result with exit 126 and no sentinel output; one workload pane with managed Sidebar only; clean detached checkout; model tools restricted to `read`, `grep`, `find`, `ls`; private scratch mode 0700; immutable guard copy mode 0400 with SHA-256 `fc5f4ad477bb389eec6550996d6c9dda4950132b3b26dbc46aae1d8c6ecd51b9`; no credential values inspected or logged. Controller regression evidence for the same guard also records in-worktree read/grep/find/ls allowed, outside-worktree read blocked, and no write/bash model tools.
- Reviewed candidate SHA: `7783d20b350290a22f197d13923e127767c13ab5`.
- Review source bounds: SEC/GRANT job cards, architecture section 8 lines 183–214 only, ADR-0004, D-024/D-025, SEC contract/schema/Rust/tests/fixtures/handoff, r1/r3 reports, and protocol evidence reference primitive.
- Verdict: `PASS`.
- Reviewer did not run Cargo, Git, or other commands and did not independently verify the candidate hash.

## Findings

None.

## Review confirmations

- The r1 fixes are present: typed parsing before generic JSON-value parsing; duplicate-member regressions; frozen transcript golden vector and mutation/evidence-order coverage; serialized UID claim wording distinguishes it from separately supplied peer identity.
- Both r3 findings are resolved. Evidence is a bounded closed map keyed by canonical references. Rust rejects malformed keys, unknown metadata, excess entries, and duplicate decoded raw keys including escaped spellings. The schema and Rust align on evidence map shape, key grammar, and the 1–64 bound. JSON Schema cannot inspect duplicate raw object names after JSON data-model parsing; the Rust parser rejects them. Descriptor control rules match through U+009F.
- Alternative B is faithful: authenticator envelope is required and bounded; structure, issuer ID, peer caller authentication, and digests do not prove grant authenticity. The frozen transcript binds all claims and authenticator format. Unsupported formats fail closed at the verifier boundary; GRANT-01 can implement the selected mechanism within the reserved envelope without a SEC v1 schema change.
- Arguments are exactly `{}` with normalized UTF-8 bytes `{}`; SHA-256 is binding only. The sole scope is fixed non-path descriptor metadata; path-shaped fixtures make no filesystem canonicalization claim.
- `trusted_now >= expires_at` is expired and replay state belongs to GRANT-01. Bounds and unknown-field/version rejection are specified. Receipts record execution/observation, not postcondition proof; failed and unknown/unverifiable postconditions remain ineligible.
- No daemon, executor, grant issuer, trust root, cryptographic verifier, replay store, socket runtime, privilege/multi-user policy, or filesystem enforcement is implemented.

## Validation independently run

| Command/check | Result |
|---|---|
| Read-only source/contract/security review at exact candidate SHA | PASS, no findings |
| Cargo / Git / shell commands | Not run |

## Residual risk

SEC-01 does not verify authenticator authenticity, enforce runtime policy/replay, resolve host filesystem paths, or verify postcondition evidence. These remain later GRANT-01/enforcement ownership. The review does not claim the candidate SHA independently verified; controller pins the detached checkout and owns validation evidence.
