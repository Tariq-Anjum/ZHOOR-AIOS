# Adoption decisions

## Accepted

| ID | Decision | Rationale |
|---|---|---|
| D-001 | ZHOOR is the new architecture/product direction; repository rename is deferred. | Identity work must not block architecture. |
| D-002 | ZHOOR v0.2 is normative; conflicting HERMES documents are reference only. | Prevents accidental implementation of contradictory legacy plans. |
| D-003 | First slice is protocol + deterministic `skill_router` + deterministic `risk_judge` + traces/evaluation. | Tests the decision fabric without model or autonomy risk. |
| D-004 | Recommendation, authorization, and execution are separate boundaries. | Model confidence cannot become authority. |
| D-005 | `my-ai-brain` remains canonical durable memory. | Avoids a second memory authority. |
| D-006 | Jev is not a dependency; Needle is benchmark-only after evaluation foundations exist. | Avoids premature vendor/model coupling. |
| D-007 | Every job uses `skill-router` and Karpathy guidance; one bounded logical unit per card, resumable only after interruption with the same handoff/attempt count. | Controls context, scope, hallucination, and cost. |
| D-008 | Two failures on the same blocker require stop and handoff. | Prevents loops and wasted resources. |
| D-009 | Parallel agents require isolated worktrees and disjoint write sets. | Prevents branch and file collisions. |

## Open gates

| Gate | Owner | Required evidence |
|---|---|---|
| Language/toolchain and package layout | ARC-02 + architecture review | Minimal spike, build/test/lint/type-check, dependency/security/resource comparison |
| Concrete trace/eval storage | TRC-01 proposal, later ADR if durable | Replay requirements, retention, migration, performance measurements |
| Local semantic backend | BENCH-02 | Frozen baseline comparison, calibration, latency/memory/cost/privacy/failure report |
| Queue depth/warm-pool/deadlines | BENCH-01/SHADOW-01 | Hardware measurements, not intuition |
| First reversible capability | CANARY-01 + owner | Passing aiosd contract, shadow evidence, rollback and thresholds |
| Repository rename | Owner, after first slice | Packaging/URLs/automation migration checklist |
