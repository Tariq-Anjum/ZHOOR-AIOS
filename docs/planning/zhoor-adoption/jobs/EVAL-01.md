# EVAL-01 — First skill-router dataset batch


**Depends on:** EVAL-00  
**Recommended:** GPT-5.6 Luna  
**Review:** DeepSeek V4.1 Flash plus human label spot-check  
**Timebox:** one dataset/evaluation session

## Objective

Create the first 25–50 reviewed `skill_router` cases grounded in deterministic registry state, including abstention and adversarial cases, using the shared evaluation harness.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read ZHOOR architecture section 10, the EVAL-00 harness contract, accepted worker/protocol/trace interfaces, and only the registry fixtures needed for labels.
- Karpathy application: keep metrics minimal and explicit; a model may draft cases but cannot define labels.

## Write scope

- First `skill_router` dataset batch, provenance metadata, oracle/label validation tests, and `docs/planning/zhoor-adoption/handoffs/EVAL-01.md`.
- ARC-03 must replace this description with exact path globs before `READY`.

## Acceptance

- 25–50 cases begin coverage of exact/capability matching, requirements, activation modes, duplicate/unknown candidates, ties, budget limits, adversarial descriptors, and abstention.
- Every label records source, reviewer, registry/policy version, and rationale/reason code.
- Batch validation reports schema/provenance validity and deterministic expected outcomes.
- Repeated runs are deterministic and traces link to cases without embedding raw secrets; EVAL-03 owns aggregate metrics/reporting.
- No second model acts as judge or ground truth.
- Targeted and full checks pass.

## Stop

If deterministic ground truth cannot be established for a case, exclude it or mark it review-required; never infer the label by model vote. Do not modify the shared harness; stop after two interface failures.
