# ARC-01 review by sol

## Verdict

**ARC-01 BASELINE NOT YET APPROVED** (`changes-required`). The owner must adjudicate this report independently of the challenger report. ARC-02 is the next *logical* unit after ARC-01 correction and acceptance, but its current write-scope prerequisite must first be made executable.

## Review ownership

- Output: `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-sol.md`
- Role: primary; observed model / harness: GPT-6 Sol / Pi (high).
- Reviewed SHA: `6a73756e8694d7d70e79cb3c8d037b54a79c5a32`, detached checkout `/tmp/zhoor-arc01/20260928-governance-review/sol`.
- Implementation or implementation handoff edited: no. No other reviewer report was consulted.
- Assumption: owner-verified current identity, push, and lane observations take precedence over dated planning statements; prior GPT-5.6 authorship records are historical provenance, not present-day assignments.

## Scope reviewed

Root `README.md`, `.hermes.md`, `AGENTS.md`; constitution; architecture v0.2; ADR-0001/0002; adoption README, roadmap, board, decisions, agent rules, runbook, model/skill matrix, ARC-01 card, review template. To test dependency and state claims also inspected ARC-02/03, BASE-01, SEC-01, GRANT-01 cards, jobs index, active `handoffs/NEXT_ACTIONS.md`, tracked runtime-file inventory, and the bounded `system-info` package README and library. No unrelated historical HERMES plans or other reviewer reports were inspected.

## Blocking/material findings

1. **ARC01-S01 — BLOCKER — `docs/planning/zhoor-adoption/jobs/ARC-02.md:21`, `jobs/ARC-03.md:3,10`, `AGENT_OPERATING_RULES.md:71`, `WORK_BOARD.md:14-15`.** Conflicting authority: the board/roadmap order makes ARC-03 depend on ARC-02, while the rules forbid editing descriptive future write areas before ARC-03 pins exact paths. Evidence: ARC-02's only spike/evidence write area must be “ARC-03-pinned”; ARC-03 is blocked on ARC-02's evidence. Why it matters: after ARC-01 is accepted, ARC-02 still cannot start within its own write scope, so the first dependency gate deadlocks. **Required disposition:** owner/orchestrator must pin ARC-02's isolated spike area and executable validation independently before assigning ARC-02, without prematurely unblocking later implementation cards.

2. **ARC01-S02 — MAJOR — `docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md:32-40`, `docs/planning/zhoor-adoption/README.md:22-31`, `ORCHESTRATOR_RUNBOOK.md` §Continuity and source of truth.** Conflicting authority: the current owner order is explicit owner decisions → accepted ADRs → current ZHOOR architecture → constitution as amended → tracked workspace. Evidence: ADR-0001 and the adoption entry point instead place the current constitution above accepted ADRs; the runbook places Git before accepted decisions without explaining the distinction between factual state and normative authority. Why it matters: a later reader may let an unamended constitutional sentence or repository snapshot overrule ADR-0002 or an explicit owner decision. **Required disposition:** establish one unambiguous normative precedence across active entry points while preserving Git as evidence of actual state, not a policy override.

3. **ARC01-S03 — MAJOR — `docs/planning/zhoor-adoption/DECISIONS.md:7,30`, `jobs/ARC-01.md:35`, `README.md:54-55`, `WORK_BOARD.md:59`, `handoffs/NEXT_ACTIONS.md:3,61,143`; historical source `docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md:13,28`.** Conflicting authority: owner-verified in-place GitHub rename and published `origin/integration/zhoor-adoption`; root README and constitution already name ZHOOR-AIOS. Evidence: accepted D-001/open rename gate and the ARC-01 checklist still call rename deferred; the adoption README, board and live handoff say the integration tranche is not pushed. ADR-0001's original deferral is historical context, not proof of current remote state. Why it matters: agents can target the wrong repository or hold a completed identity gate open and reviewers are instructed to certify a false state. **Required disposition:** record the later owner decision/current remote state in active governance and retire the outdated check/gate without silently rewriting ADR provenance.

4. **ARC01-S04 — MAJOR — `README.md:10-14`, `docs/planning/zhoor-adoption/README.md:54,56,59`, `ROADMAP.md:45`, `docs/constitution/HERMES_CONSTITUTION.md` §11.** Conflicting authority: accepted ADR-0002 and the candidate Git tree. Evidence: tracked `crates/system-info/Cargo.toml`, Rust library/CLI and `.github/workflows/system-info.yml` exist, yet the active entry points claim no implementation, manifest, tests, CI or product behavior. ADR-0002 explicitly allows the completed rootless read-only `system.info.read` exception before the decision-worker baseline; the constitution's original sequence and the roadmap's “no product behavior” foundation gate do not mention it. Why it matters: resumption, baseline checks, and new scaffolding could inadvertently treat the existing package as unauthorized or overwrite/duplicate it. **Required disposition:** reconcile current-state and phase-gate language explicitly with ADR-0002's *single* bounded exception; do not broaden that exception or advance `aiosd`/canary gates.

