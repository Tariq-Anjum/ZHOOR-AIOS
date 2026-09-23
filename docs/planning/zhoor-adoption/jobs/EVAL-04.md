# EVAL-04 — Complete and aggregate risk-judge dataset

**Depends on:** EVAL-02  
**Recommended:** GPT-5.6 Luna  
**Review:** DeepSeek V4.1 Flash plus human label spot-check  
**Timebox:** one dataset session

## Objective

Add the second bounded batch and aggregate the `risk_judge` dataset to 50–100 reviewed cases, then produce the first complete worker report.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read EVAL-00 contract, EVAL-02 handoff/cases, and versioned risk policy only.
- Karpathy application: fill documented coverage gaps only; do not modify policy or production worker.

## Write scope

- Second `risk_judge` dataset batch, aggregate manifest/report, worker-specific metric expectations/tests, and `docs/planning/zhoor-adoption/handoffs/EVAL-04.md`.
- ARC-03 must replace this description with exact path globs before `READY`.

## Acceptance

- Aggregate contains 50–100 unique, provenance-valid reviewed cases.
- Coverage includes unknown capability, shell/package/service/network/privilege/mutation/path scope, missing evidence, trust-tier conflicts, prompt injection, malformed input, and safe inspection.
- Report includes schema validity, score/recommendation correctness, false-low-risk rate, abstention/escalation behavior, latency, and regressions.
- No authoritative policy denial or grant is attributed to `risk_judge`.

## Stop

The global two-attempt rule applies. Do not alter prior reviewed labels without explicit policy/human evidence; stop and report conflicts.
