# POL-01 — deterministic risk policy handoff

## Pinned v1 contract

- Policy version: 1.
- This package returns a PolicyAssessment containing risk level, scrutiny recommendation, evaluated/abstained state, advisory escalation, stable reason codes, and stable evidence requirements. It has no allow, deny, grant, or executable-request result.
- The sole known low-risk capability in v1 is the exact ID system.info.read. Unknown IDs abstain, recommend human review, and require verified evidence plus human review.
- The closed hard-risk facts are shell, package installation, service control, network egress, resource scope exceeded, privilege, and mutation. Any such fact yields high risk, human review, advisory escalation, and verified evidence requirements. No fact or evidence combination authorizes an operation.
- A known low-risk request requires observed evidence; verified evidence also satisfies that minimum. untrusted and policy tiers do not satisfy it.
- Missing eligible evidence adds missing_required_evidence, abstains, recommends advisory escalation, and adds a human-review requirement without lowering the risk classification.
- Semantic ambiguity raises low risk to medium and routine scrutiny to increased scrutiny. It leaves high and unknown classifications unchanged.
- The only v1 prompt-injection indicator is the typed upstream fact safeguard_bypass_instruction. It raises scrutiny to at least increased and recommends advisory escalation. POL-01 does not inspect raw text or claim to detect prompt injection.
- Hard-risk facts dominate ambiguity. An unrecognized capability abstains; it is unknown-risk when no hard fact applies. A simultaneous hard fact remains high-risk, with the unknown-capability reason and human-review scrutiny retained.
- Reason codes and evidence requirements are unique and ordered by their closed Rust enum order, independent of input order.
- Inputs are explicit normalized facts only. The crate does not parse commands, canonicalize paths, probe the host, fetch evidence, resolve secrets, consult models, or execute capabilities.
- JSON Schemas close both input and output structures. Capability IDs are 1–256 Unicode scalar values, have no leading/trailing whitespace or control characters, and Rust/schema boundary whitespace rules include U+FEFF consistently; C0, DEL, C1, and U+FEFF invalid fixtures pin the boundary classes. Inputs and assessments expose bounded `from_json_str` parsing entry points and do not implement public `Deserialize`, so callers cannot bypass the raw JSON byte limit and parser limits with direct serde entry points. Input JSON and assessment JSON are limited to 65,536 bytes. Input arrays are bounded to 7 risk signals, 1 prompt indicator, and 4 trust tiers; assessment arrays are bounded to 11 reasons and 3 evidence requirements, matching schema limits. Rust rejects duplicate input facts and duplicate assessment reason/evidence entries. Directly constructed input facts are bounded again in `evaluate`. Evidence duplicate validation is bounded and deterministic. Rust uses the protocol-owned TrustTier::parse for evidence tier vocabulary.
- The policy consumes existing protocol trust-tier terms without changing protocol contracts. It does not produce a protocol denied disposition.

## Architecture mapping

| Input fact | Classification and scrutiny | Reason | Evidence requirement |
|---|---|---|---|
| system.info.read, no other signal | low / routine | none | observed |
| Any other capability ID | unknown / human review; abstain and advise escalation | unknown_capability | verified and human review |
| Any hard-risk signal | high / human review; advise escalation | signal-specific code | verified and human review |
| Semantic ambiguity | at least medium / increased | semantic_ambiguity | base requirement remains |
| safeguard_bypass_instruction | at least medium / increased; advise escalation | safeguard_bypass_instruction | base requirement remains |
| Required trust tier absent | classification unchanged; abstain and advise escalation | missing_required_evidence | current tier plus human review |

## Implementation and validation status

