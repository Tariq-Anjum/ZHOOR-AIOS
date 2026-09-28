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

- `crates/registry/**` — `zhoor-registry` package, implementation, tests, and package-local docs.
- `schemas/registry/**` and `fixtures/registry/**` for registry descriptor schema/data owned only by this card.
- `docs/planning/zhoor-adoption/handoffs/REG-01.md`.
- Protocol schemas/types in `schemas/protocol/**` and `crates/protocol/**` are read-only dependencies. Do not edit root manifests/lockfiles.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-registry` and `cargo test --offline --locked -p zhoor-registry --test registry_contract`.

Full: `./scripts/validate.sh`.

Lockfile timing: component workers do not edit or commit the root `Cargo.lock`. Before integration, run the package checks from `docs/development/ZHOOR_TOOLCHAIN.md` without `--locked` (keep `--offline`) and report any generated lockfile diff without staging it. After integrating the package source/manifests, the controller reconciles and reviews the single root lockfile update, then runs the locked targeted and full commands above on the integration candidate. This card is complete only after those post-integration checks pass.

## Acceptance

- Validates `id`, description, capabilities, requirements, activation, risk level, and version/hash as approved by protocol.
- Exact ID and capability queries are deterministic and stably ordered.
- Duplicate IDs, invalid activation, unsatisfied requirements, malformed metadata, and registry-version mismatch fail explicitly.
- No `SKILL.md` body loading, network, embedding, or model dependency.
- Targeted and full canonical checks pass.

## Stop

The global two-attempt rule applies. If protocol lacks a required descriptor/version field, stop and request a PRO-01 follow-up; do not silently fork the schema.
