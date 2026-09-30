# POL-01 independent review — round 3

- Candidate SHA: `233968080aed28cb8b8fa5efa3859774fc824419`
- Reviewer route: Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only / fresh session
- Containment: PASS before the source-bearing review packet
- Verdict: `CHANGES_REQUIRED`
- Review scope: source and contract inspection only; the reviewer ran no commands or tests.

## Findings

1. Public serde entry points bypassed the limits before allocation. `PolicyInput::from_json_str` applied the byte cap, but the public derived `Deserialize` path bypassed it and array limits ran only later in `evaluate`. `PolicyAssessment` deserialized unbounded vectors before checking duplicates. The reviewer cited `crates/risk-policy/src/lib.rs:32, 173-205, 232-245`.
2. The schema rejected leading U+FEFF as ECMAScript whitespace, but Rust `trim()` and `char::is_control` did not. The reviewer cited `schemas/risk-policy/policy-input-v1.schema.json:17-21` and `crates/risk-policy/src/lib.rs:249-253`; `"\uFEFFsystem.info.read"` was the concrete mismatch.

## Reviewer checks

- Ambiguity cannot lower hard-risk; missing evidence does not lower risk or scrutiny.
- POL-01 does not authorize or deny execution; no execution, fetching, secret-resolution, or model path was found.
- Unknown capabilities abstain/escalate and are not safe by default.

## Disposition

Both findings were addressed in implementation commit `881385613d5f7faa1f7a56c8a687da609fac1611`. Public `Deserialize` implementations were removed so bounded `from_json_str` methods are the only JSON entry points. Rust now rejects U+FEFF boundary whitespace, and schema/Rust both bound assessment arrays. Controller reran the required package, locked contract, and full validation; this reviewer did not run Cargo.
