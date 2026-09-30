# ZHOOR adoption work board

The owner/orchestrator is the only editor of this file. This is the sole status authority; job cards intentionally contain no status field. Implementation agents accept assigned `READY` cards; review agents accept assigned `REVIEW` cards.

Status values: `READY`, `ASSIGNED`, `BLOCKED`, `REVIEW`, `DONE`, `DEFERRED`.

Before any downstream implementation card becomes `READY`, `ARC-03` must replace descriptive write areas with concrete repository-relative path globs and exact targeted/full validation commands. Parallel cards must be mechanically disjoint.

## Board

| ID | Status | Depends on | Deliverable | Recommended runtime role | Independent review role | Planned exclusive area |
|---|---|---|---|---|---|---|
| ARC-01 | DONE | none | Governance baseline review and owner-adjudicated closure | architecture-primary | independent-challenger + owner | review handoff only |
| ARC-02 | DONE | ARC-01 closure audit PASS | Reproducible Rust/Python/bounded-mixed evidence spike; evidence only | implementation-low-cost | independent-challenger before DONE | `docs/planning/zhoor-adoption/evidence/ARC-02/`; disposable builds under `/tmp/zhoor-arc02/<RUN_ID>/` |
| ARC-03 | DONE | ARC-02 DONE + independent review | Toolchain ADR, canonical commands, pinned paths | architecture-primary | independent-challenger + owner | `docs/architecture/adr/ADR-0003-*.md`; `docs/development/ZHOOR_TOOLCHAIN.md`; `docs/planning/zhoor-adoption/WORK_BOARD.md`; downstream path/validation sections; `docs/planning/zhoor-adoption/handoffs/ARC-03.md` |
| BASE-01 | DONE | ARC-03 | Minimal verified project scaffold | implementation-low-cost | independent-challenger | `Cargo.toml`; `Cargo.lock`; `rust-toolchain.toml`; `.cargo/**`; `.github/workflows/zhoor.yml`; `scripts/validate.sh`; `src/**`; `tests/smoke.rs`; `docs/planning/zhoor-adoption/handoffs/BASE-01.md` |
| PRO-01 | DONE | BASE-01 | Versioned decision protocol and fixtures | implementation-low-cost | security-reviewer | `schemas/protocol/**`; `crates/protocol/**`; `fixtures/protocol/**`; `docs/planning/zhoor-adoption/handoffs/PRO-01.md` |
| PRO-02 | ASSIGNED | PRO-01 | Registry descriptor identity/version contract | implementation-low-cost | independent-architecture-reviewer | `schemas/protocol/**`; `crates/protocol/**`; `fixtures/protocol/**`; `docs/planning/zhoor-adoption/handoffs/PRO-02.md` |
| REG-01 | BLOCKED | PRO-01, PRO-02 | Compact descriptor registry/index loader | implementation-low-cost | evaluation-reviewer | `schemas/registry/**`; `crates/registry/**`; `fixtures/registry/**`; `docs/planning/zhoor-adoption/handoffs/REG-01.md` |
| POL-01 | BLOCKED | PRO-01 | Deterministic risk rules and recommendations | implementation-low-cost | security-reviewer | `schemas/risk-policy/**`; `crates/risk-policy/**`; `fixtures/risk-policy/**`; `docs/planning/zhoor-adoption/handoffs/POL-01.md` |
| TRC-01 | DONE | PRO-01 | Sanitized trace writer and replay | implementation-low-cost | evaluation-reviewer | `schemas/trace/**`; `crates/trace/**`; `fixtures/trace/**`; `docs/planning/zhoor-adoption/handoffs/TRC-01.md` |
| SEC-01 | BLOCKED | PRO-01 | `aiosd`/grant contracts and hostile fixtures | security-reviewer | independent-challenger | `schemas/aiosd/**`; `crates/aiosd-contracts/**`; `fixtures/aiosd/**`; `docs/contracts/aiosd/**`; `docs/planning/zhoor-adoption/handoffs/SEC-01.md` |
| WRK-01 | BLOCKED | REG-01, TRC-01 | Deterministic `skill_router` | implementation-low-cost | evaluation-reviewer | `crates/skill-router/**`; `fixtures/skill-router/**`; `docs/planning/zhoor-adoption/handoffs/WRK-01.md` |
| WRK-02 | BLOCKED | POL-01, TRC-01 | Deterministic non-authoritative `risk_judge` | implementation-low-cost | evaluation-reviewer | `crates/risk-judge/**`; `fixtures/risk-judge/**`; `docs/planning/zhoor-adoption/handoffs/WRK-02.md` |
| EVAL-00 | BLOCKED | WRK-01, WRK-02 | Shared deterministic evaluation harness | evaluation-reviewer | independent-challenger | `tools/eval/harness/**`; `tools/eval/tests/harness/**`; `docs/evaluations/harness/**`; `docs/planning/zhoor-adoption/handoffs/EVAL-00.md` |
| EVAL-01 | BLOCKED | EVAL-00 | First 25–50 skill-router cases | evaluation-reviewer | independent-challenger + human spot-check | `eval/datasets/skill-router/batch-01/**`; `tools/eval/tests/skill_router_batch_01/**`; `docs/planning/zhoor-adoption/handoffs/EVAL-01.md` |
| EVAL-02 | BLOCKED | EVAL-00 | First 25–50 risk cases | evaluation-reviewer | independent-challenger + human spot-check | `eval/datasets/risk-judge/batch-01/**`; `tools/eval/tests/risk_judge_batch_01/**`; `docs/planning/zhoor-adoption/handoffs/EVAL-02.md` |
| EVAL-03 | BLOCKED | EVAL-01 | Complete 50–100 skill cases and aggregate report | evaluation-reviewer | independent-challenger + human spot-check | `eval/datasets/skill-router/batch-02/**`; `eval/reports/skill-router/**`; `tools/eval/tests/skill_router_batch_02/**`; `docs/planning/zhoor-adoption/handoffs/EVAL-03.md` |
| EVAL-04 | BLOCKED | EVAL-02 | Complete 50–100 risk cases and aggregate report | evaluation-reviewer | independent-challenger + human spot-check | `eval/datasets/risk-judge/batch-02/**`; `eval/reports/risk-judge/**`; `tools/eval/tests/risk_judge_batch_02/**`; `docs/planning/zhoor-adoption/handoffs/EVAL-04.md` |
| AUD-01 | BLOCKED | EVAL-03, EVAL-04 | Deterministic slice architecture/security audit | security-reviewer | human owner | `docs/planning/zhoor-adoption/handoffs/reviews/AUD-01-*.md` only |
| BENCH-01 | BLOCKED | AUD-01 | Backend adapter, mocks, frozen benchmark runner | implementation-low-cost | evaluation-reviewer | `crates/backend-adapter/**`; `docs/planning/zhoor-adoption/handoffs/BENCH-01.md` |
| BENCH-02 | BLOCKED | BENCH-01 | Approval-gated measured run and report | evaluation-reviewer | independent-challenger | `eval/benchmarks/configs/**`; `eval/benchmarks/results/**`; `eval/reports/benchmarks/**`; `docs/planning/zhoor-adoption/handoffs/BENCH-02.md` |
| SHADOW-01 | BLOCKED | BENCH-02 | No-effect shadow integration/report | implementation-low-cost | evaluation-reviewer | `crates/shadow/**`; `eval/reports/shadow/**`; `docs/planning/zhoor-adoption/handoffs/SHADOW-01.md` |
| GRANT-01 | BLOCKED | SEC-01, SHADOW-01 | Policy-owned grant issuer/verifier for `capabilities.describe` | security-reviewer | independent-challenger | `crates/grant/**`; `fixtures/grant/**`; `docs/contracts/grant/**`; `docs/planning/zhoor-adoption/handoffs/GRANT-01.md` |
| AIOSD-01 | BLOCKED | GRANT-01 | Enforcement core and immutable append-only receipts | security-reviewer | independent-challenger | `crates/aiosd-core/**`; `fixtures/aiosd-core/**`; `docs/contracts/aiosd-core/**`; `docs/planning/zhoor-adoption/handoffs/AIOSD-01.md` |
| AIOSD-02 | BLOCKED | AIOSD-01 | Unix transport, peer identity, lifecycle hardening | security-reviewer | independent-challenger | `crates/aiosd-unix/**`; `docs/contracts/aiosd-transport/**`; `docs/planning/zhoor-adoption/handoffs/AIOSD-02.md` |
| AUD-02 | BLOCKED | AIOSD-02 | Promotion and threat-model audit | security-reviewer | human owner | `docs/planning/zhoor-adoption/handoffs/reviews/AUD-02-*.md` only |
| CANARY-01 | BLOCKED | AUD-02 | One reversible canary proposal | architecture-primary | human owner | `docs/architecture/adr/ADR-CANARY-01-*.md`; `docs/evaluations/canary-01/**`; `docs/planning/zhoor-adoption/handoffs/CANARY-01.md` |

