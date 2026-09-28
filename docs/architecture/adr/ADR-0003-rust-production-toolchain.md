---
title: Select the Rust production toolchain and workspace layout
status: Accepted
proposed: 2026-09-28
deciders: Project owner (accepted 2026-09-29); ARC-03 controller recommendation
---

# ADR-0003: Select the Rust production toolchain and workspace layout

## Status

Accepted by the human owner on 2026-09-29. ARC-03 is closed; BASE-01 is the only card advanced to `READY`.

## Context

ZHOOR separates the Generation/Reasoning Plane from the deterministic Decision Plane. Reasoning may propose or classify; deterministic protocol validation and policy establish what is authorized; a narrow enforcement boundary executes only typed capabilities. The production implementation needs one language for protocol validation, registry, policy, traces, deterministic workers, and later enforcement code so that domain rules do not drift across runtimes.

The target workstation has 16 GB RAM. ARC-02 compared tiny single-process Rust and Python JSONL implementations plus a Rust core behind a Python process. Across the bounded runs, Rust had lower observed process startup and RSS on this host; Python was the most convenient environment for offline fixture and report work; the mixed option added another process and its lifecycle/stdio contract. Run-to-run variance was material. This is local evidence from a small workload, not a service guarantee or a universal language benchmark.

Downstream work requires versioned wire contracts, disjoint Rust packages for protocol/registry/policy/trace/workers/enforcement, deterministic fixtures and tests, offline evaluation, and reproducible CI. ADR-0002's `system.info.read` package is one narrowly permitted empirical slice and did not choose the repository toolchain.

## Decision

1. **Primary production language/toolchain:** Rust, using the Rust 2024 edition and the exact `rustc`/Cargo toolchain version `1.98.1` observed by ARC-02. `rust-toolchain.toml` pins that version and the `rustfmt` and `clippy` components. Toolchain upgrades are explicit reviewed changes; no floating `stable` channel is used for canonical validation.
2. **Canonical package layout:** one root Cargo workspace with the small `zhoor` CLI package and independently testable packages under `crates/`: `zhoor-protocol`, `zhoor-registry`, `zhoor-risk-policy`, `zhoor-trace`, `zhoor-aiosd-contracts`, `zhoor-skill-router`, `zhoor-risk-judge`, and, only at their later gates, `zhoor-backend-adapter`, `zhoor-shadow`, `zhoor-grant`, `zhoor-aiosd-core`, and `zhoor-aiosd-unix`. The existing `crates/system-info` remains its ADR-0002 slice and must not be treated as the enforcement daemon. `BASE-01` creates only the workspace/CLI smoke scaffold; each later card creates only its assigned package.
3. **Wire-contract source of truth:** versioned JSON Schema Draft 2020-12 files under `schemas/` are the canonical language-neutral wire contracts. `PRO-01` owns request/response/evidence schemas under `schemas/protocol/**`; `TRC-01` alone owns the trace schema under `schemas/trace/**`; `SEC-01` owns `schemas/aiosd/**`. Rust packages are the sole production validators and domain implementations. Their contract tests must check schema/fixture conformance and verify Rust acceptance/rejection against the shared valid/invalid fixtures, including unknown fields and unsupported versions; PRO-01 pins this gate for protocol contracts. Any schema-checking tool is test-only and must not become a production runtime dependency. Do not maintain a second handwritten Python validator or policy implementation. Any generated Rust bindings are derived from those schemas and are not edited as a second schema authority.
4. **Production domain ownership:** Rust owns protocol decoding/validation, registry semantics, policy rules, trace semantics, deterministic workers, and future grant/enforcement behavior. The deterministic production request path has no Python process, RPC, FFI, or subprocess boundary.
5. **Permitted Python boundary:** Python 3.14.7 and its standard library may be used for offline-only evaluation analysis, dataset/fixture generation, benchmark/report tooling, and developer utilities. Python is development tooling, not a production dependency. It may read/write canonical JSON Schema and fixture data, prepare inputs, and summarize outputs; any normative contract or decision validation is delegated to the Rust implementation/tests. No third-party Python dependency is approved by this ADR. Python tools must not authorize, execute, or duplicate protocol/policy/domain rules.
6. **Mixed-language production boundary:** none. There is no justified Python process on the production request path. Reconsider only through a later ADR supported by workload evidence and explicit ownership of lifecycle, failure, versioning, and security boundaries.
7. **Dependency policy:** keep the runtime dependency set small and offline-buildable. Add a dependency only when the owning card demonstrates a requirement, updates the lockfile through the controlled integration path, and records security/supply-chain and resource consequences. This ADR adds no dependency and selects no model, backend, database, framework, provider, or harness. It supersedes the illustrative `src/<component>/` layout and ARC-02 attribution in `TARGET_SCAFFOLD.md`; ARC-02 collected evidence but did not select production layout.
8. **Quality commands:** `docs/development/ZHOOR_TOOLCHAIN.md` is normative for repository-root targeted and full validation. `scripts/validate.sh` is the stable interface that `BASE-01` will implement; until then, ARC-03 runs the documentation checks that exist and does not fabricate a working source build.
9. **Resource posture:** production components remain a small rootless Rust process/package set with no resident Python worker. The 16 GB target is a constraint, not a measured capacity guarantee. Offline Python work is short-lived; no background service is introduced.