- Assignment base: 9f88a8c248a0fc7e03b8f8f0efa916d6cca28b27.
- Starting local/remote SHA: 9e12c30cd6ec0a677d062d1bc529e5cd90db42fa; both matched before authorization.
- Owner authorization: D-023, commit cb8581948d0769295b937281b5cff299a5e4e506.
- READY to ASSIGNED: commit 9f88a8c248a0fc7e03b8f8f0efa916d6cca28b27.
- Implementation commits: 5fa9c2ebcb6ad3357cdb883084b8c9736b425b14 (initial policy) and f86c7431cae6ba4bbc4cd870d957dee89c75d10d (schema/Rust control-character parity correction).
- Root Cargo.lock reconciliation: commit b5ede62a51bf9c2ef62d660a14ffb58dd020927a; the reviewed diff adds only the zhoor-risk-policy package entry and references dependencies already locked.
- Worker-stage validation: PASS on Rust 1.98.1, offline: format, Clippy (-D warnings), check, package tests (10 unit + 3 contract), debug build, and release build.
- Canonical targeted validation: PASS after the control-character correction; ./scripts/validate.sh --package zhoor-risk-policy and cargo +1.98.1 test --offline --locked -p zhoor-risk-policy --test risk_policy_contract (3 contract tests). The C1-control invalid fixture is rejected by both schema and Rust.
- Full validation: PASS after the control-character correction; ./scripts/validate.sh.
- Task-owned diff check: PASS from assignment SHA through integrated implementation and lock reconciliation.
- Cargo.lock SHA-256 before reconciliation: 8f7a3f52711204eb3bc5ad0ddb81f5cead9184fee82cb6bc0b9f0669c153275a. Reconciled SHA-256: 2ead4aae23cd06bfb70b681acd6635a1fae685991e1d974a9fa9b14eda6c7f50; locked targeted/full validation left it unchanged.
- D-018: PASS, all 28 status/path/SHA-256 records matched after canonical validation.
- First reviewer attempt: candidate 551978760f12498248f63602bd2895dec93ef4a5; containment PASS, but the controller stopped the lane before verdict after identifying the C1 schema/Rust mismatch. It is not a review PASS and is not reused.
- Second exact-SHA review: candidate 6c9bb61bb667ff269ee1d1b0095f9ec4e0585b09; Pi/OpenAI-Codex/GPT-6 Luna/xhigh/read-only, fresh contained session; verdict CHANGES_REQUIRED. Findings: Rust assessment deserialization accepted duplicate reason/evidence entries rejected by schema; input parsing/evidence duplicate checking lacked explicit bounds and used quadratic scanning. Reviewer report: `docs/planning/zhoor-adoption/handoffs/reviews/POL-01-herdr-review-r2.md`; no Cargo commands were run by reviewer.
- Review corrections: commit 1051f3970b2b67c086eb1c2ef07153b2ea3bde1e adds duplicate-closed assessment deserialization and fixtures, matching array limits in schema/Rust, a 65,536-byte JSON parsing limit, and bounded deterministic duplicate checks. The corrected package checks, locked contract test, and full validation all passed; Cargo.lock remained unchanged at SHA-256 2ead4aae23cd06bfb70b681acd6635a1fae685991e1d974a9fa9b14eda6c7f50. Fresh exact-SHA review, closure, and publication: pending.
- Third exact-SHA review: candidate 233968080aed28cb8b8fa5efa3859774fc824419; Pi/OpenAI-Codex/GPT-6 Luna/xhigh/read-only, fresh contained session; verdict CHANGES_REQUIRED. Findings: public serde implementations bypassed limits before allocation; U+FEFF was rejected by schema but not Rust. Reviewer report: `docs/planning/zhoor-adoption/handoffs/reviews/POL-01-herdr-review-r3.md`; no Cargo commands were run by reviewer.
- Review corrections: commit 881385613d5f7faa1f7a56c8a687da609fac1611 removes public `Deserialize` paths and makes `from_json_str` the only JSON parsing entry point for both types, rejects U+FEFF boundary whitespace in Rust, adds matching assessment array limits, and adds parity/limit fixtures. The corrected package checks, locked contract test, and full validation all passed; Cargo.lock remained unchanged at SHA-256 2ead4aae23cd06bfb70b681acd6635a1fae685991e1d974a9fa9b14eda6c7f50. Fresh exact-SHA review, closure, and publication: pending.
- Fourth exact-SHA review: candidate a6b7893e0a6d8c1f51d46283cb922c0f9bbb546b; Pi/OpenAI-Codex/GPT-6 Luna/xhigh/read-only, fresh contained session; verdict CHANGES_REQUIRED because C0 and DEL parity classes were not fixture-pinned. Reviewer report: `docs/planning/zhoor-adoption/handoffs/reviews/POL-01-herdr-review-r4.md`; no Cargo commands were run by reviewer.
- Review correction: commit `274fd5b77689a4148fbe53d099aa0ef25598694d` adds invalid C0 and DEL capability fixtures; the contract inventory compares each invalid fixture against schema and bounded Rust parsing/evaluation. After this correction, `./scripts/validate.sh --package zhoor-risk-policy` passed with 12 unit tests and 3 contract test functions; the targeted locked contract test passed; full `./scripts/validate.sh` passed. Cargo.lock stayed at SHA-256 `2ead4aae23cd06bfb70b681acd6635a1fae685991e1d974a9fa9b14eda6c7f50` throughout locked validation. Fresh exact-SHA review PASS: candidate `9890525fe98e872b89c9f7d2750b89e087cb24e6`, Pi/OpenAI-Codex/GPT-6 Luna/xhigh/read-only; report `docs/planning/zhoor-adoption/handoffs/reviews/POL-01-herdr-review.md`. Reviewer ran no commands. Closure and publication: pending.

