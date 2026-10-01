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

## Shared-memory lifecycle

- The global and AI-Tools shared-memory lifecycle applies to ZHOOR controller
  sessions. The controller/orchestrator owns bootstrap and closeout, recalling
  only relevant durable ZHOOR decisions, project state, orchestration policy,
  environment, and tool/provider context.
- Bounded implementation workers, auditors, reviewers, and challengers do not
  write shared memory. They stay within their assigned source scope and report
  durable findings to the controller, which independently verifies them before
  persistence.
- Canonical memory remains `/mnt/MD/AI-tools/my-ai-brain`; OB1/`open-brain` is a
  searchable mirror only. Controller-owned memory writes, validation,
  canonical-repository synchronization, and mirroring are control-plane work
  outside the ZHOOR product candidate and do not expand a card's product write
  scope. These operations must not mutate the reviewed candidate.
- At verified card or maintenance closeout, persist and synchronize durable
  changed state, or record `MEMORY_CLOSEOUT=NO_CHANGE`.
- If canonical memory write, validation, or synchronization fails, preserve the
  verified ZHOOR state, report `MEMORY_SYNC_FAILED`, and do not start another
  owner-assigned product card until reconciled. If only OB1 mirroring fails,
  preserve canonical memory, report `MEMORY_SYNC_DEGRADED`, and do not silently
  start another card.
- A current explicit owner instruction `NO_MEMORY_SYNC` overrides this
  lifecycle for that task.
