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

- `crates/skill-router/**` — `zhoor-skill-router` deterministic worker, tests, and package-local docs.
- `fixtures/skill-router/**`.
- `docs/planning/zhoor-adoption/handoffs/WRK-01.md`.
- Registry, protocol, and trace paths are read-only dependencies. No Python/model/backend implementation.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-skill-router` and `cargo test --offline --locked -p zhoor-skill-router --test skill_router_contract`.

Full: `./scripts/validate.sh`.

Lockfile timing: component workers do not edit or commit the root `Cargo.lock`. Before integration, run the package checks from `docs/development/ZHOOR_TOOLCHAIN.md` without `--locked` (keep `--offline`) and report any generated lockfile diff without staging it. After integrating the package source/manifests, the controller reconciles and reviews the single root lockfile update, then runs the locked targeted and full commands above on the integration candidate. This card is complete only after those post-integration checks pass.

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