## Alternatives considered

### Rust-first — selected

One compiled typed implementation owns the deterministic and future enforcement-facing logic. ARC-02 observed lower startup/RSS for its small Rust exercise on this workstation. The tradeoff is a compile toolchain and a larger development learning curve; exact workspace/package boundaries and tests keep this manageable.

### Python-first — rejected for production

Python is convenient for fixtures and evaluation and its standard library exposed Linux peer credentials without unsafe Rust FFI in the probe. However, ARC-02 observed higher and noisier process startup/RSS in its bounded exercise, and dynamic runtime validation would need explicit enforcement tests. The measured probe did not test a production service, so these results are supporting evidence rather than proof. Python remains available for offline development tooling.

### Bounded Rust/Python mixed production — rejected

The tested option kept the rule in Rust but required a Python parent and a subprocess/stdio contract. It added a runtime and failure/lifecycle boundary without a demonstrated production requirement; its measured cold start was highest in this small exercise. Keeping Python off the production path removes that maintenance and security boundary while preserving its offline tooling value.

### Other languages/frameworks — deferred

No evidence in ARC-02 justifies a second production language, asynchronous runtime framework, service framework, database, model SDK, or provider integration. They remain out of scope until a later card supplies a concrete need and reviewable evidence.

## Consequences

- **Dependencies/supply chain:** Cargo and a pinned Rust toolchain are required to build production code. The workspace lockfile and offline commands make dependency resolution reviewable. Python uses only the interpreter standard library; no pip runtime or development package is authorized here.
- **Resource/security:** one Rust implementation avoids a Python request-path process and prevents a second policy authority. Rust does not itself authorize capabilities: validation, deterministic policy, explicit grants, authenticated identity, and typed execution remain separate gates. Rootless-by-default behavior and narrow future polkit use remain unchanged.
- **Schemas/contracts:** JSON Schema is the language-neutral wire authority. Rust is the only production semantic validator. Offline Python tools may manipulate fixtures and schema data but must delegate normative validity to Rust contract checks.
- **Testing/reproducibility:** every crate has focused tests; canonical commands pin format, lint, static checking, unit/contract tests, full workspace validation, and release build. CI invokes the same full command. No command depends on network resolution during validation.
- **Packaging:** production distribution is Rust workspace artifacts. There is no Python package to install or ship with a request handler.
- **Maintenance/ergonomics:** Rust's compiler and workspace boundaries add initial setup work. Python remains available where its concise stdlib is useful without creating semantic ownership. Shared schema and fixture tests protect the language boundary.
- **Provider/model/harness independence:** this decision concerns implementation language only. It does not bind ZHOOR to a model, provider, agent harness, hosted backend, or runtime inference framework.
- **Migration/reversibility:** the workspace is new and no production subsystem exists yet, so package boundaries can be changed before downstream implementations. A future language or mixed-boundary change requires a new accepted ADR and migration plan; ARC-02 data alone is insufficient to justify it.
- **Downstream cards:** `BASE-01` creates the workspace and validation wrapper; `PRO-01` owns canonical protocol schema/package; each later card has a concrete disjoint package/data/report scope and targeted command in its card. All later statuses remain governed by dependencies and the work board.

## Non-goals

- Implementing the full `aiosd` daemon or any production subsystem in ARC-03.
- Expanding ADR-0002 beyond rootless read-only `system.info.read`.
- Selecting a model, provider, harness, inference backend, hosted service, or runtime framework.
- Selecting a database or adding persistence beyond the accepted append-only ledger direction.
- Implementing downstream packages, generated bindings, CI, or the validation script in ARC-03.
- Treating ARC-02's measurements as production guarantees or changing promotion/security gates.
