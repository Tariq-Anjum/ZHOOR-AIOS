---
title: Adopt ZHOOR as the architectural direction
status: Accepted
accepted: 2026-09-23
deciders: Project owner and lead architecture review
supersedes: Conflicting HERMES-AIOS implementation assumptions
---

# ADR-0001: Adopt ZHOOR as the architectural direction

## Context

HERMES-AIOS contains extensive architecture and implementation-planning documents but no source package, build manifest, tests, CI workflow, or executable contract fixtures. Its documents also disagree about package layout, milestone completion, planner/router ownership, memory authority, and when security becomes enforceable.

The ZHOOR review produced a narrower architecture designed for the actual local-first constraints: deterministic mechanisms before models, bounded typed decisions, policy above confidence, a separate privileged execution boundary, evidence provenance, and evaluation before autonomy.

The repository will retain the HERMES-AIOS name temporarily and be renamed later. A repository rename must not block architecture or implementation.

## Decision

1. ZHOOR is the new product and architectural direction inside this repository.
2. `docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md` is the normative technical baseline for new implementation.
3. The initial implementation is limited to the deterministic `skill_router` and `risk_judge`, protocol fixtures, and replayable traces. Neither worker may execute capabilities.
4. Policy authorization and privileged execution remain separate. `aiosd` is the only planned privileged execution boundary and begins with the non-mutating `capabilities.describe` capability.
5. `my-ai-brain` remains canonical durable memory. ZHOOR owns only bounded session state and sanitized trace/evaluation data. OB1 is optional governed evidence/mirroring.
6. Existing HERMES documents are retained as historical/reference material. A concept is not implementation authority unless this ADR, the ZHOOR architecture, or a later accepted ADR explicitly adopts it.
7. No model, provider, Jev, Needle, Ollama, workflow engine, vector database, plugin framework, or UI is a prerequisite for the first deterministic slice.
8. Repository rename, dashboard work, autonomous repair, broad workflow orchestration, distributed execution, and plugin marketplaces are deferred.

## Source-of-truth order

When documents conflict, use this order:

1. Current constitution.
2. Accepted ADRs.
3. `docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md`.
4. Accepted contracts and schemas produced by the ZHOOR adoption plan.
5. `docs/planning/zhoor-adoption/` execution controls.
6. Legacy HERMES-AIOS documents as non-normative reference.
7. Raw discussions and chat archives as research only.

A later ADR may amend this decision. Silent divergence is not allowed.

## Consequences

### Positive

- Implementation starts from enforceable contracts and a measurable vertical slice.
- Models remain replaceable and cannot become authorization boundaries.
- Security, traces, calibration, and abstention behavior are designed before autonomy.
- The project avoids implementing the broad legacy kernel, memory, plugin, and workflow plans without evidence.

### Costs

- Much of the legacy documentation must be dispositioned rather than implemented directly.
- The language/toolchain decision remains gated on a short evidence-producing spike.
- Initial delivery intentionally provides recommendations and traces, not autonomous OS repair.

## Required follow-up

Use `docs/planning/zhoor-adoption/README.md` as the agent entry point. Work must follow its dependency board, per-job write scopes, validation gates, and handoff protocol.
