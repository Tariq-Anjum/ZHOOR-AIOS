# Next actions — ARC-01 review wave (owner checklist)

Branch: `integration/zhoor-adoption` · Push: **no** (owner-only) · Status authority: [`../WORK_BOARD.md`](../WORK_BOARD.md)

State at the time of writing: the governance tranche is committed on the branch (audit base `dbffd08`); the
Sol lane has moved from Zed to **Codex** and the review-file naming is now reviewer-token based
(`D-010`…`D-013` in [`../DECISIONS.md`](../DECISIONS.md)). `ARC-01` is `REVIEW`; every other card is `BLOCKED`.

Authorship provenance of the wave-0 artefact, per commit: `1904baf` — GPT-5.6 Sol (Zed harness);
`148d103` — Qwen3.8 Max (Pi), after Zed credits were exhausted; `2efdbd3` and `2f3270f` — Qwen3.8 Max (Pi),
this session (`PI_MODEL=qwen/qwen3.8-max-0902`). The handoff/lane layer is therefore Qwen-authored, which
constrains who may act as independent challenger — see section 2.

## 0. Baseline validation — already re-run, expected green

| Check | Result |
|---|---|
| `git diff --check` and `git diff --cached --check` (committed tree) | clean |
| Markdown relative-link sweep over `docs/planning/zhoor-adoption/` (42 files) | 0 broken links |
| Card parity, compared as sets | 25 card files = 25 board rows = 25 `jobs/README.md` IDs = 25 `ORCHESTRATOR_RUNBOOK.md` IDs |

Both reviewers must reproduce this table independently. Any deviation is a finding, not repo drift.

One documented exception: all 25 job cards use two-space Markdown hard breaks in their header block, so an
*uncommitted* diff that touches a header line makes `git diff --check` report trailing whitespace by design.
Run the check on a committed tree; that specific report is not a finding.

## 1. ARC-01 primary review — Codex / GPT-5.6 Sol

Paste into the Codex (GPT-5.6 Sol) session.

```text
You are assigned ARC-01 (governance baseline review) as PRIMARY reviewer only, in the repository at
/mnt/MD/AI-tools/zhoor-repo-review, branch integration/zhoor-adoption. Do not switch branches.
Record the exact commit you reviewed (git rev-parse HEAD) in the report's Scope section.

Read, in order:
1. docs/planning/zhoor-adoption/README.md
2. docs/planning/zhoor-adoption/AGENT_OPERATING_RULES.md
3. docs/planning/zhoor-adoption/jobs/ARC-01.md
Then read only the context ARC-01 lists:
- README.md (repository root)
- .hermes.md
- docs/constitution/HERMES_CONSTITUTION.md
- docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md
- docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md
- docs/planning/zhoor-adoption/ROADMAP.md
- docs/planning/zhoor-adoption/WORK_BOARD.md

Load skill-router and karpathy-guidelines: state assumptions, do not overbuild, stay in review scope,
judge against the card's success condition. This is review, not implementation or redesign.

Write scope — your single write is:
docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-sol.md
using docs/planning/zhoor-adoption/templates/REVIEW_TEMPLATE.md
Set Role: primary, Model/harness: GPT-5.6 Sol / Codex.
Verdict: approve | approve-with-nonblocking-notes | changes-required | blocked
Do NOT edit any governance document, job card, the board, or another reviewer's file.

Run the ARC-01 checks:
- Repository rename deferred without ambiguity (ADR-0001 + constitution + root README agree)
- Legacy HERMES docs non-normative where they conflict (SYSTEM_OVERVIEW,
  09_IMPLEMENTATION_ROADMAP, Kanban banners present)
- First slice has no model dependency and no autonomous effect
- Memory / policy / inference / execution ownership does not overlap
- WORK_BOARD dependencies match the ROADMAP.md mermaid graph
- Every job is a bounded logical unit; every architecture/security change has independent review
- Lane and ownership naming agrees across WORK_BOARD.md, ORCHESTRATOR_RUNBOOK.md,
  MODEL_AND_SKILL_MATRIX.md, DECISIONS.md and the cards (GPT-5.6 Sol on Codex; no retired
  Zed lane references); review filenames are <JOB-ID>-<reviewer-token>.md

Independently re-run mechanical validation and record results in the template's validation table
(all three are expected green — see handoffs/NEXT_ACTIONS.md section 0):
- git diff --check and git diff --cached --check (run on the committed tree; see the hard-break
  exception in handoffs/NEXT_ACTIONS.md section 0)
- markdown relative-link sweep over docs/planning/zhoor-adoption/
- card parity: 25 card files = 25 board rows = jobs/README.md IDs = ORCHESTRATOR_RUNBOOK.md IDs
  (compare ID sets, not counts)

Stop rule: if you fail twice to verify a contradiction, record the evidence and stop — do not fix.
No commit, no push, no network, no installs. Ask the owner before anything outside this card.
At ~60% context, write the report and stop.

Commit: no. Push: no.
```