## Closure checkpoint

- Run ID: `ZHOOR-20260930T120000Z-POL-01`.
- Starting canonical local/remote SHA: `9e12c30cd6ec0a677d062d1bc529e5cd90db42fa`.
- Owner authorization: D-023, commit `cb8581948d0769295b937281b5cff299a5e4e506`; POL-01 BLOCKED → READY.
- READY → ASSIGNED commit: `9f88a8c248a0fc7e03b8f8f0efa916d6cca28b27`.
- Implementation commits: `5fa9c2ebcb6ad3357cdb883084b8c9736b425b14`, `f86c7431cae6ba4bbc4cd870d957dee89c75d10d`, `1051f3970b2b67c086eb1c2ef07153b2ea3bde1e`, `881385613d5f7faa1f7a56c8a687da609fac1611`, and `274fd5b77689a4148fbe53d099aa0ef25598694d`.
- Root Cargo.lock reconciliation: commit `b5ede62a51bf9c2ef62d660a14ffb58dd020927a`; worker-stage lock delta was not committed by the worker.
- Exact reviewed candidate: `9890525fe98e872b89c9f7d2750b89e087cb24e6`; PASS by fresh Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only review after containment PASS. Review report: `docs/planning/zhoor-adoption/handoffs/reviews/POL-01-herdr-review.md`. Reviewer did not run Cargo.
- Post-correction targeted validation: `./scripts/validate.sh --package zhoor-risk-policy` PASS (12 unit tests, 3 contract-test functions, format, Clippy, check, debug and release builds); `cargo +1.98.1 test --offline --locked -p zhoor-risk-policy --test risk_policy_contract` PASS (3 contract-test functions); full `./scripts/validate.sh` PASS. All Cargo work remained offline.
- Task-owned `git diff --check` from assignment through candidate: PASS.
- Cargo.lock SHA-256 before reconciliation: `8f7a3f52711204eb3bc5ad0ddb81f5cead9184fee82cb6bc0b9f0669c153275a`; reconciled value before/after all locked validation: `2ead4aae23cd06bfb70b681acd6635a1fae685991e1d974a9fa9b14eda6c7f50` (unchanged during locked validation).
- D-018: PASS, all 28 status/path/content-hash records reconciled immediately before closure.
- Board transition: POL-01 REVIEW → DONE in commit `9219483974ac46d464ccc93703e91bc29a4b9abc`. PRO-01, PRO-02, REG-01, and TRC-01 remain DONE; SEC-01, WRK-01, and WRK-02 remain BLOCKED. No later card was started.
- Herdr: five fresh read-only review sessions were created across correction rounds; the first was stopped before verdict, rounds 2–4 returned CHANGES_REQUIRED and were stopped/completed, and round 5 returned PASS and was stopped/completed. Only the Codex controller remains active.
- Closure transition commit: `9219483974ac46d464ccc93703e91bc29a4b9abc`; closure evidence handoff commit: `9de0215a45a2f1d768f2281318407d9922780eb0`. The closure checkpoint was normally fast-forward pushed after fetch/ancestry verification, and local/remote SHAs matched at that checkpoint. The subsequent handoff-only update follows the same normal fast-forward publication path. No force push, main promotion, tag, or release.
- Residual limitations: v1 only recognizes `system.info.read` as a known low-risk capability; signals and prompt-injection indicator are caller-supplied facts; evidence trust tiers do not authenticate evidence or prove source independence; policy output remains advisory.
- Explicit boundary: no grants issued, no authoritative execution denial invented, no secrets resolved, no capabilities executed, and no network/model dependency added.

## Limitations

- This v1 policy knows only system.info.read as a low-risk capability; other capability IDs abstain unless a later accepted policy version adds them.
- Risk signals and prompt-injection indicators are caller-supplied normalized facts. The crate does not infer them from request arguments or text.
- Evidence requirements classify trust-tier metadata only; they do not authenticate evidence, prove source independence, or establish authorization.
- Risk classifications and scrutiny recommendations are advisory. Policy authorization, grant handling, and enforcement belong to later components.
