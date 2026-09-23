# ARC-01 — Governance baseline review


**Depends on:** none  
**Recommended:** GPT-5.6 Sol  
**Independent challenger:** Qwen3.8 Max  
**Timebox:** one review session

## Objective

Verify that ADR-0001, the copied ZHOOR v0.2 architecture, constitution amendment, repository entry points, and this adoption workspace form a coherent source-of-truth hierarchy. This is review, not redesign.

## Required context and skills

- Load `skill-router`.
- Load `karpathy-guidelines`; explicitly state assumptions, avoid overbuilding, stay in review scope, and judge against the success condition.
- Load `file-based-planning` only if review cannot finish in one session.
- Read: root `README.md`, `.hermes.md`, constitution, ADR-0001, ZHOOR architecture, and this workspace's `README.md`, `ROADMAP.md`, `WORK_BOARD.md`.

## Write scope

- `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-primary.md` only.
- Do not edit governance documents or create an implementation handoff during review.

## Success condition

A review report identifies every blocking inconsistency in authority, identity, scope, safety boundaries, or job dependencies; otherwise it explicitly approves the baseline.

## Checks

- Repository rename is deferred without ambiguity.
- Legacy HERMES docs are non-normative where they conflict.
- First slice has no model dependency or autonomous effect.
- Memory, policy, inference, and execution ownership do not overlap.
- Board dependencies match the roadmap.
- Every job is a bounded logical unit and every architecture/security change has independent review.

## Stop

Do not solve findings. After two attempts to verify a contradiction, record evidence and stop. Use `templates/REVIEW_TEMPLATE.md` for the unique review handoff.
