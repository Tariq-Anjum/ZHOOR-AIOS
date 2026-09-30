# PRO-02 handoff — owner contract selected

## Run state

- Starting local SHA: `d15138727c43877049f51326d05aba2987d88e0f`
- Starting remote SHA: `2e73ef44181bc74b55fad25e7e2fd5f1dac9eaa1`
- Prior PRO-02 authorization SHA: `431024a356f9d3c80f39a961c73f10ac4d2abc0d`
- Prior assignment SHA: `2cb8d54560079d1a012ee5014e1b96dd9f15f1a0`
- Prior blocker SHA / current starting HEAD: `f006fbea9df6efe8eaa252debd53d55d8a4dc115`
- Status: `READY — owner decision D-021 recorded; separate assignment commit pending`
- Implementation source/schema/fixture changes: none
- Validation: not run; source mutation has not started
- Exact-SHA architecture challenge: `OWNER_CONTRACT_DECISION_REQUIRED` at prior assignment SHA `2cb8d54560079d1a012ee5014e1b96dd9f15f1a0`
- Reviewer route: contained Pi / OpenAI-Codex / GPT-6 Luna xhigh / read-only; `AGENT_CONTAINMENT=PASS`; prior lane stopped
- Challenge evidence: `/tmp/zhoor-pro-02/ZHOOR-20260930T081737Z-PRO-02/architecture-challenge-result.md`

## Evidence and owner decision D-021

Protocol v1 candidates are `Vec<String>` and no existing request/response contract defines registry descriptor identity. The prior independent architecture challenge found two reasonable descriptor-format placements and required the owner to choose. The owner selected registry-root placement.

- A v1 registry snapshot carries `descriptor_format_version: "1"` and `registry_version` once at its root. Entries do not repeat the descriptor format version. Mixed formats are unsupported.
- Reject the entire snapshot before processing entries if the root format version is absent, malformed, unknown, or unsupported. There is no per-entry fallback.
- Descriptor format version identifies the descriptor field contract, not the skill artifact version. No separate artifact release-version field is defined in v1 absent evidence from an accepted contract.
- `artifact_sha256` identifies exact raw `SKILL.md` bytes using lowercase `sha256:<64 lowercase hexadecimal characters>`. It does not cover descriptor metadata, canonicalized data, or companion files. It conveys no authentication, trust, authorization, anonymization, or signature.
- `registry_version` is a registry snapshot identity label, not a descriptor-format or artifact version. A value cannot be reused for differing snapshot contents within the already established registry identity/scope. If a caller supplies an expected value and it differs from the loaded snapshot, return an explicit mismatch failure; do not silently substitute a snapshot.
- Preserve request-v1 and response-v1, including string request candidates.

## Remaining pre-source gate

Before source mutation, inspect accepted architecture and canonical registry ownership to identify the existing registry identity/scope for the non-reuse rule. Do not invent a global namespace. If two materially reasonable scopes remain, stop with `OWNER_CONTRACT_DECISION_REQUIRED` and no more than two concrete alternatives. The old implementation worktree remains clean and pinned to its former assignment SHA; remove it through normal Git worktree management and create a fresh worktree from the new assignment SHA.

PRO-02 is the sole READY implementation card under D-021. REG-01, POL-01, SEC-01, WRK-01, and WRK-02 remain BLOCKED. PRO-02 closure will not automatically resume REG-01.
