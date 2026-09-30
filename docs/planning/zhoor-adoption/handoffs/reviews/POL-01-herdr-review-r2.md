# POL-01 independent review — round 2

- Candidate SHA: `6c9bb61bb667ff269ee1d1b0095f9ec4e0585b09`
- Reviewer route: Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only / fresh session
- Containment: PASS before the source-bearing review packet
- Verdict: `CHANGES_REQUIRED`
- Review scope: source and contract inspection only; the reviewer ran no commands or tests.

## Findings

1. `PolicyAssessment` derived `Deserialize` with vector fields for `reasons` and `evidence_requirements`. Rust accepted duplicate array elements while `policy-assessment-v1.schema.json` declares `uniqueItems: true` for both arrays. The invalid assessment fixtures did not cover duplicates.
2. Policy input parsing and evaluation had no explicit input-size limits. Evidence-tier duplicate detection searched every earlier element, making malformed large lists quadratic before trust-tier validation.

## Reviewer checks

- Ambiguity does not lower a hard-risk result; missing evidence does not lower scrutiny.
- Unknown capabilities abstain and recommend escalation; they are not treated as safe by default.
- The reviewer found no execution, network fetch, secret resolution, model consultation, authorization, or denial path.
- The reviewer confirmed the C1-control schema/Rust correction and fixture from the prior correction.

## Disposition

Both findings were addressed in implementation commit `1051f3970b2b67c086eb1c2ef07153b2ea3bde1e`. That change rejects duplicate assessment entries during Rust deserialization, bounds JSON and fact-array sizes, aligns schema array limits, and removes the unbounded duplicate-scan behavior. Controller validation is recorded in the POL-01 handoff; this review did not independently run Cargo.
