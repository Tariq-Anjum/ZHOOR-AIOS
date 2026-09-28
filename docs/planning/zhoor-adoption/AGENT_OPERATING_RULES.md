# Agent operating rules

These rules apply to every ZHOOR adoption job, regardless of harness or model.

## Mandatory startup

1. Confirm the assigned job ID and read only its job card.
2. Run `git status` and confirm you are in the worktree/branch assigned by the owner.
3. Read the card's required context only. Do not recursively ingest all legacy documents.
4. Route skills through `skill-router` using the local skill index.
5. Load the Karpathy guidance for every job:
   - registry ID: `karpathy-guidelines`
   - workspace source from the `AI-tools` root: `my-ai-skills/guidance/karpathy-guidelines/GUIDANCE.md` (installed fallback: `skills/installed/karpathy-guidelines/GUIDANCE.md`)
   - apply all four rules: state assumptions, do not overbuild, stay in scope, and optimize for the stated success condition.
6. Restate the job's success condition and write scope before editing.

## Context and token limits

- One card is one bounded logical work unit, normally one session. An interrupted card may resume using the same handoff and attempt count; never pick up a second card.
- Prefer targeted reads and symbol/content search over whole-tree reads.
- Do not read raw discussion files unless the card explicitly lists one.
- Target no more than 8–12 substantive tool cycles before a checkpoint.
- At roughly 60% context use, stop implementation, run the narrowest useful validation, and write the handoff.
- Store durable discoveries in the job handoff, not in chat history.
- Never paste large command logs into documentation; record the command, result, and the few relevant lines.

## Retry and stop rules

- Make at most two focused attempts at the same failing operation or diagnostic.
- After the second failure, stop. Record the exact command, error, attempted fixes, changed files, and best next step in the handoff.
- Stop immediately and ask the owner/orchestrator when:
  - the card's assumptions conflict with repository facts;
  - a dependency is incomplete;
  - work would cross the declared write scope;
  - a schema or security invariant requires a new architecture decision;
  - credentials, secrets, destructive operations, or external spending are required;
  - validation reveals pre-existing failures that obscure the job result.
- Do not enter retry loops, repeatedly reinstall dependencies, or weaken tests to obtain green output.

## Approval behavior

Auto-approve routine, reversible, project-local reads, edits, formatting, and declared validation commands when the harness permits it.

Do not auto-approve:

- destructive deletion or history rewriting;
- network calls, package installation, telemetry, credentials, secrets, or paid APIs not explicitly declared by the card;
- pushing, force-pushing, merging, tagging, releasing, or changing repository visibility;
- architecture scope changes, planning decisions, grilling/interviews, or final review acceptance;
- any operation after the stop conditions above are met.

## Scope and implementation

- Implement the smallest change that meets the card's acceptance criteria.
- Do not opportunistically clean legacy docs or unrelated code.
- Do not add dependencies unless the card authorizes them and records why.
- Do not add a semantic/model path to a deterministic-only card.
- Models provide bounded outputs; they never authorize capability execution.
- Preserve evidence provenance, abstention, denial, and typed error behavior.
- No shell, service control, package installation, network egress, or privileged operation may be made autonomous in these waves.

## Concurrent-agent safety

- Use isolated worktrees for concurrent jobs.
- Obey the card's exclusive write set. If another branch changed one of those files, stop for rebase/reassignment rather than resolving architecture conflicts alone.
- `WORK_BOARD.md` is the only status authority. Implementers accept assigned `READY` cards; reviewers accept assigned `REVIEW` cards.
- Workers do not edit `WORK_BOARD.md`, shared decisions, another worker's implementation handoff, or review files they do not own.
- Implementation handoffs are `docs/planning/zhoor-adoption/handoffs/<JOB-ID>.md`; review handoffs are unique `docs/planning/zhoor-adoption/handoffs/reviews/<JOB-ID>-<REVIEWER>.md` files.
- Communicate only through the assigned worktree diff or owner-authorized commit plus handoffs; do not attempt to message other agents.
- Card paths are repository-relative unless the card explicitly says they are adoption-workspace-relative.
- Descriptive future write areas are not permission to edit. ARC-03 must pin concrete path globs and exact validation commands before a downstream card becomes `READY`.

## Git discipline

- Do not create/switch branches, commit, push, merge, or rebase unless the owner explicitly assigns that operation.
- When commits are requested: one logical conventional commit per card, after declared validation passes.
- Never discard or rewrite unrecognized user/agent changes.
- A blocked job should normally produce only its handoff; do not commit partial speculative implementation unless requested.

## Completion

A job is complete only when:

1. all acceptance criteria are satisfied;
2. for downstream implementation cards, the exact ARC-03-pinned targeted and full validation commands were run and results are recorded truthfully; ARC-02 is the exception because it precedes ARC-03: its evidence handoff must record reproducible spike commands/results and its independent review must pass before ARC-02 is DONE;
3. no files outside the write scope changed;
4. `docs/planning/zhoor-adoption/handoffs/<JOB-ID>.md` records assumptions, files, commands, results, risks, and recommended next action;
5. the agent stops and returns control to the owner/orchestrator.
