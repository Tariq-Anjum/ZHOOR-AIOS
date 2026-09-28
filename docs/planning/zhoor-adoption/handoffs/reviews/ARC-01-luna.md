# ARC-01 review by luna

## Verdict

`changes-required`

**ARC-01 BASELINE NOT YET APPROVED**

Material authority, execution-verification ownership, repository identity, and project-state inconsistencies remain. Do not mark ARC-01 `DONE` until the owner adjudicates these findings and the governing documents are reconciled.

## Review ownership

- Output path: `docs/planning/zhoor-adoption/handoffs/reviews/ARC-01-luna.md`
- Reviewer token: `luna`
- Role: `independent-challenger`
- Model / harness: GPT-6 Luna / Codex, xhigh
- Exact commit reviewed: `6a73756e8694d7d70e79cb3c8d037b54a79c5a32`
- Implementation and implementation handoff edited: `no`

## Scope reviewed

Exact-SHA governance baseline only. Inspected the required root identity documents, constitution, architecture, ADR-0001/0002, adoption README, roadmap, board, decisions, operating rules, model/skill matrix, runbook, ARC-01 card, and review template. Also inspected ARC-02 and ARC-03 cards to verify the dependency sequence. Reviewed the candidate commit’s changed-path summary and relevant identity-migration hunks. Did not inspect the prohibited ARC-01 reports or unrelated historical HERMES documents.

## Blocking findings

1. **[MAJOR] Authority order conflicts with the current owner authority hierarchy** — `docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md:30-40`; `docs/planning/zhoor-adoption/README.md:22-30`.
   - **Conflicting authority:** The current owner-provided order places accepted ADRs and the architecture above the Constitution as amended by accepted ZHOOR decisions. Both documents instead place the current Constitution above accepted ADRs and architecture.
   - **Evidence:** ADR-0001 lists Constitution first, then ADRs, then architecture; the adoption README repeats that order. This leaves the status of accepted amendments such as ADR-0002 unclear against the Constitution’s roadmap.
   - **Why it matters:** Workers cannot reliably resolve conflicts or determine whether a later accepted ADR can amend an earlier constitutional sequence.
   - **Required disposition:** Reconcile both hierarchy statements with the owner’s order and state that the Constitution is interpreted as amended by accepted ZHOOR decisions.

2. **[MAJOR] Execution-result verification has no defined owner in the frozen architecture** — `docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md:38-47, 181-205`; `docs/architecture/adr/ADR-0002-rootless-system-info-slice.md:21-31`.
   - **Conflicting authority:** The required execution pipeline and ADR-0002 require verification to remain distinct from policy, execution, and audit evidence.
   - **Evidence:** The architecture boundary table assigns runtime, policy, and `aiosd` responsibilities but no execution-result verifier. Its sequence ends with `aiosd` returning an execution receipt to policy. ADR-0002 requires “result verification” as a distinct step but does not name its owner or define its contract. The architecture’s `verdict` worker is defined for supported/unsupported/insufficient claims, not post-execution result verification.
   - **Why it matters:** Future implementations could conflate argument validation, successful execution receipts, semantic result verification, and audit evidence, weakening the required separation.
   - **Required disposition:** Assign a deterministic owner and contract for post-execution verification, distinct from policy and `aiosd`, and pin its relationship to the audit receipt before execution-capable work is unblocked.

3. **[MAJOR] Repository rename remains recorded as deferred** — `docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md:17, 28`; `docs/planning/zhoor-adoption/DECISIONS.md:7, 30`; `docs/planning/zhoor-adoption/jobs/ARC-01.md:35`.
   - **Conflicting authority:** Current owner evidence and the candidate’s active entry points state that the former HERMES-AIOS GitHub repository was renamed in place to ZHOOR-AIOS.
   - **Evidence:** `README.md:3`, `.hermes.md:5`, and `docs/constitution/HERMES_CONSTITUTION.md:15` record the rename as complete. ADR-0001 still says the name will change later and lists rename as deferred; D-001 and the open rename gate repeat that obsolete state. ARC-01’s check assumes “Repository rename is deferred.”
   - **Why it matters:** The accepted decision, open gate, and review check contradict the active identity and could trigger unnecessary or misdirected rename work.
   - **Required disposition:** Reconcile ADR-0001, DECISIONS.md, and the ARC-01 check with the completed rename; close the stale rename gate.

