# SHADOW-01 — Shadow-mode integration


**Depends on:** BENCH-02 reviewed  
**Recommended:** DeepSeek V4.1 Flash  
**Review:** GPT-5.6 Luna  
**Timebox:** one implementation session

## Objective

Add a no-effect shadow mode that records deterministic and candidate-backend recommendations against explicit operator labels/actions without changing routing or executing capabilities.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read architecture sections 6, 9, and 10; BENCH-01 report; trace/eval public interfaces.
- Karpathy application: one shadow adapter and report path only; no live policy changes.

## Write scope

- `crates/shadow/**` — `zhoor-shadow` no-effect adapter/coordinator, tests, and package-local report/sanitization contracts.
- `eval/reports/shadow/**` — sanitized shadow report outputs only.
- `docs/planning/zhoor-adoption/handoffs/SHADOW-01.md`.
- No live policy changes or backend implementation. BENCH paths and production policy paths are read-only.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-shadow` and `cargo test --offline --locked -p zhoor-shadow --test shadow_contract`.

Full: `./scripts/validate.sh`.

## Acceptance

- Shadow output has no effect on returned production decision or any capability.
- Deterministic and candidate routes retain separate traces/timings and common case identity.
- Operator labels are explicit with provenance; absence of a label is not silently treated as correctness.
- Reports disagreements, abstentions, latency/resource impact, and policy-sensitive errors.
- Raw secrets/transcripts are excluded and retention is bounded.
- Feature can be disabled without changing deterministic behavior.

## Stop

Do not auto-promote the backend or tune thresholds from shadow data in this card. After two trace/report integration failures, stop and hand off evidence. Shadow completion is required before GRANT-01 or any `aiosd` implementation.
