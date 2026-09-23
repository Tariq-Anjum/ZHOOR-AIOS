# <JOB-ID> review by <REVIEWER>

## Verdict

`approve | approve-with-nonblocking-notes | changes-required | blocked`

## Review ownership

- Output path: `docs/planning/zhoor-adoption/handoffs/reviews/<JOB-ID>-<REVIEWER>.md`
- Implementation and implementation handoff edited: `no`

## Scope reviewed

- Commit/diff/files and acceptance criteria.

## Blocking findings

1. `[severity] path:line` — evidence, impact, required correction.

## Non-blocking findings

1. `[severity] path:line` — evidence and recommendation.

## Invariants checked

- [ ] No model output authorizes execution.
- [ ] Deterministic/index path remains independently usable.
- [ ] Unknown/invalid/over-budget input fails closed or abstains explicitly.
- [ ] Provenance and trace fields are preserved without secrets/reasoning transcripts.
- [ ] No undeclared dependency, network, telemetry, or write-scope expansion.
- [ ] Relevant tests and fixtures cover negative behavior.

## Validation independently run

| Command/check | Result |
|---|---|
| | |

## Residual risk

- Risk, owner, and required gate.
