# TRC-01 — Sanitized trace and replay core


**Depends on:** PRO-01  
**Recommended:** Muse Spark 1.3  
**Review:** DeepSeek V4.1 Flash  
**Timebox:** one implementation session

## Objective

Implement deterministic trace creation, serialization, sanitization, and replay input reconstruction for protocol-level decisions.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Load `file-based-planning` only if storage trade-offs exceed this session.
- Read ZHOOR architecture sections 6, 10, and 11; protocol interfaces; ARC-02 toolchain docs.
- Karpathy application: prefer an append/test fixture implementation; do not choose a broad database platform.

## Write scope

- Trace/replay package/module.
- Trace sanitization policy/config.
- Trace fixtures/tests.
- Trace-specific docs and `docs/planning/zhoor-adoption/handoffs/TRC-01.md`.
- ARC-03 must replace this description with exact path globs and validation commands before `READY`.

## Acceptance

- Records protocol/worker/model-or-deterministic/policy/registry versions, normalized hashes, route, confidence, evidence refs, timings, disposition, and error/receipt refs where applicable.
- Never records secrets, unrestricted transcripts, hidden reasoning, or raw evidence bodies by default.
- Replay reconstructs normalized decision inputs and detects hash/version mismatch.
- Identical normalized input produces stable hashes; each invocation retains unique trace/request identity.
- Queue/load/inference timings remain distinct.
- Targeted and full checks pass.

## Stop

The global two-attempt rule applies. Do not introduce canonical memory or a vector database. If durable storage needs an ADR, implement only the in-memory/file fixture boundary and hand off the decision.
