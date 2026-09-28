# ZHOOR adoption workspace

**Point every implementation or review agent to this directory first.**

This workspace governs bounded, dependency-ordered ZHOOR AIOS work. It is the execution control plane for the canonical repository `Tariq-Anjum/ZHOOR-AIOS`.

## Start here

1. Read `AGENT_OPERATING_RULES.md`.
2. Read `WORK_BOARD.md`. Implementation agents accept only owner-assigned `READY` jobs; review agents accept only owner-assigned `REVIEW` jobs.
3. Read only that job's file under `jobs/` and the files listed in its context section.
4. Load `skill-router` and the mandatory Karpathy guidance named in the job. Load no other skill unless the card requests it.
5. Work only in the card's write scope.
6. Validate with the card's commands.
7. Implementers write `handoffs/<JOB-ID>.md`; reviewers write unique `handoffs/reviews/<JOB-ID>-<REVIEWER>.md` files. Use the matching template.
8. Stop. The orchestrator—not the worker—updates the central board and unblocks dependents.

## Authority

The adoption decision is `../../architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md`. The normative architecture is `../../architecture/ZHOOR_ARCHITECTURE-v0.2.md`.

When documents conflict:

1. Current constitution
2. Accepted ADRs
3. ZHOOR architecture
4. Accepted schemas/contracts
5. This adoption workspace
6. Legacy HERMES documents
7. Raw discussions/chat archives

Legacy HERMES documents are reference material, not build instructions, unless a current job explicitly adopts a concept.

## Workspace map

| File | Purpose |
|---|---|
| `ARCHITECTURE_AUDIT.md` | Evidence-based review of HERMES assets, gaps, and dispositions |
| `ROADMAP.md` | Dependency graph, phase gates, session boundaries, and completion definition |
| `WORK_BOARD.md` | Current job status, dependencies, model recommendation, and write-set ownership |
| `AGENT_OPERATING_RULES.md` | Safety, context, retry, approval, Git, and handoff rules |
| `MODEL_AND_SKILL_MATRIX.md` | Cost-aware agent/model allocation and minimal skill stacks |
| `ORCHESTRATOR_RUNBOOK.md` | Exact Pi/OpenCode/Codex lanes, wave order, lane-unavailability substitution, assignment prompt, and resume protocol |
| `TARGET_SCAFFOLD.md` | Intended repository shape; it is not permission to create all directories at once |
| `DECISIONS.md` | Adoption-level decisions and open gates |
| `FINDINGS.md` | Facts discovered during the architecture audit |
| `ERRORS.md` | Failed approaches and blockers that must not be rediscovered |
| `jobs/` | Bounded logical work packets; interrupted cards may resume only themselves |
| `handoffs/` | One report per completed or blocked job |
| `templates/` | Handoff and review templates |

## Current state

- Repository: documentation-only; no runtime source, package manifest, tests, or CI.
- Git: the governance tranche is committed on the named branch `integration/zhoor-adoption` (audit base was `origin/main` at `dbffd08`). Not pushed — push is owner-only.
- Architecture: ZHOOR v0.2 adopted; implementation not started.
- Lanes: GPT-5.6 Sol runs through Codex; the former Zed lane is retired. Substitution rules live in `ORCHESTRATOR_RUNBOOK.md` §Lane unavailability.
- Immediate gate: `ARC-01` governance review — Sol (Codex) primary plus GPT-5.6 Luna (Codex) independent challenger, adjudicated by the owner; Qwen3.8 Max is ineligible for that challenge because it authored the handoff/lane layer. Assignment prompts: `handoffs/NEXT_ACTIONS.md`.
- First implementation gate: `ARC-02` gathers toolchain evidence; `ARC-03` selects the stack, pins paths/checks, and must finish before source scaffolding.
- Needle/Jev: not required now. Needle may be measured only in approval-gated `BENCH-02` after deterministic datasets, traces, and audit exist. Jev is not a planned dependency.

## Human/orchestrator setup before parallel work

1. Done — the checkout is on the named branch `integration/zhoor-adoption`; planning changes were preserved.
2. Done — the architecture/planning tranche is committed there as reviewed documentation changes.
3. Create one isolated Git worktree and branch per concurrently assigned job. Never let multiple agents switch branches in one shared checkout.
4. Assign only status-eligible cards whose dependencies are satisfied: `READY` for implementation or `REVIEW` for review.
5. Do not mark downstream cards `READY` until ARC-03 has pinned concrete path globs and exact validation commands; ensure concurrent cards are mechanically disjoint.
6. Merge one wave at a time, run its audit card, then update `WORK_BOARD.md`.

Suggested branch naming: `work/<job-id>-short-name`. Push only when the owner explicitly requests it.

## Planning doctrine

- Minimum context, tokens, code, and model; maximum useful work.
- Deterministic and indexed paths before semantic models.
- Contracts and negative fixtures before implementations.
- One bounded logical job per agent; interrupted work may resume only that same card and attempt count.
- Human/deterministic ground truth; no model-majority acceptance.
- A second agent reviews security-sensitive or architecture-sensitive changes.
- Two failed attempts on the same blocker means stop and hand off evidence.
