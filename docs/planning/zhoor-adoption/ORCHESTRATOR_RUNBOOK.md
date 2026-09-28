# Orchestrator runbook for Pi, OpenCode, and Codex

The human owner assigns jobs. Agents do not message one another; isolated worktrees, diffs/commits, tests, and handoffs coordinate work.

## Recommended lanes

| Harness | Model | Primary cards | Review cards |
|---|---|---|---|
| Codex | GPT-5.6 Sol | ARC-01, ARC-03, AUD-01, AUD-02, CANARY-01 | ARC-02 (inside ARC-03), SEC-01, GRANT-01, BENCH-01/02, AIOSD-01/02 |
| Pi | GLM 5.3 Flash | BASE-01, PRO-01, REG-01, WRK-02, AIOSD-02 | bounded implementation review |
| Pi | Qwen3.8 Max | ARC-02, SEC-01, GRANT-01 | ARC-01 challenger only by recorded owner waiver (author conflict), ARC-03 challenger, POL-01, WRK-02, EVAL-03 |
| Pi | Qwen3.8 Flash | EVAL-02 | registry/fixture bulk review |
| OpenCode | Muse Spark 1.3 | TRC-01, WRK-01, BENCH-01, AIOSD-01 | bounded runtime review |
| OpenCode | DeepSeek V4.1 Flash | EVAL-00, EVAL-03, BENCH-02, SHADOW-01 | TRC-01, EVAL-01, EVAL-04 |
| Codex | GPT-5.6 Luna | POL-01, EVAL-01, EVAL-04 | ARC-01 challenger (recommended), BASE-01, PRO-01, WRK-01, EVAL-02, SHADOW-01 |

Start only agents needed for the current wave.

## Lane unavailability

GPT-5.6 Sol runs through the Codex harness. The former `Lead/Zed` lane is retired (owner-reported Zed credit exhaustion); no card depends on the Zed harness.

When a card requires the Sol lane and that lane is unavailable:

1. The owner may substitute Qwen3.8 Max (Pi) as `interim-primary` — unless that model authored the artefact under review — and the substitution must be recorded in `DECISIONS.md` before work starts.
2. The independent challenge is then owed by a different model family — GPT-5.6 Luna (Codex), or GPT-5.6 Sol once the lane returns — before the card may become `DONE`.
3. One model never writes both the primary and the challenger report for the same card, and an agent that helped author the artefact under review is never its only reviewer and never its independent challenger.
4. Substitution never lowers an acceptance gate: architecture/security acceptance stays with Sol plus the human owner even if it is delivered later.

## Wave order

| Wave | Cards | Parallel? | Gate |
|---|---|---|---|
| 0 | ARC-01 | no | Owner adjudicates two independent reports: Sol (Codex) primary + non-Qwen challenger (Luna recommended) |
| 1 | ARC-02 | no | Evidence complete, no decision |
| 2 | ARC-03 | no | Sol + Qwen challenger + owner accept ADR, paths, commands |
| 3 | BASE-01 | no | Clean canonical checks |
| 4 | PRO-01 | no | Protocol review merged |
| 5 | REG-01, POL-01, TRC-01, SEC-01 | yes, pinned disjoint worktrees | Individual reviews and full checks |
| 6 | WRK-01, WRK-02 | yes | Workers merged |
| 7 | EVAL-00 | no | Harness contract review |
| 8 | EVAL-01, EVAL-02 | yes | First batches human spot-checked |
| 9 | EVAL-03, EVAL-04 | yes | 50–100 cases/worker and aggregate reports |
| 10 | AUD-01 | no | Explicit Sol + owner approval |
| 11 | BENCH-01 | no | Adapter/runner review; no live candidate run |
| 12 | BENCH-02 | no | Owner approves any install/network; measured report reviewed |
| 13 | SHADOW-01 | no | Shadow evidence reviewed |
| 14 | GRANT-01 | no | Policy trust path/security review |
| 15 | AIOSD-01 | no | Enforcement/receipt review |
| 16 | AIOSD-02 | no | Unix peer/lifecycle review |
| 17 | AUD-02 | no | Human promotion decision |
| 18 | CANARY-01 | no | Proposal ADR only |

