# ARC-03 independent exact-SHA re-review — Sol

RUN_TASK: ZHOOR-ARC03-SOL-CHALLENGE-01
ROLE / HARNESS / OBSERVED MODEL / REASONING / SESSION: Independent challenger #2 / Pi / controller-verified `openai-codex` / `gpt-6-sol` / high / read-only session, no ARC-03 authorship.
REVIEWED_SHA: `b81410be5caf915dfece4b98a86f66ca223a1320`
AUDIT_RESULT: CHANGES_REQUIRED

## Finding

1. **Medium — BASE-01 still contradicts the corrected lockfile workflow.** `docs/development/ZHOOR_TOOLCHAIN.md` and component-card timing paragraphs allow pre-integration offline checks without `--locked`, with any Cargo-generated root lockfile diff reported but not staged or committed by the component worker. BASE-01's Write scope still says component workers use only already-locked dependencies and `--locked` validation. Sol requests changing that ownership/timing guidance to distinguish the worker pre-integration checks from controller-owned lockfile reconciliation and mandatory locked targeted/full checks after integration; generated lockfile changes may be reported but never staged or committed by the component worker.

The earlier scope finding is resolved: `TARGET_SCAFFOLD.md` is absent from the candidate change set. Its stale illustration is not a blocker because ADR-0003 explicitly supersedes it and BASE-01 must follow the accepted ADR, toolchain document, and exact card paths. The schema finding is resolved by Draft 2020-12 plus schema validation of every valid/invalid protocol fixture and paired Rust acceptance/rejection in `protocol_contract`; this creates a fixture-conformance gate without a production schema runtime. Exhaustive equivalence remains a future review risk.

## Recovery simulation

After BASE-01 wording correction, compare its ownership/validation guidance to the toolchain document and PRO-01. Walk a newly added workspace member through offline unlocked worker checks, an uncommitted generated lockfile diff, controller-owned reconciliation, then locked targeted/full validation on the integration candidate before card completion. Confirm no status, ownership root, dependency, or promotion-gate changes.

## Residual risk

Offline checks require cached dependencies and the pinned toolchain. ARC-02's noisy JSONL measurements do not establish production latency, memory, or lifecycle behavior. Schema fixture parity is bounded evidence, not exhaustive equivalence. Unix peer identity, grant verification, postconditions, and append-only receipt guarantees remain downstream security gates. No runtime capability or provider dependency is added here.

## Validation reported by reviewer

Inspected the complete controller-provided `87ae6b8e...` to `b81410be...` correction diff, ADR-0003, toolchain document, BASE-01/PRO-01 cards, target scaffold, ARC-03 card/handoff, and work board. Exact detached checkout SHA was verified from the checkout's HEAD file. Sol did not read another reviewer's report.

Unrun: Git commands, Rust/Python source checks, and `./scripts/validate.sh`; the reviewer had file-reading tools but no command-execution tool. BASE-01 has not created the root workspace or validation script.
