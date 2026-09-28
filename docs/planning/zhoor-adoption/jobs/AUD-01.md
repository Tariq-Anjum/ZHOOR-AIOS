# AUD-01 — Deterministic slice audit


**Depends on:** EVAL-03, EVAL-04  
**Recommended:** GPT-5.6 Sol  
**Acceptance:** human owner  
**Timebox:** one audit session

## Objective

Audit the merged protocol, registry, risk rules, traces, workers, and evaluation suites before any inference backend or daemon implementation begins.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read ADRs, architecture invariants, public interfaces, test/evaluation reports, and job handoffs. Inspect implementation by targeted symbol searches.
- Karpathy application: report root causes and blockers; do not refactor during audit.

## Write scope

- `docs/planning/zhoor-adoption/handoffs/reviews/AUD-01-*.md` — one uniquely named audit report only.
- No implementation, fixture, schema, or shared handoff ownership. Validation is read-only.


## Path ownership and validation

Targeted: `git diff --check -- docs/planning/zhoor-adoption/handoffs/reviews/AUD-01-*.md`; the report records relevant candidate tests/checks run and their results.

Full candidate validation responsibility: `./scripts/validate.sh` (read-only; do not edit implementation).

## Required audit

- Dependency direction and write-scope conformance.
- Fail-closed schema/version/budget behavior.
- No model/backend/network/execution dependency in deterministic slice.
- Trace completeness, sanitization, and replay determinism.
- Dataset provenance and absence of model-judged ground truth.
- Accuracy, abstention, false-choice, false-low-risk, latency, and regression evidence.
- Security/adversarial fixture coverage.
- Resource behavior on target hardware where runnable.

## Success condition

Produce an approve/changes-required verdict with evidence and severity-ranked findings. `BENCH-01` remains blocked unless the report explicitly approves backend experimentation.

## Stop

Do not repair findings in this session. After two failed validation attempts, record exact commands/errors and mark the audit blocked.