## Worktree and status policy

- Provide one isolated `work/<job-id>-short-name` worktree/branch per active card.
- Merge dependency branches before creating dependent worktrees.
- `WORK_BOARD.md` is the only status source. Implementation agents accept `READY`; review agents accept `REVIEW`.
- ARC-03 must pin exact path globs and validation commands before downstream cards become `READY`.
- No card permits push. Explicitly add `commit: yes/no` and `push: yes/no` to assignments.

## Assignment prompt

```text
You are assigned [JOB-ID] only in the HERMES-AIOS → ZHOOR repository.
Read docs/planning/zhoor-adoption/README.md, AGENT_OPERATING_RULES.md, and jobs/[JOB-ID].md; then read only card-listed context.
Use skill-router and karpathy-guidelines: state assumptions, do not overbuild, stay in scope, optimize for the success condition.
Worktree/branch: [PATH / BRANCH]. Obey the ARC-03-pinned paths and validation commands. Do not switch branches.
Auto-approve routine reversible project-local reads/edits/checks. Ask before destructive, network/install, credential, push/merge/release, architecture, grilling, or final acceptance.
After two failures on one blocker, stop. At ~60% context, validate and update docs/planning/zhoor-adoption/handoffs/[JOB-ID].md.
Do not talk to other agents or edit WORK_BOARD.md. Commit: [yes/no]. Push: [yes/no].
```

## Review prompt

```text
Review [JOB-ID] only. The board status is REVIEW. Read the card, immutable implementation handoff, acceptance criteria, and provided diff/commit. Load skill-router and karpathy-guidelines. Do not edit implementation or its handoff. Independently run the exact targeted/full checks. Write docs/planning/zhoor-adoption/handoffs/reviews/[JOB-ID]-[REVIEWER].md using templates/REVIEW_TEMPLATE.md, where [REVIEWER] is your reviewer token (sol, luna, qwen-max, qwen-flash, glm, muse, deepseek, owner). Record your role (primary or independent-challenger), model/harness, and the exact commit you reviewed. An independent challenger does not read the primary report before writing its own. After two validation failures, stop and report blocked. Approval is evidence-based, never model consensus.
```

## Context reset/resume

1. Resume only the same logical card; do not add another card.
2. Read the card and existing implementation handoff first.
3. Run `git status` and inspect only the owned diff.
4. Continue from the handoff's next action.
5. Preserve the original two-attempt count.
6. Update the same implementation handoff; reviewers always use unique review files.

## Failure handling

- Missing dependency: block; do not emulate it.
- Merge conflict in owned files: return to orchestrator.
- Pre-existing failure: record exact evidence; do not fix unrelated work.
- Ambiguity: use one stated conservative reversible interpretation only when within scope; otherwise stop.
- Architecture/security conflict: stop for Sol/human review.
- Install/network: stop for approval and document version, license, privacy, telemetry, cost, and rollback.

## Cost controls

- GLM/Muse: bounded implementation; Luna/Flash: fixtures, datasets, routine review; Max/Sol: hard decisions/security.
- One implementer plus one reviewer; do not ask multiple models to duplicate implementation.
- No Needle, Jev, BMAD, or OpenSpec installation in early waves.
- End sessions after handoff; do not keep agents resident while blocked.

## Continuity and source of truth

Use this priority when records disagree:

1. Git repository state.
2. Accepted architecture and ADRs.
3. This tracked runbook and other tracked project contracts.
4. Local orchestration `STATE.md` checkpoint.
5. Current runtime observations.
6. Conversation memory, which is never required for recovery.

