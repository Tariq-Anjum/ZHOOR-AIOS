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
| D-010 | GPT-5.6 Sol runs through the Codex harness; the `Lead/Zed` lane is retired. | Owner-reported Zed credit exhaustion. Architecture/security acceptance authority must not be stranded on an unavailable harness. |
| D-011 | Owner-reported and confirmed by session provenance (`PI_MODEL=qwen/qwen3.8-max-0902`): the governance tranche on `integration/zhoor-adoption` was prepared without the Sol lane, with Qwen3.8 Max (Pi) substituted. `ARC-01` must therefore be reviewed by Sol (Codex) as primary, plus a challenger independent of that Qwen3.8 Max authorship (open gate below), before it becomes `DONE`. | Compensating control: authors of a tranche are never its only reviewers, and never its independent challenger. |
| D-012 | Review files are `<JOB-ID>-<reviewer-token>.md`; the role (primary, independent-challenger, interim-primary) is recorded inside the report, never in the filename. | A two-report card cannot have two `-primary` files, and filenames must stay unique per reviewer. |
| D-013 | If a required lane is unavailable, substitution needs a recorded decision, an independent challenge from a different model family, and the card stays in `REVIEW` until that report exists. | Availability pressure must not silently remove independent review. |

## Open gates

| Gate | Owner | Required evidence |
|---|---|---|
| Language/toolchain and package layout | ARC-02 + architecture review | Minimal spike, build/test/lint/type-check, dependency/security/resource comparison |
| Concrete trace/eval storage | TRC-01 proposal, later ADR if durable | Replay requirements, retention, migration, performance measurements |
| Local semantic backend | BENCH-02 | Frozen baseline comparison, calibration, latency/memory/cost/privacy/failure report |
| Queue depth/warm-pool/deadlines | BENCH-01/SHADOW-01 | Hardware measurements, not intuition |
| First reversible capability | CANARY-01 + owner | Passing aiosd contract, shadow evidence, rollback and thresholds |
| Repository rename | Owner, after first slice | Packaging/URLs/automation migration checklist |
| ARC-01 challenger independence | Owner | The artefact under review (`148d103`, `2efdbd3`) was prepared by Qwen3.8 Max (Pi), so a Qwen3.8 Max challenge would be a self-review. Required: a challenger report from a non-Qwen model — `ARC-01-luna.md` (GPT-5.6 Luna, Codex) or `ARC-01-sol.md` covering both roles is **not** acceptable as its own challenge — or an explicit owner waiver recorded in this table. |