## 2. ARC-01 independent challenge — choose the reviewer first

May run in parallel with section 1.

**Author conflict — resolve before assigning.** The handoff/lane layer of the artefact under review
(`148d103`, `2efdbd3`, `2f3270f`) was prepared by Qwen3.8 Max (Pi), so a Qwen3.8 Max challenge would partly be a
self-review; `1904baf` itself was Sol-authored. This is the `ARC-01 challenger independence` open gate in
[`../DECISIONS.md`](../DECISIONS.md).

- **2a. Recommended — Codex / GPT-5.6 Luna** → writes `ARC-01-luna.md`. Non-Qwen, independent of the author,
  and cheap enough for a review card. Same-family-as-primary (GPT-5.6) is accepted here because the primary and
  challenger are different models and the author is neither.
- **2b. Only with an explicit owner waiver recorded in `../DECISIONS.md` — Pi / Qwen3.8 Max (herdr)** → writes
  `ARC-01-qwen-max.md`. Strongest review model available, but it is grading its own prior work.

Substitute the marked lines for the chosen reviewer, then paste the block.

```text
You are assigned ARC-01 (governance baseline review) as INDEPENDENT CHALLENGER only, in the repository
at /mnt/MD/AI-tools/zhoor-repo-review, branch integration/zhoor-adoption. Do not switch branches.
Record the exact commit you reviewed (git rev-parse HEAD) in the report's Scope section.

Do NOT read docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-sol.md — not before your own report
is written. Your value is independence; the owner adjudicates the two reports.

Author check: the handoff/lane layer of the artefact under review was prepared by Qwen3.8 Max (Pi); commit
1904baf was prepared by GPT-5.6 Sol. If you are Qwen3.8 Max, say so explicitly in the report's Review ownership
section and note the owner waiver that authorizes it; otherwise record that you are independent of the author.

Read, in order:
1. docs/planning/zhoor-adoption/README.md
2. docs/planning/zhoor-adoption/AGENT_OPERATING_RULES.md
3. docs/planning/zhoor-adoption/jobs/ARC-01.md
Then read only the context ARC-01 lists:
- README.md (repository root)
- .hermes.md
- docs/constitution/HERMES_CONSTITUTION.md
- docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md
- docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md
- docs/planning/zhoor-adoption/ROADMAP.md
- docs/planning/zhoor-adoption/WORK_BOARD.md

Load skill-router and karpathy-guidelines: state assumptions, do not overbuild, stay in review scope,
judge against the card's success condition. This is review, not implementation or redesign.

Write scope — your single write is:
docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-REVIEWER.md
using docs/planning/zhoor-adoption/templates/REVIEW_TEMPLATE.md
Set Role: independent-challenger, Model/harness: MODEL / HARNESS.
Before pasting, substitute either:
  REVIEWER=luna      MODEL=GPT-5.6 Luna   HARNESS=Codex   (recommended, section 2a)
  REVIEWER=qwen-max  MODEL=Qwen3.8 Max    HARNESS=Pi      (only with the recorded owner waiver, 2b)
Verdict: approve | approve-with-nonblocking-notes | changes-required | blocked
Do NOT edit any governance document, job card, the board, or another reviewer's file.

Run the ARC-01 checks, and actively try to break the baseline rather than confirm it:
- Repository rename deferred without ambiguity (ADR-0001 + constitution + root README agree)
- Legacy HERMES docs non-normative where they conflict (SYSTEM_OVERVIEW,
  09_IMPLEMENTATION_ROADMAP, Kanban banners present)
- First slice has no model dependency and no autonomous effect
- Memory / policy / inference / execution ownership does not overlap
- WORK_BOARD dependencies match the ROADMAP.md mermaid graph
- Every job is a bounded logical unit; every architecture/security change has independent review
- Lane and ownership naming agrees across WORK_BOARD.md, ORCHESTRATOR_RUNBOOK.md,
  MODEL_AND_SKILL_MATRIX.md, DECISIONS.md and the cards (GPT-5.6 Sol on Codex; no retired
  Zed lane references); review filenames are <JOB-ID>-<reviewer-token>.md
- Substitution path: if a lane is unavailable, ORCHESTRATOR_RUNBOOK.md section "Lane unavailability"
  plus DECISIONS.md D-013 still guarantee an independent report from a different model family

Independently re-run mechanical validation and record results in the template's validation table
(all three are expected green — see handoffs/NEXT_ACTIONS.md section 0):
- git diff --check and git diff --cached --check (run on the committed tree; see the hard-break
  exception in handoffs/NEXT_ACTIONS.md section 0)
- markdown relative-link sweep over docs/planning/zhoor-adoption/
- card parity: 25 card files = 25 board rows = jobs/README.md IDs = ORCHESTRATOR_RUNBOOK.md IDs
  (compare ID sets, not counts)

Stop rule: if you fail twice to verify a contradiction, record the evidence and stop — do not fix.
No commit, no push, no network, no installs. Ask the owner before anything outside this card.
At ~60% context, write the report and stop.

Commit: no. Push: no.
```

