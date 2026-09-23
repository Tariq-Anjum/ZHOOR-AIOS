# EVAL-02 — First risk-judge adversarial dataset batch


**Depends on:** EVAL-00  
**Recommended:** Qwen3.8 Flash  
**Review:** Qwen3.8 Max plus human label spot-check  
**Timebox:** one dataset session

## Objective

Create the first 25–50 reviewed `risk_judge` cases, emphasizing fail-closed behavior, prompt injection, trust-tier confusion, privilege/mutation, and insufficient evidence, using the shared evaluation harness.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read ZHOOR architecture sections 8–10, accepted risk rules, and the EVAL-00 harness contract.
- Karpathy application: do not expand the risk taxonomy during dataset generation; report missing rules instead.

## Write scope

- First `risk_judge` dataset/provenance batch, expected-score/recommendation tests, and `docs/planning/zhoor-adoption/handoffs/EVAL-02.md`.
- ARC-03 must replace this description with exact path globs before `READY`.

## Acceptance

- 25–50 cases begin coverage of unknown capability, shell/package/service/network/privilege/mutation/path scope, missing evidence, conflicting trust tiers, injected instructions, malformed input, and safe non-mutating inspection.
- Labels come from versioned hard policy and human review, not model agreement.
- Batch validation checks score/recommendation correctness, abstention/escalation advice, schema/provenance validity, and determinism; EVAL-04 owns aggregate metrics/reporting.
- Any case that could reduce hard-rule scrutiny is explicitly tested; no case attributes authoritative denial or grants to `risk_judge`.
- Targeted and full checks pass.

## Stop

Do not modify production policy from this card. Record policy gaps for a separate reviewed follow-up. Stop after two dataset/harness interface failures.
