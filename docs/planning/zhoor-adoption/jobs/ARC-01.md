# ARC-01 — Governance baseline review


**Depends on:** none  
**Recommended:** GPT-5.6 Sol (Codex harness) — primary reviewer  
**Independent challenger:** Qwen3.8 Max (Pi harness)  
**Acceptance:** human owner (wave 0 gate)  
**Timebox:** one review session

## Objective

Verify that ADR-0001, the copied ZHOOR v0.2 architecture, constitution amendment, repository entry points, and this adoption workspace form a coherent source-of-truth hierarchy. This is review, not redesign.

## Required context and skills

- Load `skill-router`.
- Load `karpathy-guidelines`; explicitly state assumptions, avoid overbuilding, stay in review scope, and judge against the success condition.
- Load `file-based-planning` only if review cannot finish in one session.
- Read: root `README.md`, `.hermes.md`, constitution, ADR-0001, ZHOOR architecture, and this workspace's `README.md`, `ROADMAP.md`, `WORK_BOARD.md`.

## Write scope

- Primary reviewer writes `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-sol.md` only.
- Independent challenger writes `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-qwen-max.md` only, and does not read the primary report first.
- Author conflict: the tranche under review was prepared by Qwen3.8 Max (Pi), so a Qwen3.8 Max challenge is a self-review. Resolve the `ARC-01 challenger independence` open gate in `docs/planning/zhoor-adoption/DECISIONS.md` before assigning the challenger.
- Do not edit governance documents, job cards, or `WORK_BOARD.md`, and create no implementation handoff during review.
- If the Sol lane is unavailable, `docs/planning/zhoor-adoption/ORCHESTRATOR_RUNBOOK.md` §Lane unavailability governs substitution; one model never writes both reports.

## Success condition

Each review report identifies every blocking inconsistency in authority, identity, scope, safety boundaries, or job dependencies; otherwise it explicitly approves the baseline. The owner adjudicates the two reports.

## Checks

- Repository rename is deferred without ambiguity.
- Legacy HERMES docs are non-normative where they conflict.
- First slice has no model dependency or autonomous effect.
- Memory, policy, inference, and execution ownership do not overlap.
- Board dependencies match the roadmap.
- Every job is a bounded logical unit and every architecture/security change has independent review.
- Lane and ownership naming agrees across `WORK_BOARD.md`, `ORCHESTRATOR_RUNBOOK.md`, `MODEL_AND_SKILL_MATRIX.md`, `DECISIONS.md`, and the cards, with no reference to a retired harness.

## Stop

Do not solve findings. After two attempts to verify a contradiction, record evidence and stop. Use `templates/REVIEW_TEMPLATE.md` for the unique review handoff.