## 2c. If the Codex / GPT-5.6 Sol lane is unavailable

Do not silently collapse ARC-01 into a single Qwen3.8 Max report. Per
[`../ORCHESTRATOR_RUNBOOK.md`](../ORCHESTRATOR_RUNBOOK.md) §Lane unavailability and `D-013`:

1. Owner records the substitution in [`../DECISIONS.md`](../DECISIONS.md) first.
2. Qwen3.8 Max (Pi) runs section 2's prompt with `Role: interim-primary` in `ARC-01-qwen-max.md`.
   Because Qwen3.8 Max is also the tranche's author, this route leaves **no** author-independent
   reviewer until step 3 is completed.
3. The independent challenge is then owed by a different model family — GPT-5.6 Luna (Codex)
   writing `ARC-01-luna.md`, or Sol when the lane returns.
4. `ARC-01` stays `REVIEW` until that second report exists. Architecture/security acceptance is
   not lowered, only deferred.

## 3. Owner checklist after the reports return

1. Read `reviews/ARC-01-sol.md` and the challenger report (`reviews/ARC-01-luna.md`, or
   `reviews/ARC-01-qwen-max.md` if the owner waiver in section 2b was recorded).
2. Adjudicate per [`reviews/README.md`](reviews/README.md): tests, deterministic evidence,
   architecture authority, then human judgment — never model voting or averaging.
3. Both verdicts `approve` / `approve-with-nonblocking-notes` → tell Pi/GLM:
   - Pi flips `ARC-01` → `DONE`, sets `ARC-02` → `READY`, rewrites "Current owner action".
   - Pi prepares the ARC-02 assignment prompt (toolchain evidence spike, Qwen3.8 Max primary;
     GPT-5.6 Sol reviews that evidence inside ARC-03).
4. Any `changes-required` → the governance fixes belong to the **Codex / GPT-5.6 Sol** lane
   (above the GLM 5.3 Flash tier). Pi prepares a changes-required packet: exact file:line
   findings from both reports, the owning lane, and the re-review requirement.
5. Any `blocked` → read the recorded evidence; the owner decides the next step. Pi does not
   attempt the fix.
6. Fallback: if a reviewer writes to a wrong path or stops half-finished, Pi may relocate or
   mechanically complete the file and re-run validation — report *content* stays the reviewer's,
   and Pi records what it touched.

## 4. Entry point for the next agent

Read in this order, then stop and wait for an assignment:

1. [`../README.md`](../README.md) — authority order and workspace map.
2. [`../AGENT_OPERATING_RULES.md`](../AGENT_OPERATING_RULES.md) — startup, context, retry, approval, Git, completion.
3. [`../WORK_BOARD.md`](../WORK_BOARD.md) — the only status authority; current owner action at the bottom.
4. This file — the live wave.
5. [`../ORCHESTRATOR_RUNBOOK.md`](../ORCHESTRATOR_RUNBOOK.md) — lanes, wave order, prompts, resume protocol.
6. Only then, the assigned `../jobs/<JOB-ID>.md` and the context it lists.

Current wave: **0** (`ARC-01`). `ARC-02` is the only wave-1 card and stays `BLOCKED` until
`ARC-01` is `DONE`.

Lane summary: Codex/GPT-5.6 Sol — architecture and security acceptance; Pi/Qwen3.8 Max —
independent challenger plus `ARC-02`, `SEC-01`, `GRANT-01`; Pi/GLM 5.3 Flash — bounded
implementation and board/handoff edits; OpenCode and Codex/Luna lanes per the runbook table.

Hard rules for every agent: no push, no branch switching, no board edits by workers, one card per
session, two-failure stop, one handoff per card. Durable memory stays in `my-ai-brain`; record only
durable, non-sensitive outcomes.