## Assignment rules

- Architecture/security acceptance uses independent evidence and owner review. The concrete current reviewer lanes are in `MODEL_AND_SKILL_MATRIX.md`.
- Independence requires a different reviewer and session with no authorship conflict; model/provider family is not an authority requirement.
- Review files are `<JOB-ID>-<reviewer-token>.md`; use `herdr-primary`, `herdr-challenger`, `herdr-review`, `sol-final`, or `owner`. The role is recorded inside the report, never inferred from the filename.
- If a reviewer lane is unavailable, substitute a different reviewer/session with no authorship conflict, record it in `DECISIONS.md`, and keep the card in `REVIEW` until the owed independent report exists.
- Herdr Runtime Server is the default session and handoff surface for all non-final-audit work.
- Tests, deterministic oracles, provenance, and human decisions control promotion.

## Parallelism

- `REG-01` remains BLOCKED until `PRO-02` is DONE and the owner separately authorizes resumption. Other cards retain their current board gates.
- `WRK-01` and `WRK-02` may run in parallel after dependencies merge.
- `EVAL-01` and `EVAL-02` may run in parallel; then `EVAL-03` and `EVAL-04` may run in parallel.
- From `AUD-01` onward, serialize except where a later accepted ADR explicitly changes the security order.

## Current owner action

Done: ARC-01, ARC-02, ARC-03, and BASE-01 are complete. ADR-0003 is accepted, and the minimal Rust workspace scaffold passed deterministic validation and independent exact-SHA review.

Next: PRO-02 — registry descriptor identity/version contract. PRO-02 is the only `READY` card. REG-01 remains `BLOCKED` pending PRO-02 and a separate owner authorization; POL-01, SEC-01, WRK-01, and WRK-02 remain `BLOCKED`.
