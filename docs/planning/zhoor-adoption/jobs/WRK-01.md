# WRK-01 — Deterministic skill router


**Depends on:** REG-01, TRC-01  
**Recommended:** Muse Spark 1.3 or GLM 5.3 Flash  
**Review:** GPT-5.6 Luna  
**Timebox:** one implementation session

## Objective

Implement the Level 0/1 `skill_router`: validate a bounded request, filter and rank compact candidates deterministically, return a typed choice or explicit abstention, and emit a trace.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read ZHOOR architecture sections 3–5, accepted protocol/registry/trace public interfaces, and no raw discussions.
- Karpathy application: exact metadata matching only; no semantic backend, prompt, embedding, or skill-body loading.

## Write scope

- `skill_router` worker package/module.
- Worker-specific fixtures/tests/docs and `docs/planning/zhoor-adoption/handoffs/WRK-01.md`.
- ARC-03 must replace this description with exact path globs and validation commands before `READY`.

## Acceptance

- Enforces candidate count/input budgets and activation/requirement constraints.
- Uses documented stable tie-breaking and reason codes.
- Returns only IDs supplied/registered in the request scope.
- Abstains on no eligible match or unresolved ambiguity; never fabricates a skill.
- Emits a sanitized trace for resolved, abstained, and invalid paths.
- Has no capability execution or model dependency.
- Targeted and full checks pass.

## Stop

If ranking needs semantic inference, create a benchmark requirement in the handoff; do not add a model route. Two failed interface integrations require stop.
