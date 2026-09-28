# CANARY-01 — Reversible capability canary proposal


**Depends on:** AUD-02 explicit permission  
**Recommended:** GPT-5.6 Sol  
**Acceptance:** human owner through a new ADR  
**Timebox:** one planning session; no implementation

## Objective

Propose exactly one low-risk, reversible capability canary with typed arguments, policy/evidence requirements, grant scope, independent verification, rollback, metrics, and kill switch.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read AUD-02, aiosd contracts, promotion gates, and only capability-specific evidence needed for the proposal.
- Karpathy application: one minimal capability; state uncertainty; do not treat schedule or confidence as evidence.

## Write scope

- `docs/architecture/adr/ADR-CANARY-01-*.md` — one proposal ADR only.
- `docs/evaluations/canary-01/**` — capability-specific threat, rollback, and evaluation proposal material.
- `docs/planning/zhoor-adoption/handoffs/CANARY-01.md`.
- No runtime implementation, grant issuance, or capability activation.


## Path ownership and validation

Targeted: `git diff --check -- docs/architecture/adr/ADR-CANARY-01-*.md docs/evaluations/canary-01/ docs/planning/zhoor-adoption/handoffs/CANARY-01.md`.

Full: `./scripts/validate.sh`.

## Required proposal fields

- Capability ID and exact schema.
- Why it is low-risk and reversible.
- Preconditions, trust tier, independent evidence, human approval, and denial rules.
- Grant lifetime, identity, resource scope, normalized argument binding, and receipt fields.
- Verification oracle and rollback procedure.
- Offline/replay/shadow/canary metrics and explicit thresholds.
- Kill switch, incident response, and data-retention rules.
- Alternatives and reason to defer higher-risk capabilities.

## Success condition

A reviewable ADR exists; it grants no authority until the human owner accepts it. Shell, package install, service control, arbitrary filesystem mutation, network egress, or privilege escalation are ineligible as the first canary.

## Stop

The global two-attempt rule applies. If no capability meets all requirements, recommend no canary. Do not weaken thresholds or expand to multiple capabilities.
