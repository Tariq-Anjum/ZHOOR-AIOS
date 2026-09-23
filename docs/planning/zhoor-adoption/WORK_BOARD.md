# ZHOOR adoption work board

The owner/orchestrator is the only editor of this file. This is the sole status authority; job cards intentionally contain no status field. Implementation agents accept assigned `READY` cards; review agents accept assigned `REVIEW` cards.

Status values: `READY`, `ASSIGNED`, `BLOCKED`, `REVIEW`, `DONE`, `DEFERRED`.

Before any downstream implementation card becomes `READY`, `ARC-03` must replace descriptive write areas with concrete repository-relative path globs and exact targeted/full validation commands. Parallel cards must be mechanically disjoint.

## Board

| ID | Status | Depends on | Deliverable | Recommended implementer | Independent review | Planned exclusive area |
|---|---|---|---|---|---|---|
| ARC-01 | REVIEW | none | Governance baseline review | GPT-5.6 Sol | Qwen3.8 Max + owner | review handoff only |
| ARC-02 | BLOCKED | ARC-01 | Reproducible Rust/Python/bounded-mixed evidence spike | Qwen3.8 Max | GPT-5.6 Sol in ARC-03 | isolated spike evidence |
| ARC-03 | BLOCKED | ARC-02 | Toolchain ADR, canonical commands, pinned paths | GPT-5.6 Sol | Qwen3.8 Max + owner | ADR/toolchain docs/board/card path sections |
| BASE-01 | BLOCKED | ARC-03 | Minimal verified project scaffold | GLM 5.3 Flash | GPT-5.6 Luna | root manifests/config, smoke source/test, CI |
| PRO-01 | BLOCKED | BASE-01 | Versioned decision protocol and fixtures | GLM 5.3 Flash | GPT-5.6 Luna | protocol schemas/package/tests |
| REG-01 | BLOCKED | PRO-01 | Compact descriptor registry/index loader | GLM 5.3 Flash | Qwen3.8 Flash | registry package/fixtures/tests |
| POL-01 | BLOCKED | PRO-01 | Deterministic risk rules and recommendations | GPT-5.6 Luna | Qwen3.8 Max | risk-policy package/fixtures/tests |
| TRC-01 | BLOCKED | PRO-01 | Sanitized trace writer and replay | Muse Spark 1.3 | DeepSeek V4.1 Flash | trace package/fixtures/tests |
| SEC-01 | BLOCKED | PRO-01 | `aiosd`/grant contracts and hostile fixtures | Qwen3.8 Max | GPT-5.6 Sol | aiosd schemas/fixtures/contract tests |
| WRK-01 | BLOCKED | REG-01, TRC-01 | Deterministic `skill_router` | Muse Spark 1.3 | GPT-5.6 Luna | skill-router worker/tests |
| WRK-02 | BLOCKED | POL-01, TRC-01 | Deterministic non-authoritative `risk_judge` | GLM 5.3 Flash | Qwen3.8 Max | risk-judge worker/tests |
| EVAL-00 | BLOCKED | WRK-01, WRK-02 | Shared deterministic evaluation harness | DeepSeek V4.1 Flash | GPT-5.6 Luna | eval harness/report contract/tests |
| EVAL-01 | BLOCKED | EVAL-00 | First 25–50 skill-router cases | GPT-5.6 Luna | DeepSeek V4.1 Flash + human spot-check | skill dataset batch 1 |
| EVAL-02 | BLOCKED | EVAL-00 | First 25–50 risk cases | Qwen3.8 Flash | GPT-5.6 Luna + human spot-check | risk dataset batch 1 |
| EVAL-03 | BLOCKED | EVAL-01 | Complete 50–100 skill cases and aggregate report | DeepSeek V4.1 Flash | Qwen3.8 Max + human spot-check | skill dataset batch 2/report |
| EVAL-04 | BLOCKED | EVAL-02 | Complete 50–100 risk cases and aggregate report | GPT-5.6 Luna | DeepSeek V4.1 Flash + human spot-check | risk dataset batch 2/report |
| AUD-01 | BLOCKED | EVAL-03, EVAL-04 | Deterministic slice architecture/security audit | GPT-5.6 Sol | Human owner | review handoff only |
| BENCH-01 | BLOCKED | AUD-01 | Backend adapter, mocks, frozen benchmark runner | Muse Spark 1.3 | GPT-5.6 Sol | benchmark adapter/runner/tests |
| BENCH-02 | BLOCKED | BENCH-01 | Approval-gated measured run and report | DeepSeek V4.1 Flash | GPT-5.6 Sol | benchmark configs/results/report |
| SHADOW-01 | BLOCKED | BENCH-02 | No-effect shadow integration/report | DeepSeek V4.1 Flash | GPT-5.6 Luna | shadow adapter/tests/report |
| GRANT-01 | BLOCKED | SEC-01, SHADOW-01 | Policy-owned grant issuer/verifier for `capabilities.describe` | Qwen3.8 Max | GPT-5.6 Sol | policy grant package/tests/docs |
| AIOSD-01 | BLOCKED | GRANT-01 | Enforcement core and immutable append-only receipts | Muse Spark 1.3 | GPT-5.6 Sol | aiosd core/receipt tests |
| AIOSD-02 | BLOCKED | AIOSD-01 | Unix transport, peer identity, lifecycle hardening | GLM 5.3 Flash | GPT-5.6 Sol | aiosd transport/integration tests |
| AUD-02 | BLOCKED | AIOSD-02 | Promotion and threat-model audit | GPT-5.6 Sol | Human owner | review handoff only |
| CANARY-01 | BLOCKED | AUD-02 | One reversible canary proposal | GPT-5.6 Sol | Human owner | proposal ADR only |

## Assignment rules

- Architecture/security acceptance stays with GPT-5.6 Sol plus human review.
- Prefer a reviewer from a different model family; do not use model consensus as acceptance.
- Qwen3.8 Max is the challenger for protocol/security reasoning.
- GLM 5.3 Flash and Muse Spark are default implementation agents.
- Luna, Qwen3.8 Flash, and DeepSeek V4.1 Flash handle bounded fixtures, datasets, measurements, and routine review cost-effectively.
- Tests, deterministic oracles, provenance, and human decisions control promotion.

## Parallelism

- After `PRO-01`, `REG-01`, `POL-01`, `TRC-01`, and `SEC-01` may use separate worktrees only after ARC-03 has pinned disjoint paths.
- `WRK-01` and `WRK-02` may run in parallel after dependencies merge.
- `EVAL-01` and `EVAL-02` may run in parallel; then `EVAL-03` and `EVAL-04` may run in parallel.
- From `AUD-01` onward, serialize except where a later accepted ADR explicitly changes the security order.

## Current owner action

Review this tranche, establish a named integration branch, commit it if accepted, assign `ARC-01` review, then unblock only one wave at a time.
