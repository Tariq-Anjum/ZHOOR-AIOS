# ARC-03 — Toolchain adjudication and path pinning

**Depends on:** ARC-02  
**Recommended:** GPT-5.6 Sol  
**Review:** Qwen3.8 Max, then human owner acceptance  
**Timebox:** one architecture decision session

## Objective

Adjudicate ARC-02 evidence, accept one primary implementation stack/layout, pin canonical quality commands, and replace downstream descriptive write areas with concrete repository-relative path globs before any card becomes `READY`.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read ARC-02 handoff/evidence, ADR-0001, `TARGET_SCAFFOLD.md`, downstream card write scopes, and no unrelated legacy plans.
- Karpathy application: choose one smallest sufficient stack; do not run another broad spike or scaffold code.

## Write scope

- Toolchain ADR under `docs/architecture/adr/`.
- `docs/development/ZHOOR_TOOLCHAIN.md` with exact format/lint/static-check/test/build commands and stable targeted aliases.
- `WORK_BOARD.md` and downstream job-card path/validation sections only, to pin disjoint repository-relative paths and commands.
- `docs/planning/zhoor-adoption/handoffs/ARC-03.md`.

## Acceptance

- One primary stack/layout is accepted; alternatives and mixed-language boundary are explicit.
- Every downstream card has concrete write globs and exact targeted/full validation commands before being marked `READY`.
- Parallel cards have mechanically disjoint paths.
- No model/backend/database/runtime dependency is added.
- Owner accepts the ADR; acceptance is not auto-approved.

## Stop

After two evidence/command verification failures, stop and request owner adjudication. Do not hide uncertainty by selecting multiple production stacks.
