# BENCH-01 — Backend adapter and benchmark runner


**Depends on:** AUD-01 approval  
**Recommended:** Muse Spark 1.3  
**Review:** GPT-5.6 Sol  
**Timebox:** one adapter/runner implementation session; no measured candidate run

## Objective

Implement the thinnest replaceable backend adapter, mocks, and frozen benchmark runner/configuration for deterministic/index baselines and one approved local candidate. Do not perform the measured candidate run in this card.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read architecture sections 5–7 and 10, AUD-01, datasets, backend interface decision, and no unrelated raw discussions.
- Karpathy application: benchmark a bounded hypothesis; do not build an inference server or fine-tune a model.

## Write scope

- `crates/backend-adapter/**` — `zhoor-backend-adapter` interface, one approved adapter, mocks, benchmark runner/configuration, tests, and package-local docs.
- `docs/planning/zhoor-adoption/handoffs/BENCH-01.md`.
- No Needle installation or BENCH-02 result/config ownership. Root manifests/lockfiles are read-only; request controlled integration for any new dependency.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-backend-adapter` and `cargo test --offline --locked -p zhoor-backend-adapter --test backend_adapter_contract`.

Full: `./scripts/validate.sh`.

Lockfile timing: component workers do not edit or commit the root `Cargo.lock`. Before integration, run the package checks from `docs/development/ZHOOR_TOOLCHAIN.md` without `--locked` (keep `--offline`) and report any generated lockfile diff without staging it. After integrating the package source/manifests, the controller reconciles and reviews the single root lockfile update, then runs the locked targeted and full commands above on the integration candidate. This card is complete only after those post-integration checks pass.

## Acceptance

- Runner guarantees baselines and semantic candidate consume identical frozen cases and emits the fields needed for accuracy, abstention, semantic/schema errors, calibration, resource, latency, cost/privacy, timeout, and unavailable-backend analysis.
- Mocks prove unavailable/deadline/schema-failure paths before any download or live run.
- Runtime still works fully without the backend.
- Semantic output cannot lower deterministic hard risk or authorize action.
- Failed/unavailable/deadline-missed backend yields typed abstention/escalation.

## Needle guardrail

Needle is optional, not required. BENCH-01 performs no Needle installation. BENCH-02 may do so only with owner approval after verifying current upstream package/version and license, using a disposable environment, telemetry disabled, sanitized data, and no training extras/hosted services.

## Stop

After two installation/adapter failures, stop—do not churn versions. Never expose secrets or real transcripts. Backend promotion requires separate review.
