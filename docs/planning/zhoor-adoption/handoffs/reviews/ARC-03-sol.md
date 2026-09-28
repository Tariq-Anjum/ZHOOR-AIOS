# ARC-03 independent exact-SHA review — Sol

RUN_TASK: ZHOOR-ARC03-SOL-CHALLENGE-01
ROLE / HARNESS / OBSERVED MODEL / REASONING / SESSION: Independent challenger #2 / Pi / controller-observed `openai-codex` / `gpt-6-sol` / high / fresh read-only session; no ARC-03 authorship.
REVIEWED_SHA: `87ae6b8e4207138b8fceb9020c648851e992e36a`
AUDIT_RESULT: CHANGES_REQUIRED

## Findings

1. **High — Locked package checks could not pass before the integration-owned root lockfile is reconciled.** `jobs/BASE-01.md` defines a `crates/*` workspace and says `Cargo.lock` is integration-owned, while component cards require `--locked` and forbid lockfile edits. Adding workspace members can change the lockfile graph. Sol requested explicit pre-integration versus post-integration checks and a controller-owned lockfile reconciliation gate before card completion.
2. **High — Candidate changed outside ARC-03's declared write scope.** `TARGET_SCAFFOLD.md` was absent from `jobs/ARC-03.md`'s write scope and the handoff's file list. Sol requested owner authorization to expand scope or removal of that change.
3. **Medium — Schema/Rust agreement lacked an enforceable protocol gate.** ADR-0003 named JSON Schema canonical and Rust the only production validator, but the PRO-01 card did not pin a dialect or require CI-checked schema/fixture conformance and Rust acceptance/rejection agreement beyond examples. Sol requested an explicit dialect and paired conformance suite; any schema checker should avoid an unnecessary production dependency.

Sol otherwise found Rust-first defensible for the bounded evidence, described performance claims accurately, and found the documented parallel ownership roots mechanically disjoint. Residual risks include lack of production concurrency/service lifecycle/16 GB capacity evidence and future review of Unix peer identity, grants, postconditions, receipts, supply chain, and offline caches.

## Validation reported by reviewer

Sol inspected the complete controller-provided base-to-candidate diff and candidate ADR-0001/0002/0003, architecture, toolchain doc, target scaffold, ARC-03 card, work board, ARC-02 evidence, and downstream BASE-01–CANARY-01 cards. It did not run Git/Cargo/Python checks because its fresh reviewer toolset was read-only file access. `BASE-01` has not yet created the root workspace or validation script.

## Controller disposition

Findings 1 and 3 were confirmed and corrected in the controller's working tree. Finding 2 was confirmed; the `TARGET_SCAFFOLD.md` edit was removed from the corrected candidate to stay within ARC-03's declared write scope. The old scaffold remains illustrative and its attribution/layout inconsistency is recorded as a residual documentation risk for owner consideration. Focused exact-SHA re-review is pending.
