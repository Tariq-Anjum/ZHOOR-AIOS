# Orchestrator runbook for Herdr Runtime Server sessions

The human owner assigns jobs. Agents do not message one another; isolated worktrees, diffs/commits, tests, and handoffs coordinate work.

## Runtime lanes

| Runtime surface | Role | Cards | Notes |
|---|---|---|---|
| Herdr Runtime Server | implementation/evidence worker | bounded implementation and evidence cards | Use the job's stable role; see current concrete assignments in `MODEL_AND_SKILL_MATRIX.md`. |
| Herdr Runtime Server | architecture-primary / independent-challenger | architecture and review cards | Different reviewers and independent sessions; neither may have authored the reviewed tranche. |
| Herdr Runtime Server | exact-SHA auditor | `AUD-02` and owner-designated final audits | Must be independent of the closure authors; see the current reviewer mapping. |

Start only agents needed for the current wave.

## Handoff and lane availability

1. Start every card in a named Herdr session `zhoor-<JOB-ID>`.
2. If an agent becomes unavailable, hand the same card and attempt count to another owner-assigned session using its stable job role.
3. The implementer and independent reviewer must be different agents in separate sessions. An agent/model that authored the reviewed tranche cannot act as its independent challenger.
4. Do not require a particular model/provider family when authorship and session separation establish independence.
5. A handoff never changes the card, branch, write scope, or acceptance gate. Record any substitution in `DECISIONS.md`.

## Wave order

| Wave | Cards | Parallel? | Gate |
|---|---|---|---|
| 0 | ARC-01 | no | Owner adjudicates two independent Herdr reports |
| 1 | ARC-02 | no | Bounded evidence complete, then separate independent review before ARC-02 DONE |
| 2 | ARC-03 | no | Consume accepted ARC-02 evidence; owner accepts production ADR, paths, commands |
| 3 | BASE-01 | no | Clean canonical checks |
| 4 | PRO-01 | no | Protocol review merged |
| 5 | REG-01, POL-01, TRC-01, SEC-01 | yes, pinned disjoint worktrees | Individual reviews and full checks |
| 6 | WRK-01, WRK-02 | yes | Workers merged |
| 7 | EVAL-00 | no | Harness contract review |
| 8 | EVAL-01, EVAL-02 | yes | First batches human spot-checked |
| 9 | EVAL-03, EVAL-04 | yes | 50–100 cases/worker and aggregate reports |
| 10 | AUD-01 | no | Independent Herdr audit and owner approval |
| 11 | BENCH-01 | no | Adapter/runner review; no live candidate run |
| 12 | BENCH-02 | no | Owner approves any install/network; measured report reviewed |
| 13 | SHADOW-01 | no | Shadow evidence reviewed |
| 14 | GRANT-01 | no | Policy trust path/security review |
| 15 | AIOSD-01 | no | Enforcement/receipt review |
| 16 | AIOSD-02 | no | Unix peer/lifecycle review |
| 17 | AUD-02 | no | Sol final audit plus human promotion decision |
| 18 | CANARY-01 | no | Proposal ADR only |

## Worktree and status policy

- Provide one isolated `work/<job-id>-short-name` worktree/branch per active card.
- Merge dependency branches before creating dependent worktrees.
- `WORK_BOARD.md` is the only status source. Implementation agents accept `READY`; review agents accept `REVIEW`.
- ARC-03 must pin exact path globs and validation commands before downstream cards become `READY`.
- No card permits push. Explicitly add `commit: yes/no` and `push: yes/no` to assignments.

## Milestone publication

After a milestone's implementation/review is complete, deterministic checks pass, the independent exact-SHA audit passes, and protected status is reconciled, the orchestrator may make a normal non-force push of `integration/zhoor-adoption` to `origin/integration/zhoor-adoption` without another owner interaction. Fetch and verify the remote first; if it has diverged from the known branch, stop. This standing authorization excludes `main`, tags, releases, force pushes of any kind, history rewriting, branch deletion, and repository settings.

## Assignment prompt