5. **ARC01-S05 — MAJOR — `docs/planning/zhoor-adoption/ORCHESTRATOR_RUNBOOK.md:7-27`, `WORK_BOARD.md:13-41`, `MODEL_AND_SKILL_MATRIX.md:5,24-25`, `DECISIONS.md:16`, `jobs/ARC-01.md:5-6`, `handoffs/NEXT_ACTIONS.md:28-30,171-184`.** Conflicting authority: observed lanes are Pi / GPT-6 Sol / high and Codex / GPT-6 Luna / xhigh; GPT-5.6 provenance in D-011 is historical. Evidence: current lane/assignment/acceptance controls still require Codex / GPT-5.6 Sol and Codex / GPT-5.6 Luna, with availability and fallback rules tied to those identities; the actual primary reviewer is Pi / GPT-6 Sol. Why it matters: a valid independent review may be rejected as an unavailable historical lane, or a substitution can be triggered despite the present Sol lane; security/architecture acceptance and reviewer identity become ambiguous. **Required disposition:** owner records observed lanes and ratifies current role/independence mapping across runbook, board, cards and assignment prompts; keep D-011 as dated authorship evidence and do not conflate a model name or harness with authority.

6. **ARC01-S06 — MAJOR — `docs/planning/zhoor-adoption/DECISIONS.md:17,31`, `ORCHESTRATOR_RUNBOOK.md:11,24-29,33`, `jobs/ARC-01.md:6,26-29`, `WORK_BOARD.md:13,41`.** Conflicting authority: the architecture/security independent-review guarantee and the recorded author-conflict rule. Evidence: the handoff/lane layer under review was Qwen3.8 Max-authored, yet an owner waiver can allow that same model to act as the ARC-01 *independent challenger*. The default non-Qwen challenger is sound, but the waiver path would label partial self-review independent; the runbook's generic Qwen interim-primary substitution is also unsafe for this specific card unless its author exception is enforced. Why it matters: the two-report gate can be satisfied on paper without an independent challenge of the full artefact. **Required disposition:** require at least one full-scope, non-author independent challenger for ARC-01, including any substitution; an author review may be supplemental but cannot discharge independence by waiver.

7. **ARC01-S07 — MAJOR — `docs/planning/zhoor-adoption/ROADMAP.md` §Global release gates, `jobs/ARC-02.md:6,21-27`, `jobs/ARC-03.md:3,15`, `ORCHESTRATOR_RUNBOOK.md:35-36`, `WORK_BOARD.md:14-15`.** Conflicting authority: dependent cards stay blocked until targeted checks and independent review handoffs exist, but ARC-02's review is assigned to Sol *inside ARC-03*, which cannot start until ARC-02 completes. Evidence: ARC-02 provides an implementer handoff only; wave 1 says evidence complete with no decision and wave 2 performs both review and adjudication. Why it matters: ARC-02 cannot be marked DONE under the global review gate without beginning its dependent card early, or the gate must be silently waived. **Required disposition:** specify a separate review/handshake before ARC-02 closure or an explicit bounded evidence-only transition that keeps the independent review requirement effective before ARC-03 accepts the toolchain decision.

## Invariants checked

- [x] Architecture and ADR-0002 keep model output/confidence non-authoritative; the inspected `system.info.read` implementation uses typed input, a fixed capability, deterministic policy, a root/unknown-UID denial, verification and a compact event, with no command/path argument or shell path. This is **not** `aiosd` and is not a grant or promotion precedent.
- [x] Normative architecture separates runtime inference, deterministic Policy Engine grants, `aiosd` allowlisted execution/receipts, and canonical `my-ai-brain` memory; OB1 is optional evidence/mirror. Skill routing remains metadata-first; no hard Needle/Jev/model/provider dependency was found in the inspected contract.
- [x] Board job IDs match job filenames and runbook IDs (25 each). The visible roadmap dependency graph and board agree as to *declared edges*, apart from the operational ARC-02/03 preconditions noted above. Later cards remain BLOCKED and sequenced through evaluation, grants, `aiosd`, promotion audit and canary proposal.
- [ ] Governance does not yet unambiguously express current authority, identity, lanes, state, and independent-review/ARC-02 gates (S01–S07).

## Validation independently run

| Check | Result |
|---|---|
| `pwd; git rev-parse HEAD; git branch --show-current; git status --short --branch` | Assigned path, exact SHA, detached HEAD; clean before report creation. |
| `git remote -v; git branch -r --contains HEAD` | `origin` points to `Tariq-Anjum/ZHOOR-AIOS`; local remote-tracking `origin/integration/zhoor-adoption` contains candidate. Remote publication additionally owner-verified; no network call made. |
| `git ls-files` targeted inventory | Tracked `system-info` Rust crate and CI workflow present. |
| Read-only job-ID set comparison (Python) | 25 cards = 25 board rows = 25 runbook IDs. |
| `git diff --check; git diff --cached --check` | Passed on clean committed checkout, before report. |
| Roadmap/board edge comparison | Board lists 29 prerequisite edges; script found 28 due to first inline `A[...] --> B[...]` declaration; that missing edge is visible at ROADMAP line 9. No declared dependency disagreement found by inspection. No third parser attempt made. |

No product tests/build were run: this is a governance review, not a runtime audit. No whole-workspace link sweep was run because that would risk accessing excluded review reports.

## Residual risk

Owner must adjudicate findings against the independent challenger, correct active contracts, then repeat ARC-01 review before moving the board to ARC-02. This review did not establish live Herdr authentication/provider state, remotely fetch refs, audit the entire runtime implementation, or test future security cards; no such facts are inferred from the documents.
