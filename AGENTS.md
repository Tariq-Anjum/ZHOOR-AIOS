# ZHOOR repository instructions

## Authority and recovery

- This checkout is the canonical implementation repository for the current
  milestone. Git, accepted architecture decisions, this file, the adoption
  work board, and the [orchestrator runbook](docs/planning/zhoor-adoption/ORCHESTRATOR_RUNBOOK.md)
  define project state. Conversation history is not required for recovery.
- Read the runbook and relevant accepted ADRs before orchestration or cross-
  boundary changes. `docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md` is the
  normative architecture; ADR-0002 permits only `system.info.read` as a narrow
  exception to the original sequence.
- Preserve all pre-existing dirty and untracked work. Inspect `git status`
  before editing, and stage only files and hunks owned by the current task.

## Architecture invariants

- Keep reasoning separate from policy, authorization, and execution. A model
  may propose or classify; deterministic code validates and enforces authority.
- Do not execute arbitrary model-generated shell commands. Do not run agents as
  root. Keep provider, model, and harness choices replaceable.
- Preserve the accepted phase and promotion gates. ADR-0002 does not authorize
  capability expansion, bypass ARC governance, or advance the remaining
  `aiosd` and canary sequence. Further deviations require an accepted ADR.
- Prefer the smallest coherent design, bounded resource use, and progressive
  disclosure. Do not add runtime capability during continuity/governance work.

## Engineering workflow

- Route non-trivial work through `skill-router`; load only relevant project
  instructions and skills.
- Inspect relevant code, interfaces, architecture, and ADRs before changing
  behavior. For behavior changes, update focused tests and run the applicable
  project checks. Report exactly what was run.
- Reviews are read-only. Exact-SHA audits use a detached checkout pinned to the
  candidate commit. Follow the runbook for Herdr discovery, handshakes, writer
  isolation, and recovery.
