# REG-01 — Compact skill registry


**Depends on:** PRO-01  
**Recommended:** GLM 5.3 Flash  
**Review:** Qwen3.8 Flash  
**Timebox:** one implementation session

## Objective

Implement deterministic loading and validation of compact skill descriptors and an index-first lookup surface. Do not load skill bodies.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read ZHOOR architecture sections 4 and 5, accepted protocol public interfaces, and representative compact entries from the local skills index—not every skill body.
- Karpathy application: support only fields required by v1 routing; no embeddings or fuzzy model search.

## Write scope

- Registry package/module.
- Registry descriptor schema if not owned by PRO-01.
- Registry fixtures/tests and `docs/planning/zhoor-adoption/handoffs/REG-01.md`.
- ARC-03 must replace this description with exact path globs and validation commands before `READY`.

## Acceptance

- Validates `id`, description, capabilities, requirements, activation, risk level, and version/hash as approved by protocol.
- Exact ID and capability queries are deterministic and stably ordered.
- Duplicate IDs, invalid activation, unsatisfied requirements, malformed metadata, and registry-version mismatch fail explicitly.
- No `SKILL.md` body loading, network, embedding, or model dependency.
- Targeted and full canonical checks pass.

## Stop

The global two-attempt rule applies. If protocol lacks a required descriptor/version field, stop and request a PRO-01 follow-up; do not silently fork the schema.
