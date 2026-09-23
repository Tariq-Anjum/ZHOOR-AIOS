# PRO-01 — Decision protocol schemas and fixtures


**Depends on:** BASE-01  
**Recommended:** GLM 5.3 Flash  
**Review:** GPT-5.6 Luna  
**Timebox:** one implementation session

## Objective

Implement versioned request, response, evidence-reference, confidence, disposition, result-kind, deadline/budget, and error contracts with deterministic validation and positive/negative fixtures.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Use OpenSpec only if it is already initialized and clearly reduces work; do not install it.
- Read ZHOOR architecture sections 3, 5, 9, and 10; ARC-02 toolchain docs; this card only.
- Karpathy application: implement v1 only; state any interpretation; no transport/server/model code.

## Write scope

- `schemas/decision-*.v1.*`, `schemas/evidence-ref.v1.*`, `schemas/trace.v1.*` as selected by ARC-02.
- Protocol package/module and its tests.
- `fixtures/protocol/valid/` and `fixtures/protocol/invalid/`.
- Protocol-specific docs and `docs/planning/zhoor-adoption/handoffs/PRO-01.md`.
- ARC-03 must replace this description with exact path globs and validation commands before `READY`.

## Acceptance

- Unknown versions and result/operation kinds fail closed with typed errors.
- Required IDs, evidence trust tier, budgets, deadlines, disposition, and evidence references validate.
- Over-budget and internally inconsistent envelopes are rejected.
- `choice`, `score`, `extract`, and `verdict` remain distinct; abstention is a disposition.
- Fixtures round-trip deterministically; no secrets or reasoning transcript fields exist.
- Canonical targeted and full checks pass.

## Stop

Do not invent execution grants or backend APIs. After two validation-design failures, hand off the ambiguity for architecture review.
