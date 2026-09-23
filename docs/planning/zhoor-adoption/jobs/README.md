# Job cards

Each file is one bounded logical work unit, normally one agent session. The owner assigns cards according to `../WORK_BOARD.md` and `../ORCHESTRATOR_RUNBOOK.md`; the board is the only status authority.

## Sequence

1. `ARC-01` governance review
2. `ARC-02` toolchain evidence spike
3. `ARC-03` toolchain adjudication, path/check pinning
4. `BASE-01` minimal scaffold
5. `PRO-01` protocol contracts
6. Parallel: `REG-01`, `POL-01`, `TRC-01`, `SEC-01`
7. Parallel: `WRK-01`, `WRK-02`
8. `EVAL-00` shared harness
9. Parallel first batches: `EVAL-01`, `EVAL-02`
10. Parallel completion batches: `EVAL-03`, `EVAL-04`
11. `AUD-01` deterministic slice gate
12. `BENCH-01` adapter/runner
13. `BENCH-02` approval-gated measured run
14. `SHADOW-01` no-effect shadow mode
15. `GRANT-01` policy grant authority
16. `AIOSD-01` enforcement core/receipts
17. `AIOSD-02` Unix peer/transport lifecycle
18. `AUD-02` promotion gate
19. `CANARY-01` proposal only

Every card explicitly loads `skill-router` and `karpathy-guidelines` and inherits the two-attempt stop rule from `AGENT_OPERATING_RULES.md`. Do not combine cards or begin a blocked card. Interrupted work may resume only the same card using the existing handoff and attempt count.
