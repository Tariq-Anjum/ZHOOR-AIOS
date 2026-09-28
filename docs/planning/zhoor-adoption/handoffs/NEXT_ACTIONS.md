# Next actions — ARC-01 closure and next-card handoff

Branch: `integration/zhoor-adoption` · canonical repository: `Tariq-Anjum/ZHOOR-AIOS` · status authority: [`../WORK_BOARD.md`](../WORK_BOARD.md)

## Current state

- The ARC-01 baseline candidate was `6a73756e8694d7d70e79cb3c8d037b54a79c5a32`; official reports are `reviews/ARC-01-sol.md` and `reviews/ARC-01-luna.md`.
- The owner accepted fixes O-ARC01-01 through O-ARC01-10, including the bounded ADR-0002 `system.info.read` status correction. The reports are evidence; the compact adjudication record is in `DECISIONS.md`.
- ARC-01 becomes `DONE` only after this closure commit passes the independent exact-SHA audit. ARC-02 remains blocked until that audit passes; then it is the only card to become `READY`.
- Do not start ARC-02 in the closure session.

## Closure validation and audit

1. Validate the closure-owned patch and reconcile the protected 39-path inventory against orchestrator `STATE.md`.
2. Commit only ARC-01 closure-owned changes; leave pre-existing protected changes unstaged.
3. Discover the independent Pi reviewer through Herdr, complete a role-only handshake, and prepare a detached checkout pinned to the closure SHA.
4. Require the exact-SHA audit to cover all owner dispositions, S04 authority boundaries, active identity/publication state, ARC-02/ARC-03 gates, deterministic slice, SEC-01 verification gate, model-role separation, board status, protected work, and absence of implementation code.
5. If audit passes and protected status remains intact, fetch and verify the remote. A fast-forward permits only the normal push of `integration/zhoor-adoption`; divergence means stop.
6. Verify local and remote SHAs match, then update orchestrator `STATE.md` with the closure result and ARC-02 as the sole next card.

## ARC-02 handoff after closure

ARC-02 is an evidence-only comparison of minimal Rust, minimal Python, and a bounded mixed option. Use committed evidence/reproducibility files under `docs/planning/zhoor-adoption/evidence/ARC-02/` and disposable build directories under `/tmp/zhoor-arc02/<RUN_ID>/`. Do not make a production language/layout decision or create the production scaffold. The existing `crates/system-info` ADR-0002 exception may be evidence but does not predetermine the toolchain.

ARC-02 requires its own independent review before it becomes `DONE`. ARC-03 starts only afterward and owns toolchain adjudication, production layout, canonical commands, and downstream path globs. Do not use ARC-03 as ARC-02's reviewer.
