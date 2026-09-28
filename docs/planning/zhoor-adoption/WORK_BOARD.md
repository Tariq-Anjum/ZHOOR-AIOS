# ZHOOR adoption work board

The owner/orchestrator is the only editor of this file. This is the sole status authority; job cards intentionally contain no status field. Implementation agents accept assigned `READY` cards; review agents accept assigned `REVIEW` cards.

Status values: `READY`, `ASSIGNED`, `BLOCKED`, `REVIEW`, `DONE`, `DEFERRED`.

Before any downstream implementation card becomes `READY`, `ARC-03` must replace descriptive write areas with concrete repository-relative path globs and exact targeted/full validation commands. Parallel cards must be mechanically disjoint.

## Board

| ID | Status | Depends on | Deliverable | Recommended runtime role | Independent review role | Planned exclusive area |
|---|---|---|---|---|---|---|
| ARC-01 | DONE | none | Governance baseline review and owner-adjudicated closure | architecture-primary | independent-challenger + owner | review handoff only |
| ARC-02 | READY | ARC-01 | Reproducible Rust/Python/bounded-mixed evidence spike; evidence only | implementation-low-cost | independent-challenger before DONE | `docs/planning/zhoor-adoption/evidence/ARC-02/`; disposable builds under `/tmp/zhoor-arc02/<RUN_ID>/` |
| ARC-03 | BLOCKED | ARC-02 DONE + independent review | Toolchain ADR, canonical commands, pinned paths | architecture-primary | independent-challenger + owner | ADR/toolchain docs/board/card path sections |
| BASE-01 | BLOCKED | ARC-03 | Minimal verified project scaffold | implementation-low-cost | independent-challenger | root manifests/config, smoke source/test, CI |
| PRO-01 | BLOCKED | BASE-01 | Versioned decision protocol and fixtures | implementation-low-cost | security-reviewer | protocol schemas/package/tests |
| REG-01 | BLOCKED | PRO-01 | Compact descriptor registry/index loader | implementation-low-cost | evaluation-reviewer | registry package/fixtures/tests |
| POL-01 | BLOCKED | PRO-01 | Deterministic risk rules and recommendations | implementation-low-cost | security-reviewer | risk-policy package/fixtures/tests |
| TRC-01 | BLOCKED | PRO-01 | Sanitized trace writer and replay | implementation-low-cost | evaluation-reviewer | trace package/fixtures/tests |
| SEC-01 | BLOCKED | PRO-01 | `aiosd`/grant contracts and hostile fixtures | security-reviewer | independent-challenger | aiosd schemas/fixtures/contract tests |
| WRK-01 | BLOCKED | REG-01, TRC-01 | Deterministic `skill_router` | implementation-low-cost | evaluation-reviewer | skill-router worker/tests |
| WRK-02 | BLOCKED | POL-01, TRC-01 | Deterministic non-authoritative `risk_judge` | implementation-low-cost | evaluation-reviewer | risk-judge worker/tests |
| EVAL-00 | BLOCKED | WRK-01, WRK-02 | Shared deterministic evaluation harness | evaluation-reviewer | independent-challenger | eval harness/report contract/tests |
| EVAL-01 | BLOCKED | EVAL-00 | First 25–50 skill-router cases | evaluation-reviewer | independent-challenger + human spot-check | skill dataset batch 1 |
| EVAL-02 | BLOCKED | EVAL-00 | First 25–50 risk cases | evaluation-reviewer | independent-challenger + human spot-check | risk dataset batch 1 |
| EVAL-03 | BLOCKED | EVAL-01 | Complete 50–100 skill cases and aggregate report | evaluation-reviewer | independent-challenger + human spot-check | skill dataset batch 2/report |
| EVAL-04 | BLOCKED | EVAL-02 | Complete 50–100 risk cases and aggregate report | evaluation-reviewer | independent-challenger + human spot-check | risk dataset batch 2/report |
| AUD-01 | BLOCKED | EVAL-03, EVAL-04 | Deterministic slice architecture/security audit | security-reviewer | human owner | review handoff only |
| BENCH-01 | BLOCKED | AUD-01 | Backend adapter, mocks, frozen benchmark runner | implementation-low-cost | evaluation-reviewer | benchmark adapter/runner/tests |
| BENCH-02 | BLOCKED | BENCH-01 | Approval-gated measured run and report | evaluation-reviewer | independent-challenger | benchmark configs/results/report |
| SHADOW-01 | BLOCKED | BENCH-02 | No-effect shadow integration/report | implementation-low-cost | evaluation-reviewer | shadow adapter/tests/report |
| GRANT-01 | BLOCKED | SEC-01, SHADOW-01 | Policy-owned grant issuer/verifier for `capabilities.describe` | security-reviewer | independent-challenger | policy grant package/tests/docs |
| AIOSD-01 | BLOCKED | GRANT-01 | Enforcement core and immutable append-only receipts | security-reviewer | independent-challenger | aiosd core/receipt tests |
| AIOSD-02 | BLOCKED | AIOSD-01 | Unix transport, peer identity, lifecycle hardening | security-reviewer | independent-challenger | aiosd transport/integration tests |
| AUD-02 | BLOCKED | AIOSD-02 | Promotion and threat-model audit | security-reviewer | human owner | review handoff only |
| CANARY-01 | BLOCKED | AUD-02 | One reversible canary proposal | architecture-primary | human owner | proposal ADR only |

## Assignment rules

- Architecture/security acceptance uses independent evidence and owner review. The concrete current reviewer lanes are in `MODEL_AND_SKILL_MATRIX.md`.
- Independence requires a different reviewer and session with no authorship conflict; model/provider family is not an authority requirement.
- Review files are `<JOB-ID>-<reviewer-token>.md`; use `herdr-primary`, `herdr-challenger`, `herdr-review`, `sol-final`, or `owner`. The role is recorded inside the report, never inferred from the filename.
- If a reviewer lane is unavailable, substitute a different reviewer/session with no authorship conflict, record it in `DECISIONS.md`, and keep the card in `REVIEW` until the owed independent report exists.
- Herdr Runtime Server is the default session and handoff surface for all non-final-audit work.
- Tests, deterministic oracles, provenance, and human decisions control promotion.

## Parallelism

- After `PRO-01`, `REG-01`, `POL-01`, `TRC-01`, and `SEC-01` may use separate worktrees only after ARC-03 has pinned disjoint paths.
- `WRK-01` and `WRK-02` may run in parallel after dependencies merge.
- `EVAL-01` and `EVAL-02` may run in parallel; then `EVAL-03` and `EVAL-04` may run in parallel.
- From `AUD-01` onward, serialize except where a later accepted ADR explicitly changes the security order.

## Current owner action

Done: ARC-01 adjudication and closure audit passed; current identity is `Tariq-Anjum/ZHOOR-AIOS` on `integration/zhoor-adoption`.

Next: start ARC-02's language/toolchain evidence spike in a fresh session. Produce only bounded, non-production evidence and complete its separate independent review before ARC-02 becomes DONE and ARC-03 becomes READY. Unblock one wave at a time.
