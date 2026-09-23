# Next actions — post-commit (owner checklist)

Branch: `integration/zhoor-adoption` · Commit: `1904baf` · Push: **no** (owner-only)

## 1. ARC-01 governance review — assign to Qwen3.8 Max (herdr)

Paste the block below into the Qwen3.8 Max agent session.

```text
You are assigned ARC-01 (governance baseline review) only, in the repository at
/mnt/MD/AI-tools/zhoor-repo-review, branch integration/zhoor-adoption, commit 1904baf.

Read, in order:
1. docs/planning/zhoor-adoption/README.md
2. docs/planning/zhoor-adoption/AGENT_OPERATING_RULES.md
3. docs/planning/zhoor-adoption/jobs/ARC-01.md
Then read only what ARC-01 lists as required context: root README.md, .hermes.md,
docs/constitution/HERMES_CONSTITUTION.md,
docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md,
docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md, and the workspace ROADMAP.md + WORK_BOARD.md.

Load skill-router and karpathy-guidelines; state assumptions, no redesign, review
scope only. This is review, not implementation.

Do NOT edit any governance document, job card, or the board. Your single write is:
docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-primary.md
using docs/planning/zhoor-adoption/templates/REVIEW_TEMPLATE.md
(verdict: approve | approve-with-nonblocking-notes | changes-required | blocked).

Run the ARC-01 checks:
- Repository rename deferred without ambiguity (ADR-0001 + constitution + README agree)
- Legacy HERMES docs non-normative where they conflict (SYSTEM_OVERVIEW,
  09_IMPLEMENTATION_ROADMAP, Kanban banners present)
- First slice has no model dependency or autonomous effect
- Memory / policy / inference / execution ownership does not overlap
- WORK_BOARD dependencies match ROADMAP.md mermaid graph
- Every job is a bounded logical unit; every architecture/security change has
  independent review

Independently re-run mechanical validation and record results in the template's
validation table:
- git diff --check
- markdown link sweep over docs/planning/zhoor-adoption/
- card parity: 25 card files = 25 board rows = jobs/README IDs = runbook IDs

Stop rule: if you fail twice to verify a contradiction, record the evidence and
stop — do not attempt fixes. No push, no commit, no network. Ask the owner before
anything outside this card. At ~60% context, write the report and stop.

Commit: no. Push: no.
```

## 2. Owner checklist after the review returns

1. Read `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-primary.md`.

2. On `approve` / `approve-with-nonblocking-notes` (tell Pi/GLM):
   - Pi flips ARC-01 → `DONE`, sets ARC-02 ready, updates "Current owner action"
   - Pi prepares the ARC-02 (toolchain evidence spike, Qwen3.8 Max) assignment prompt

3. On `changes-required`: findings go back to the Zed Agent (GPT-5.6 Sol) lane —
   governance fixes are above the GLM 5.3 Flash tier. Pi prepares the
   changes-required packet.

4. On `blocked`: read the recorded evidence; owner decides next step.

Fallback: if the reviewer writes the report to a wrong path or stops half-finished,
Pi may relocate/complete it mechanically and re-run validation — report *content*
stays the reviewer's.
