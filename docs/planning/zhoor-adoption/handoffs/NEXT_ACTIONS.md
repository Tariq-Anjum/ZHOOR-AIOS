# Next actions — ARC-02 evidence gate and ARC-03 handoff

Branch: `integration/zhoor-adoption` · canonical repository: `Tariq-Anjum/ZHOOR-AIOS` · status authority: [`../WORK_BOARD.md`](../WORK_BOARD.md)

## Current state

- The ARC-01 baseline candidate was `6a73756e8694d7d70e79cb3c8d037b54a79c5a32`; official reports are `reviews/ARC-01-sol.md` and `reviews/ARC-01-luna.md`.
- The owner accepted fixes O-ARC01-01 through O-ARC01-10, including the bounded ADR-0002 `system.info.read` status correction. The reports are evidence; the compact adjudication record is in `DECISIONS.md`.
- The focused ARC-01 independent exact-SHA audit passed at `40595b72ec041e7cd17f3a6306ef78eff43e6a92`; ARC-01 remains `DONE`.
- ARC-02 evidence SHA `e0edd78ed8dd6628a01b346c101a7a618e6d686c` passed exact-SHA independent review after one focused score-range correction. Reports: `reviews/ARC-02-herdr-challenger.md` (initial finding) and `reviews/ARC-02-herdr-review.md` (PASS).
- Current status: ARC-02 `DONE`; ARC-03 `READY`; all later cards remain `BLOCKED`.
- Next: ARC-03 — toolchain adjudication and path pinning. Do not start ARC-03 in the ARC-02 session.

## Closure validation and audit

1. The original 39 protected paths reconciled to 11 owner-waived governance overlaps and 28 remaining protected paths; all 28 have status/path/SHA-256 baseline records in orchestrator `STATE.md`.
2. The owner accepted the disclosed lack of pre-closure hashes for the 11 waived paths.
3. The focused audit passed at `40595b72ec041e7cd17f3a6306ef78eff43e6a92`; the status transition follows in this commit.
4. After an independent read-only post-transition confirmation, fetch and verify the remote. A fast-forward permits only the normal push of `integration/zhoor-adoption`; divergence means stop.
5. Verify local and remote SHAs match, then update orchestrator `STATE.md` with final closure and publication state.

## ARC-02 completion and next-card handoff

ARC-02 completed an evidence-only comparison of minimal Rust, minimal Python, and a bounded mixed option. Evidence SHA `e0edd78ed8dd6628a01b346c101a7a618e6d686c` passed independent exact-SHA review. The committed evidence is under `docs/planning/zhoor-adoption/evidence/ARC-02/`; disposable builds were kept under `/tmp/zhoor-arc02/<RUN_ID>/`. No production language/layout decision or production scaffold was created. The existing `crates/system-info` ADR-0002 exception did not predetermine the toolchain.

ARC-02's independent review gate passed. ARC-03 is now `READY` and owns toolchain adjudication, production layout, canonical commands, and downstream path globs. Do not treat this status handoff as ARC-03 implementation or begin it in the ARC-02 session.