```text
You are assigned [JOB-ID] only in the ZHOOR-AIOS repository.
Use the Herdr Runtime Server named session `zhoor-[JOB-ID]`. Preserve the assigned worktree, branch, and attempt count.
Read docs/planning/zhoor-adoption/README.md, AGENT_OPERATING_RULES.md, and jobs/[JOB-ID].md; then read only card-listed context.
Use skill-router and karpathy-guidelines: state assumptions, do not overbuild, stay in scope, optimize for the success condition.
Worktree/branch: [PATH / BRANCH]. Use only the write scope pinned by your card. ARC-02 uses its explicit pre-ARC-03 evidence area; downstream implementation cards obey ARC-03-pinned paths and validation commands. Do not switch branches.
Auto-approve routine reversible project-local reads/edits/checks. Ask before destructive, network/install, credential, push/merge/release, architecture, grilling, or final acceptance.
After two failures on one blocker, stop. At ~60% context, validate and update docs/planning/zhoor-adoption/handoffs/[JOB-ID].md.
Do not talk to other agents or edit WORK_BOARD.md. Before stopping, write the handoff and pass it to the next agent through Herdr. Commit: [yes/no]. Push: [yes/no].
```

## Review prompt

```text
Review [JOB-ID] only in a separate Herdr session with a different reviewer from the implementer. The board status is REVIEW. Read the card, immutable implementation handoff, acceptance criteria, and provided diff/commit. Load skill-router and karpathy-guidelines. Do not edit implementation or its handoff. Independently run the exact targeted/full checks. Write docs/planning/zhoor-adoption/handoffs/reviews/[JOB-ID]-[REVIEWER].md using templates/REVIEW_TEMPLATE.md, where [REVIEWER] is a unique runtime reviewer token. Record your role, runtime session, and exact commit. An independent challenger does not read the primary report before writing its own and must have no authorship conflict. After two validation failures, stop and report blocked. Approval is evidence-based, never model consensus.
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
- Architecture/security conflict: stop for the owner-designated audit path.
- Install/network: stop for approval and document version, license, privacy, telemetry, cost, and rollback.

## Cost controls

- Use one owner-assigned agent for each bounded card; preserve the stable role and independence requirements in the card.
- One implementer plus one reviewer; do not ask multiple models to duplicate implementation.
- No Needle, Jev, BMAD, or OpenSpec installation in early waves.
- End sessions after handoff; do not keep agents resident while blocked.

## Continuity and source of truth

Use Git, runtime observations, and `STATE.md` to establish factual state. They are not policy authority. For normative project authority, use the canonical hierarchy in ADR-0001:

1. Current Constitution.
2. Accepted ADRs.
3. Normative ZHOOR architecture.
4. Accepted contracts and schemas.
5. ZHOOR adoption/governance controls.
6. Legacy HERMES material.
7. Raw discussions/chat archives.

Conversation/runtime observations are evidence, not durable project authority, until recorded through the appropriate repository governance layer. Operational owner decisions may be recorded in `DECISIONS.md`; architectural changes require the applicable ADR/governance process.

The canonical implementation repository is `/mnt/MD/AI-tools/ZHOOR-AIOS`,
reconciled against Git evidence and external `STATE.md`. The former separate
`/mnt/MD/AI-tools/zhoor` workspace is archived at
`/mnt/MD/Archive/ZHOOR-precanonical-2026-09-28/legacy-docs/zhoor`; it is
historical context, not an active project workspace. Preserve pre-existing
dirty and untracked paths; inspect status before work and stage only task-owned
paths and hunks.

ADR-0002 is a narrow exception for the completed rootless, read-only
`system.info.read` slice. It does not authorize arbitrary capability growth,
satisfy ARC-02/ARC-03 or later cards, make the slice `aiosd`, change the remaining
ARC governance gates, or bypass the ordered evaluation, grant, `aiosd`, audit,
and canary sequence. Any further deviation requires an explicit accepted ADR.

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

## Automated Herdr topology

The controller tab and every automated role tab contain exactly one
interactive workload pane. Herdr-managed UI panes are permitted only when
their role is positively identified from live pane metadata. The current
managed UI classification is Herdr's `Sidebar` label with `herdr-sidebar-*`
tokens. Unknown additional panes and additional terminals/agents block the
lane; total pane count alone is not the acceptance check. Automated delegation
creates a new tab in the explicitly selected live workspace with `--no-focus`
and verifies its returned root pane, assigned CWD, and interactive pane
inventory. On Herdr 0.9.1, creation may temporarily select that tab, including
after the first focus snapshot; recheck after pane inventory, restore the
previously recorded controller tab with the supported focus command, and
verify it before starting or messaging the subordinate. A failed restoration
or topology check blocks the lane. There is no pane-split fallback. Persistent
shell utilities use their own tab with one interactive shell pane and the same
managed UI classification.

Use `scripts/zhoor_herdr.py` for automated lane creation and agent messaging.
It records a controller-owned lane registry under
`${XDG_STATE_HOME:-$HOME/.local/state}/zhoor-orchestrator/`, rejects duplicate
live logical roles, and retains completed tabs for inspection. Its only
pre-containment prompt is a fixed role-only handshake. The controller records
`AGENT_CONTAINMENT=PASS` with sanitized evidence after checking route, config,
children, and filesystem/runtime boundaries; the helper refuses all further
prompts until that gate is recorded. Lane IDs are re-derived from current
Herdr responses and stored for the current run; do not recover a lane by
focusing its tab. The current verified subordinate profile is Pi with an
allowlisted environment, isolated HOME/XDG/Pi/temp directories, read-only
file tools, and a controller-managed guard that blocks paths outside the
assigned worktree. The helper fails closed for harnesses without a verified
containment profile. Reviewer CWD must equal the assigned worktree root,
because the guard takes its boundary from Pi's process CWD; descendant CWDs are
rejected to avoid narrowing the readable worktree.

For permanent reviewer lanes, select only an explicit helper tier:
`--audit-tier primary` launches Pi with `openai` / ChatGPT subscription OAuth,
`gpt-6.1-sol`, and `high`; `--audit-tier secondary` fixes the same provider and
model at `xhigh`. The helper rejects provider/model/reasoning overrides and
unapproved Pi options. `openai-codex` is legacy and is not an accepted ZHOOR
review route. The helper requires the provider credential to be OAuth with the
installed ChatGPT direct-use scope; API-key credentials are rejected. It
passes the complete credential through an anonymous memfd-backed handoff. Pi
may refresh that lane-local copy; the controller's private source credential
is not modified by the reviewer. No regular auth file is created. Never print
or copy credential values.

Before either route starts, verify `pi auth check --provider openai --json
--no-refresh` reports `ready`, `openai`, and `oauth`; when ready, repeat the
normal readiness check without `--credentials` so Pi can refresh its own
credential. Verify the installed Pi catalog exposes `openai/gpt-6.1-sol` and
supports the tier's reasoning effort. If auth or model availability fails,
stop; do not fall back to an API key, `openai-codex`, or weaker containment.

Reviewer launches use exactly `--tools read,grep,find,ls` and preserve
`--no-extensions`, `--no-skills`, `--no-prompt-templates`, `--no-themes`,
`--no-context-files`, and `--no-approve`. The helper adds only its own
controller-managed read-only guard extension. No model-facing bash, write,
edit, codemode, tool-search, MCP tool, or other extension is permitted. Pi's
interactive `!` and `!!` commands reach `user_bash`; the guard replaces that
event with a bounded denial and exit 126 before local shell execution.

The primary auditor is the normal independent reviewer for important
implementation cards. Require the secondary critical auditor for security or
trust boundaries, authentication/authorization, grant/trust-root,
execution/enforcement, privilege, identity, daemon/service lifecycle,
transport, filesystem enforcement, architecture/interface changes, important
milestone closure, material primary `CHANGES_REQUIRED`, unresolved reviewer /
controller disagreement, and containment/orchestration security changes. Keep
the secondary report blinded to the primary verdict and findings until it has
formed an independent verdict. Workers remain on their existing routes.

Before relying on a topology change, test tab creation, no-focus behavior,
one-pane verification, role uniqueness, fail-closed behavior, and prompt/read/
wait/status/stop operations. Do not create demonstration agents with source
access merely to inspect the layout.

## Roles and exact-SHA audit

- **Codex** orchestrates, integrates, and writes the primary worktree.
- **Pi** may independently verify and is read-only during audits. Every audit is
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
