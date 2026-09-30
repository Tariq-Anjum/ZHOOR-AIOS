# POL-01 independent review — round 4

- Candidate SHA: `a6b7893e0a6d8c1f51d46283cb922c0f9bbb546b`
- Reviewer route: Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only / fresh session
- Containment: PASS before the source-bearing review packet
- Verdict: `CHANGES_REQUIRED`
- Review scope: source and contract inspection only; the reviewer ran no commands or tests.

## Finding

C0 and DEL capability-boundary parity were not pinned by invalid fixtures. Existing fixtures covered ordinary spaces, C1, and U+FEFF, while the schema excludes C0 and DEL too. The contract test validates only fixture files present in the inventory. The reviewer cited `schemas/risk-policy/policy-input-v1.schema.json:21` and `crates/risk-policy/tests/risk_policy_contract.rs:81-103`.

## Disposition

Added `malformed-capability-c0-control.json` and `malformed-capability-del-control.json` under `fixtures/risk-policy/invalid/`. The existing contract test requires every invalid fixture to fail both JSON Schema and Rust bounded parsing/evaluation. No production logic changed.
