# ARC-03 independent exact-SHA review — Luna

RUN_TASK: `ZHOOR-ARC03-LUNA-CHALLENGE-01`

ROLE / HARNESS / OBSERVED MODEL / REASONING / SESSION: Independent challenger #1; Pi; OpenAI Codex / GPT-6 Luna / xhigh per controller-reported live-pane handshake; fresh session, no ARC-03 authorship; read-only `read` tooling.

REVIEWED_SHA: `1f669d84765e36a64278ac73ffbef77852ee97c1`

AUDIT_RESULT: `CHANGES_REQUIRED`

## Findings

1. **P1 — Ambiguous trace-schema ownership.** `PRO-01` assigned canonical trace wire schemas to `schemas/protocol/**`, while `TRC-01` also owned trace schemas at `schemas/trace/**`. The roots did not literally overlap, but the cards both claimed trace-contract ownership. Correction: reserve request/response/evidence schemas for `PRO-01`; make `TRC-01` the sole owner of the trace schema.
2. **P2 — Target scaffold conflicted with the selected layout.** `TARGET_SCAFFOLD.md` showed component code under root `src/` and attributed the shape to ARC-02. ADR-0003 and the toolchain document specify packages under `crates/`, while ARC-02 evidence did not choose a layout. Correction: reconcile the illustrative scaffold and correct the ARC-02 attribution.
3. **P2 — Missing later packages.** ADR-0003 omitted `zhoor-backend-adapter` and `zhoor-shadow`, although BENCH-01 and SHADOW-01 require those packages. Correction: add both as later-gated packages.
4. **P3 — Overbroad CANARY-01 validation glob.** The reviewer cited the card's ADR glob as `ADR-*.md`; the card itself was already narrowed in the candidate, while the canonical toolchain table still used the broad glob. Correction: narrow the toolchain table to `ADR-CANARY-01-*.md` and include only its proposal paths.
5. **P3 — Handoff SHA marker.** The immutable candidate commit contained a pre-freeze handoff marker. The controller updated the authoritative handoff in its main worktree after freezing the exact SHA. A commit cannot include its own content-addressed SHA; the reviewed SHA is identified in the packet/report and the post-freeze controller handoff.

## Path matrix assessment

By inspection, listed implementation, fixture, report, and handoff roots for REG-01/POL-01/TRC-01/SEC-01, WRK-01/WRK-02, EVAL-01/EVAL-02, EVAL-03/EVAL-04, and GRANT-01/AIOSD-01/AIOSD-02 were disjoint. The work board kept downstream cards blocked. No generated-binding scheme/output path had been selected, so no current generated-output collision was found.

## Recovery simulation

On a corrected, newly frozen SHA, inspect the base-to-candidate changed-path list and recheck schema ownership, scaffold/layout agreement, package inventory, and CANARY-01 command. Recheck literal path overlaps and confirm downstream cards remain blocked.

## Residual risk

ARC-02 supports Rust-first and omitting Python from the production request path only as a bounded local recommendation, not an SLO or capacity guarantee. The workload was small and measurements noisy. The actual workspace, schema-to-Rust conformance mechanism, and validation script remain future implementation work.

## Validation

Inspected the ARC-03 card and handoff; AGENTS.md; ADR-0001/0002/0003; normative architecture; target scaffold; toolchain document; work board and runbook; ARC-02 evidence/handoff; and all listed downstream cards. Future Cargo/Python validation, tests, and builds were unrun because BASE-01 owns the workspace and validation script. The reviewer did not run `git diff`, `git diff --check`, or the ownership audit because only read tooling was exposed.
