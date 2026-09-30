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
| D-011 | Per-commit authorship of the wave-0 artefact: `1904baf` was prepared by GPT-5.6 Sol in the Zed harness; `148d103` (this handoff layer) and `2efdbd3`/`2f3270f` (lane rename, review naming, conflict disclosure) were prepared by Qwen3.8 Max in the Pi harness after Zed credits were exhausted (owner-reported; confirmed by `PI_MODEL=qwen/qwen3.8-max-0902`). `ARC-01` must therefore be reviewed by Sol (Codex) as primary, plus a challenger independent of the Qwen3.8 Max authorship of the handoff/lane layer (open gate below), before it becomes `DONE`. | Compensating control: authors of a tranche are never its only reviewers, and never its independent challenger. Provenance is recorded per commit, not per branch. |
| D-012 | Review files are `<JOB-ID>-<reviewer-token>.md`; the role (primary, independent-challenger, interim-primary) is recorded inside the report, never in the filename. | A two-report card cannot have two `-primary` files, and filenames must stay unique per reviewer. |
| D-013 | If a required lane is unavailable, substitution needs a recorded decision, an independent challenge from a different model family, and the card stays in `REVIEW` until that report exists. | Availability pressure must not silently remove independent review. |
| D-014 | All sessions and agents use named Herdr Runtime Server sessions for continuity and handoff. Non-Sol agents handle implementation, evidence, and routine review; Sol is reserved exclusively for the final audit. This supersedes the active lane assignments in D-010–D-013; those rows remain historical provenance only. | Keeps handoffs durable and prevents silent lane changes. |
| D-015 | Current identity is `/mnt/MD/AI-tools/ZHOOR-AIOS` / `Tariq-Anjum/ZHOOR-AIOS`; the in-place rename and publication of `integration/zhoor-adoption` are complete. Stable assignments use job roles; reviewer independence is a separate reviewer/session with no authorship conflict, without a vendor/model-family requirement. This supersedes the active lane/independence constraints in D-010–D-014; current concrete mappings are in `MODEL_AND_SKILL_MATRIX.md`. After milestone implementation/review, deterministic checks, independent audit PASS, and protected-status reconciliation, the orchestrator may normally push only `integration/zhoor-adoption` to its matching `origin` branch. If remote divergence is found, stop. This excludes `main`, tags, releases, force pushes, history rewrites, branch deletion, and repository settings. | Records current operational state and a bounded publication authorization without rewriting earlier provenance. |
| D-016 | ARC-01 owner adjudication accepts and closes O-ARC01-01 through O-ARC01-10, covering both official reports for candidate `6a73756e8694d7d70e79cb3c8d037b54a79c5a32`: `ARC-01-sol.md` and `ARC-01-luna.md`. Review independence: PASS. Required fixes are recorded in this closure commit; ARC-01 may become DONE only after an independent exact-SHA closure audit passes. | Compact owner record; the reports remain the detailed review evidence. |
| D-017 | For ARC-01 closure only, the owner retrospectively accepts incorporation of the 11 enumerated governance-overlap paths below. The original checkpoint had 39 protected paths; the other 28 remain protected. Pre-closure hashes were not recorded, so byte-for-byte preservation of the former dirty content cannot be independently proven; the owner accepts this residual risk. Future protected-work baselines record status, repository-relative path, and SHA-256 of working-tree bytes where safe; missing/deleted/special states are typed, and secret-sensitive paths use `HASH_WITHHELD_SECRET` and stay outside automated commit/reconciliation scopes. | Narrow disposition of the repeated Pi audit blocker; no general staging waiver. |
| D-018 | Protected working-tree baselines preserve pre-existing content rather than making pathnames globally immutable. An explicitly authorized orchestrator status/control hunk may modify a protected shared control file only after baseline-hash verification and with unrelated dirty content preserved and left unstaged. Workers remain limited to their card write scopes. | Allows exact owner-authorized milestone transitions while preserving unrelated pre-existing work. |
| D-019 | On 2026-09-30, the owner explicitly designated REG-01 as the next milestone and authorized the controller-only transition `BLOCKED → READY`. This is a scheduling decision only; REG-01 scope and contracts are unchanged. POL-01, SEC-01, WRK-01, and WRK-02 remain BLOCKED, and no other implementation card is authorized. | Records the owner’s explicit post-TRC-01 scheduling decision. |

### ARC-01 protected-work disposition

The 11 owner-waived incorporated paths are: `docs/planning/zhoor-adoption/DECISIONS.md`; `docs/planning/zhoor-adoption/MODEL_AND_SKILL_MATRIX.md`; `docs/planning/zhoor-adoption/ORCHESTRATOR_RUNBOOK.md`; `docs/planning/zhoor-adoption/README.md`; `docs/planning/zhoor-adoption/WORK_BOARD.md`; `docs/planning/zhoor-adoption/handoffs/NEXT_ACTIONS.md`; `docs/planning/zhoor-adoption/handoffs/reviews/README.md`; `docs/planning/zhoor-adoption/jobs/ARC-01.md`; `docs/planning/zhoor-adoption/jobs/ARC-02.md`; `docs/planning/zhoor-adoption/jobs/ARC-03.md`; `docs/planning/zhoor-adoption/jobs/SEC-01.md`.

## Open gates

| Gate | Owner | Required evidence |
|---|---|---|
| Language/toolchain and package layout | ARC-02 + architecture review | Minimal spike, build/test/lint/type-check, dependency/security/resource comparison |
| Concrete trace/eval storage | TRC-01 proposal, later ADR if durable | Replay requirements, retention, migration, performance measurements |
| Local semantic backend | BENCH-02 | Frozen baseline comparison, calibration, latency/memory/cost/privacy/failure report |
| Queue depth/warm-pool/deadlines | BENCH-01/SHADOW-01 | Hardware measurements, not intuition |
| First reversible capability | CANARY-01 + owner | Passing aiosd contract, shadow evidence, rollback and thresholds |
| Post-execution verification contract | SEC-01 | Define postcondition schema, verification owner, typed verdict/status, required deterministic evidence, unknown/failure behavior, evidence linkage, and how verification affects later policy/state before any execution-capable workflow is eligible for promotion. |