4. **[MAJOR] Current project status omits the accepted and tracked `system.info.read` slice** — `README.md:10`; `docs/planning/zhoor-adoption/README.md:54`.
   - **Conflicting authority:** ADR-0002 permits the narrow rootless, read-only `system.info.read` slice, and current owner evidence confirms it is completed.
   - **Evidence:** The adoption README says the repository is documentation-only with no runtime source, package manifest, or CI. The exact-SHA tree tracks `crates/system-info/Cargo.toml`, `crates/system-info/src/lib.rs`, `crates/system-info/src/main.rs`, and `.github/workflows/system-info.yml`. The root README’s unqualified statement that product implementation has not started does not account for this accepted exception.
   - **Why it matters:** The board and entry points misstate current artifacts and can cause workers to overlook or duplicate the authorized slice. This finding does not imply that the decision-runtime or `aiosd` core has been implemented.
   - **Required disposition:** Scope the status claim to the decision-runtime/core implementation and explicitly account for the ADR-0002 slice and its tracked package/workflow.

## Non-blocking findings

1. **[MINOR] Published branch is still described as unpublished** — `docs/planning/zhoor-adoption/README.md:55`; `docs/planning/zhoor-adoption/WORK_BOARD.md:59`.
   - **Evidence:** Both say the governance tranche is “not pushed.” The owner-provided evidence says the candidate is published at `origin/integration/zhoor-adoption`; the local commit decoration also includes that remote-tracking ref.
   - **Why it matters:** This stale state can mislead handoffs about whether publication has occurred.
   - **Required disposition:** Update the workspace snapshot to match the published branch state while retaining the rule that future pushes require owner authorization.

2. **[MINOR] Active lane records use stale GPT-5.6 assignments** — `docs/planning/zhoor-adoption/ORCHESTRATOR_RUNBOOK.md:9-26`; `docs/planning/zhoor-adoption/README.md:57-58`; `docs/planning/zhoor-adoption/WORK_BOARD.md:41`; `docs/planning/zhoor-adoption/DECISIONS.md:16`.
   - **Evidence:** These passages state that GPT-5.6 Sol runs through Codex and recommend GPT-5.6 Luna for the challenger/current lane. Current owner-verified observations are Pi / GPT-6 Sol / high and Codex / GPT-6 Luna / xhigh. D-011’s older GPT-5.6 references are explicitly commit provenance and need not be changed as history.
   - **Why it matters:** Active handshakes and assignments can start from an inaccurate model/runtime map. The present non-Qwen challenger assignment remains independent of the Qwen-authored handoff layer.
   - **Required disposition:** Refresh or clearly date active lane mappings; preserve historical authorship records as historical provenance.

## Invariants checked

- Model output is recommendation/classification only; deterministic policy owns authorization, and `aiosd` owns the planned privileged execution boundary.
- ADR-0002 is explicitly narrow: one rootless, read-only `system.info.read` capability, no paths or commands, no shell, mutation, or network; it does not authorize model execution or capability expansion.
- `my-ai-brain` remains canonical durable memory; OB1 is an optional governed mirror.
- Rules/indexes precede semantic routing; provider/model choices remain replaceable, and Needle/Jev are optional or gated. Herdr is used as orchestration/observability transport, not policy authority.
- Roadmap and board agree that ARC-01 gates ARC-02, ARC-02 gates ARC-03, and downstream implementation waits for ARC-03 path/check pinning. No later card bypasses those gates.
- After ARC-01 findings are corrected and the owner adjudicates the reports, ARC-02 is the next bounded unit: an evidence spike with no decision or product implementation. ARC-03 adjudicates the evidence before scaffolding.
- The ARC-01 review assignment retains an independent non-Qwen challenge and owner adjudication. Architecture/security review gates remain explicit.
- **Not verified here:** runtime behavior, system-info tests, or implementation-level negative cases. This review is limited to the governance baseline.

## Validation independently run

| Command/check | Result |
|---|---|
| `git rev-parse HEAD` | Matched `6a73756e8694d7d70e79cb3c8d037b54a79c5a32`. |
| `git status --short --branch` | Initially clean, detached checkout at the requested SHA. |
| `git show --stat --oneline HEAD` and `git diff-tree --name-status -r HEAD` | Confirmed candidate commit and changed paths. |
| `git ls-files` | Confirmed tracked `system-info` package sources/manifest and workflow. |
| Targeted `rg -n` over the required governance documents | Located stale rename, publication, and lane statements. |
| Tests/builds | Not run; runtime implementation behavior was outside this governance review scope. |

## Residual risk

This review does not re-test the `system.info.read` implementation or independently discover live Herdr workers. Current runtime identities and publication status are based on the owner-verified evidence in the assignment; the candidate checkout corroborates the published remote-tracking ref. Historical HERMES material was not consulted because it is non-normative and outside the requested scope.
