# TRC-01 — Sanitized trace and replay core


**Depends on:** PRO-01  
**Recommended:** Muse Spark 1.3  
**Review:** DeepSeek V4.1 Flash  
**Timebox:** one implementation session

## Objective

Implement deterministic trace creation, serialization, sanitization, and replay input reconstruction for protocol-level decisions.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Load `file-based-planning` only if storage trade-offs exceed this session.
- Read ZHOOR architecture sections 6, 10, and 11; protocol interfaces; ARC-02 toolchain docs.
- Karpathy application: prefer an append/test fixture implementation; do not choose a broad database platform.

## Write scope

- `crates/trace/**` — `zhoor-trace` package, trace/replay implementation, sanitization rules, tests, and package-local docs.
- `schemas/trace/**` as the sole canonical trace wire schema and `fixtures/trace/**`. Protocol request/response/evidence schemas in `schemas/protocol/**` are read-only dependencies.
- `docs/planning/zhoor-adoption/handoffs/TRC-01.md`.
- Protocol schemas/types are read-only dependencies. Do not edit root manifests/lockfiles or other component paths.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-trace` and `cargo test --offline --locked -p zhoor-trace --test trace_contract`.

Full: `./scripts/validate.sh`.

Lockfile timing: component workers do not edit or commit the root `Cargo.lock`. Before integration, run the package checks from `docs/development/ZHOOR_TOOLCHAIN.md` without `--locked` (keep `--offline`) and report any generated lockfile diff without staging it. After integrating the package source/manifests, the controller reconciles and reviews the single root lockfile update, then runs the locked targeted and full commands above on the integration candidate. This card is complete only after those post-integration checks pass.

## Acceptance

- Records protocol/worker/model-or-deterministic/policy/registry versions, normalized hashes, route, confidence, evidence refs, timings, disposition, and error/receipt refs where applicable.
- Never records secrets, unrestricted transcripts, hidden reasoning, or raw evidence bodies by default.
- Replay reconstructs normalized decision inputs and detects hash/version mismatch.
- Identical normalized input produces stable hashes; each invocation retains unique trace/request identity.
- Queue/load/inference timings remain distinct.
- Targeted and full checks pass.

## Stop

The global two-attempt rule applies. Do not introduce canonical memory or a vector database. If durable storage needs an ADR, implement only the in-memory/file fixture boundary and hand off the decision.
