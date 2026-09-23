# EVAL-03 — Complete and aggregate skill-router dataset

**Depends on:** EVAL-01  
**Recommended:** DeepSeek V4.1 Flash  
**Review:** Qwen3.8 Max plus human label spot-check  
**Timebox:** one dataset session

## Objective

Add the second bounded batch and aggregate the `skill_router` dataset to 50–100 reviewed cases, then produce the first complete worker report.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read EVAL-00 contract, EVAL-01 handoff/cases, and registry/worker reason codes only.
- Karpathy application: fill documented coverage gaps only; do not change harness or production worker.

## Write scope

- Second `skill_router` dataset batch, aggregate manifest/report, worker-specific metric expectations/tests, and `docs/planning/zhoor-adoption/handoffs/EVAL-03.md`.
- ARC-03 must replace this description with exact path globs before `READY`.

## Acceptance

- Aggregate contains 50–100 unique, provenance-valid reviewed cases.
- Coverage includes exact/capability matching, requirements, activation, duplicates/unknowns, ties, budgets, adversarial descriptors, and abstention.
- Report includes schema validity, choice accuracy, false-choice rate, abstention metrics, latency, and regressions.
- No model-generated label is accepted without deterministic/human grounding.

## Stop

The global two-attempt rule applies. Do not alter prior reviewed labels without explicit human evidence; stop and report conflicts.
