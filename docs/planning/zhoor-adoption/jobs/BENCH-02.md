# BENCH-02 — Approval-gated measured backend run

**Depends on:** BENCH-01  
**Recommended:** DeepSeek V4.1 Flash  
**Review:** GPT-5.6 Sol  
**Timebox:** one measured-run and report session

## Objective

Run the approved backend benchmark on the versioned datasets and target hardware, record reproducible measurements, and recommend reject/continue—not promotion.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read BENCH-01 runner/handoff, AUD-01, datasets, and approved package/network conditions.
- Karpathy application: run the frozen matrix; do not rewrite adapter/harness or add candidates mid-run.

## Write scope

- Versioned benchmark configuration, raw sanitized measurements, summarized report, and `docs/planning/zhoor-adoption/handoffs/BENCH-02.md`.
- ARC-03 must replace this description with exact path globs before `READY`.

## Acceptance

- Deterministic/index and semantic candidate run the same frozen cases.
- Report accuracy, abstention, semantic/schema failures, calibration, peak memory, cold/warm load, queue/inference latency, throughput, cost/privacy/egress, timeout, and unavailable-backend behavior.
- Environment, model/package versions, hardware, telemetry state, and failed cases are recorded.
- Recommendation is evidence-based and cannot authorize execution or silently change routing.

## Stop

After two measured-run failures, stop and report them; do not churn versions, install alternatives, or discard unfavorable results.
