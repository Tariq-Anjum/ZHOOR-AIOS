# Architecture audit: HERMES-AIOS to ZHOOR

**Audit date:** 2026-09-23  
**Repository state reviewed:** `origin/main` at `dbffd08`  
**Verdict:** reuse selected contracts and principles; do not implement the legacy HERMES plan wholesale.

## Executive assessment

HERMES-AIOS is a broad documentation corpus, not an incomplete application. It has 68 Markdown architecture/planning files but no runtime source, manifest, test suite, CI configuration, or executable schemas. Its strongest assets are principles and requirement checklists. Its broad kernel, workflow, memory, plugin, event-bus, and distributed-runtime designs would overbuild ZHOOR before the first decision slice can be measured.

ZHOOR should therefore be established as the normative architecture while preserving HERMES material as a reference quarry. The initial product is a local-first decision fabric, not an autonomous shell or a generic AI operating-system implementation.

## Material findings

| Finding | Evidence | Impact |
|---|---|---|
| No implementation foundation exists | `docs/notes/Audit-Initial.md`; repository tree | Treat this as greenfield contract-first work, not refactoring. |
| M0 is marked complete despite absent build/test/CI artifacts | `docs/implementation/09_IMPLEMENTATION_ROADMAP.md` vs `docs/kanban/HERMES-AIOS-Kanban.md` | Legacy milestone status is unreliable. |
| Repository layout conflicts | `docs/implementation/05_1_REPOSITORY_STRUCTURE.md` uses `src/hermes/`; later 09.x plans use top-level packages | Toolchain/layout must be re-decided, not copied. |
| Security is described but sequenced late | Tool runtime precedes security hardening in legacy roadmap | ZHOOR requires policy and execution contracts before any mutating capability. |
| Memory authority conflicts | Constitution chooses SQLite/Chroma; ZHOOR assigns durable canonical memory to `my-ai-brain` | Legacy memory implementation is not adopted. |
| Planner/router ownership overlaps | 03B and 09.x documents assign planning/tool selection inconsistently | Do not implement the legacy cognitive kernel. |
| README and agent path are stale | `README.md`, `.hermes.md` | Entry points must direct contributors to the adoption workspace. |
| Repository checkout was detached | Git audit | Owner must establish a named integration branch before parallel work. |

## Adopt, adapt, defer, reject

### Adopt as principles

- Local-first operation and replaceable providers.
- Explicit interfaces and versioned contracts.
- Deny-by-default policy.
- Correlation IDs, structured errors, cancellation, and bounded timeouts.
- Least privilege, auditability, and secret isolation.
- Small commits, tests, documentation, and architecture review gates.

### Adapt for ZHOOR

- Capability metadata becomes compact skill/capability descriptors.
- Provider abstraction becomes a thin inference-backend adapter only after benchmarks.
- Audit/event fields become sanitized decision traces and execution receipts.
- Selected storage ideas may support bounded trace/evaluation state, never canonical memory.
- A future HERMES-style client may call ZHOOR; it does not become the Decision Runtime.

### Defer until evidence exists

- Semantic backend, including Needle or Ollama selection.
- Workflow DAG engine, plugins, UI/dashboard, distributed event bus, ChromaDB, multi-agent runtime.
- Autonomous repair, self-improvement, and mutating system capabilities.
- Repository rename and packaging/release automation beyond the first runnable slice.

### Reject for the first architecture

- Direct model-to-tool or model-to-shell execution.
- One giant prompt or eager loading of all skills/memory/policy.
- One process/model per logical worker.
- Confidence as authorization or provenance.
- Model agreement as ground truth.
- HERMES-owned canonical user memory.
- Building the full generic kernel before evaluating two bounded workers.

## Target boundaries

| Component | Owns | Must not own |
|---|---|---|
| Decision Runtime | typed decisions, budgets, route selection, scheduling, trace capture | authorization or OS execution |
| Policy Engine | deterministic authorization/denial/escalation and grant issuance | inference or capability execution |
| `aiosd` | caller authentication, grant/argument validation, allowlisted execution, receipts | deciding desired action |
| `my-ai-brain` | canonical durable Markdown memory | runtime queues or privileged execution |
| Trace/eval store | sanitized replayable traces and labeled cases | unrestricted transcripts or secrets |
| OB1 | optional governed mirror/evidence graph | canonical memory authority |
| Web stack | research/browser capability | policy or execution authority |

## Architecture improvements over legacy HERMES

1. Replace the broad cognitive kernel with a small typed Decision Runtime.
2. Separate recommendation, authorization, and execution into enforceable boundaries.
3. Make abstention, overload, deadlines, provenance, and calibration protocol concepts.
4. Make trace/evaluation infrastructure part of the first vertical slice rather than a late observability milestone.
5. Keep workers as schemas/configuration over shared backends.
6. Use a measured cost ladder: rules → index → local semantic → stronger backend → reasoning model → human.
7. Begin with no model dependency and no autonomous effect.
8. Gate every expansion on datasets, replay, shadow results, and capability-specific safety metrics.

## Primary risks and controls

| Risk | Control |
|---|---|
| Legacy documents regain accidental authority | ADR precedence and explicit legacy disposition |
| Scope expands into a full AIOS | Two-worker limit and dependency-gated work board |
| Parallel agents overwrite one another | Isolated worktrees and exclusive write sets |
| Models fabricate architecture or fixtures | Deterministic/human ground truth and independent review |
| Security becomes documentation-only | Contract tests before executable capability work |
| Vendor/model lock-in | Adapter boundaries and benchmark gate |
| Context overload causes agent drift | Bounded cards, minimal required reads, checkpoint/resume handoffs |
| Retrying consumes time/tokens | Two-attempt stop rule |

## Audit conclusion

Adopt ZHOOR as a narrow, measurable subsystem and future product identity. Preserve HERMES as historical design input. Begin only after branch/worktree setup and the language/toolchain ADR; then build protocol fixtures, deterministic workers, and replayable traces before considering any inference backend.