The canonical implementation repository is determined from recorded project
state and Git evidence, not from a directory name. Do not move or merge the
separate `/mnt/MD/AI-tools/zhoor` material as part of ordinary implementation.
Preserve pre-existing dirty and untracked paths; inspect status before work and
stage only task-owned paths and hunks.

ADR-0002 is a narrow exception for the completed rootless, read-only
`system.info.read` slice. It does not authorize arbitrary capability growth,
change the remaining ARC governance gates, or bypass the ordered evaluation,
grant, `aiosd`, audit, and canary sequence. Any further deviation requires an
explicit accepted ADR.

## Runtime discovery and handshake

Create a fresh run ID for every new orchestrator session and a unique task ID
for each worker assignment. Workers identify the run, task, role, and reviewed
SHA when applicable. Ignore responses from other run IDs.

At each fresh run, discover live workers with `herdr --version` and
`herdr agent list`. Never treat pane/session IDs as durable identity. Derive a
current target from the live listing, then use `herdr agent get`, `read`, or
`explain` when identity or state is ambiguous. Check installed command help;
local Herdr help is the syntax authority. Complete a role-only handshake before
assigning source review. Record observed model and provider separately; unknown
provider, authentication, or billing details stay `UNKNOWN`.

Send multiline assignments from a temporary packet created with a quoted
here-document, then pass its contents to Herdr. Use a native file/stdin option
only when the installed help documents one. Never build long packets by
manually nesting shell quotes or include secrets.

Track work through `CREATED → SENT → ACKNOWLEDGED → WORKING → RESULT_RECEIVED
→ VERIFIED → CLOSED`. If interrupted, inspect the task, Git, and worker state
for an existing result before resending; a slow task is not a failed task.

## Roles and exact-SHA audit

- **Codex** orchestrates, integrates, and writes the primary worktree.
- **Pi** independently verifies and is read-only during audits. Every audit is
  pinned to an exact SHA in a detached checkout; deterministic checks are
  allowed, but Pi does not commit or push.
- **OpenCode** is an optional auxiliary worker with no architecture authority.
  Use only when justified, in an isolated branch/worktree with explicit
  authorization; default to sanitized/public data under the Muse Contributor
  boundary.

Prepare the audit worktree at `/tmp/zhoor-audit/<RUN_ID>` and pin it to the
candidate SHA. The packet names the canonical repository, audit checkout, base
and candidate SHAs, changed paths, relevant architecture contract, acceptance
criteria, test commands, and forbidden modifications. A report is valid only
when it identifies the reviewed SHA. Require `RUN_TASK`, `AUDIT_RESULT`,
`REVIEWED_SHA`, `FINDINGS`, `RECOVERY_SIMULATION`, and `RESIDUAL_RISK` as
applicable. Product-specific audit actions are optional; Herdr, Pi, the pinned
checkout, and deterministic commands are the verification path.

Codex owns the main worktree. Pi does not edit during audits. OpenCode writes
only to an isolated branch/worktree when explicitly assigned.

## Fresh-session recovery and compaction

For a fresh session: read repository `AGENTS.md`, this runbook, and the local
`STATE.md`; reconcile Git status, branch, HEAD, and recent commits; identify
protected dirty paths; rediscover Herdr agents; perform a fresh handshake; then
identify the current and next milestone. If HEAD differs from the checkpoint,
reconcile the difference before continuing.

Before context compaction or ending a session, update
`${XDG_STATE_HOME:-$HOME/.local/state}/zhoor-orchestrator/STATE.md` with the
full HEAD, branch, protected dirty/untracked paths, milestone and exact-SHA
audit status, unresolved blockers, and exactly one next action. Keep runtime
IDs out of durable state. Use `/tmp` only for disposable packets, worktrees,
build output, and scratch files. After compaction, recover from repository
instructions, this runbook, the checkpoint, Git, and current Herdr discovery;
do not rebuild the session from conversational summaries.
