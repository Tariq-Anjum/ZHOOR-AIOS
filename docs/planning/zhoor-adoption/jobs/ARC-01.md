# ARC-01 — Governance baseline review


**Depends on:** none
**Recommended:** Herdr Runtime Server primary session
**Independent challenger:** separate Herdr Runtime Server session
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

- Primary reviewer writes `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-herdr-primary.md` only.
- Independent challenger writes `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-herdr-challenger.md` only and does not read the primary report first.
- Both reports run in separate sessions with different, non-author reviewers. Use the current concrete reviewer mapping in `MODEL_AND_SKILL_MATRIX.md` where available.
- Do not edit governance documents, job cards, or `WORK_BOARD.md`, and create no implementation handoff during review.
- If a Herdr agent is unavailable, use the handoff rules in `docs/planning/zhoor-adoption/ORCHESTRATOR_RUNBOOK.md`; one agent never writes both reports.

## Success condition

Each review report identifies every blocking inconsistency in authority, identity, scope, safety boundaries, or job dependencies; otherwise it explicitly approves the baseline. The owner adjudicates the two reports.

## Checks

- Current repository identity and development-branch publication match the active governance record; historical rename statements remain provenance only.
- Legacy HERMES docs are non-normative where they conflict.
- First slice has no model dependency or autonomous effect.
- Memory, policy, inference, and execution ownership do not overlap.
- Board dependencies match the roadmap.
- Every job is a bounded logical unit and every architecture/security change has independent review.
- Stable job roles and independence rules agree across active governance. Historical model/harness provenance may remain; no author may independently challenge their own reviewed tranche.

## Stop

Do not solve findings. After two attempts to verify a contradiction, record evidence and stop. Use `templates/REVIEW_TEMPLATE.md` for the unique review handoff.
