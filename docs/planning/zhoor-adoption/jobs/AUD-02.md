# AUD-02 — Promotion and threat-model audit


**Depends on:** AIOSD-02  
**Recommended:** GPT-5.6 Sol  
**Acceptance:** human owner  
**Timebox:** one audit session

## Objective

Decide whether evidence supports proposing one reversible capability canary. Audit shadow results and the non-mutating `aiosd` boundary; do not implement or authorize a canary.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read current ADRs, architecture promotion gates, BENCH/SHADOW/AIOSD handoffs, threat fixtures, and relevant tests.
- Karpathy application: make uncertainty and missing evidence explicit; do not manufacture a roadmap success.

## Write scope

- `docs/planning/zhoor-adoption/handoffs/reviews/AUD-02-primary.md`.
- Optional draft recommendation inside that review; no production edits or implementation handoff.

## Required audit

- Shadow accuracy, abstention, calibration, disagreements, and operator-label provenance.
- Deterministic fallback under backend failure/saturation/deadline.
- Peer identity, grant binding/replay, argument canonicalization, allowlisting, receipts, socket lifecycle, and sanitization.
- Resource use and target-machine operation.
- False autonomous action remains zero because no autonomous capability exists.
- Residual threats and human-review requirements.

## Success condition

Return one verdict: no canary; more evidence required; or permit `CANARY-01` to draft a proposal. Only the owner can accept promotion.

## Stop

Do not fix findings or select a canary during audit. Two failed verification attempts produce a blocked verdict with exact evidence.
