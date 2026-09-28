# Next actions — ARC-01 closure and next-card handoff

Branch: `integration/zhoor-adoption` · canonical repository: `Tariq-Anjum/ZHOOR-AIOS` · status authority: [`../WORK_BOARD.md`](../WORK_BOARD.md)

## Current state

- The ARC-01 baseline candidate was `6a73756e8694d7d70e79cb3c8d037b54a79c5a32`; official reports are `reviews/ARC-01-sol.md` and `reviews/ARC-01-luna.md`.
- The owner accepted fixes O-ARC01-01 through O-ARC01-10, including the bounded ADR-0002 `system.info.read` status correction. The reports are evidence; the compact adjudication record is in `DECISIONS.md`.
- The focused independent exact-SHA audit passed at `40595b72ec041e7cd17f3a6306ef78eff43e6a92`. ARC-01 is `DONE`; ARC-02 is the only `READY` card. ARC-03 and all later cards remain blocked.
- Next: ARC-02 — language and toolchain evidence spike. Start it in a fresh session; this closure session does not start ARC-02.

## Closure validation and audit

1. The original 39 protected paths reconciled to 11 owner-waived governance overlaps and 28 remaining protected paths; all 28 have status/path/SHA-256 baseline records in orchestrator `STATE.md`.
2. The owner accepted the disclosed lack of pre-closure hashes for the 11 waived paths.
3. The focused audit passed at `40595b72ec041e7cd17f3a6306ef78eff43e6a92`; the status transition follows in this commit.
4. After an independent read-only post-transition confirmation, fetch and verify the remote. A fast-forward permits only the normal push of `integration/zhoor-adoption`; divergence means stop.
5. Verify local and remote SHAs match, then update orchestrator `STATE.md` with final closure and publication state.

## ARC-02 handoff after closure

ARC-02 is an evidence-only comparison of minimal Rust, minimal Python, and a bounded mixed option. Use committed evidence/reproducibility files under `docs/planning/zhoor-adoption/evidence/ARC-02/` and disposable build directories under `/tmp/zhoor-arc02/<RUN_ID>/`. Do not make a production language/layout decision or create the production scaffold. The existing `crates/system-info` ADR-0002 exception may be evidence but does not predetermine the toolchain.

ARC-02 requires its own independent review before it becomes `DONE`. ARC-03 starts only afterward and owns toolchain adjudication, production layout, canonical commands, and downstream path globs. Do not use ARC-03 as ARC-02's reviewer.
