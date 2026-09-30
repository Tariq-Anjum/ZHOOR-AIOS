# SEC-01 implementation handoff

## Run and authority

- RUN_ID: `ZHOOR-20260930T140229Z-SEC-01`
- Card: SEC-01; remains authorized by D-024 and assigned at `b9a9c3068d9fe41756edf8fda8afcc8c121bb193`.
- Starting published SHA: `6328c75ec05fbf60a3be7ae56a4724d1f94ee0f9`.
- Owner security decision: ADR-0004 / D-025, commit `98329749c5d93600d2147e08b8e6889d20dd1dc6`; Alternative B.
- Implementation base: `98329749c5d93600d2147e08b8e6889d20dd1dc6`.
- Route: controller-authored in isolated detached worktree `/tmp/zhoor-sec-01/ZHOOR-20260930T140229Z-SEC-01/implementation-b`.
- No implementation worker, daemon, verifier service, or secondary challenger was started.

## Contract implemented

- The sole v1 capability is non-mutating `capabilities.describe`, with exactly `{}` arguments and no defaults or unknown members. Normalized bytes are UTF-8 `{}` and the argument binding is lowercase `sha256:` plus SHA-256 of those exact bytes.
- The only scope is `registered_capability_descriptor_metadata`; it contains no path semantics.
- Grant v1 binds its version/ID, issuer ID, Unix UID claim, capability, argument digest, scope, policy version, expiry, trace ID, decision hash, and complete protocol evidence bindings. Its ordered TLV transcript is domain-separated, versioned, unambiguous, and proof-independent. Proof bytes are not self-bound; the authenticator format is bound.
- The required authenticator envelope is closed `{format, proof}`. `format` is a bounded identifier; `proof` is 1–4096 opaque bytes as lowercase hex. SEC-01 checks representation only. A parsed grant remains untrusted until GRANT-01 verifies a supported format against its trust root. No issuer ID, digest, socket identity, or schema result establishes authenticity.
- The caller claim is `kind: unix_uid` plus a u32 UID. A later trusted peer-credential value is supplied explicitly to the deterministic binding check. SEC-01 does not authenticate the peer or define GID/PID authorization.
- Expiry is unsigned Unix epoch milliseconds; validation receives trusted time explicitly; `trusted_now >= expires_at` is expired. Remaining lifetime is capped at 30 seconds. GRANT-01 owns clock source and enforcement.
- Grant ID uses lowercase UUID syntax and single-operation semantics. An explicit validation context can report consumed/replayed. SEC-01 owns no state; GRANT-01 owns generation, uniqueness, atomic consume, and replay state.
- Evidence uses protocol `EvidenceRef`; at least one reference and a decision hash are required. Evidence bindings sort by canonical reference bytes and include reference digest, kind, trust tier, and exact UTF-8 source.
- Framing is a 4-byte big-endian length plus one complete JSON message, bounded to 65,536 payload bytes, version/kind/correlation bound, and no trailing data. No socket code exists.
- Receipts bind request/grant/caller/capability/argument/outcome/result reference/timestamps/trace/decision/evidence. They record execution or observation and are not postcondition proof.
- Postcondition requirements identify a mandatory condition and deterministic verification owner. Verification records distinguish `satisfied`, `failed`, and `unknown`, bind evidence and request/trace/grant/receipt, and must match the requirement and request-bound receipt. Failed and unknown do not satisfy the promotion prerequisite; no policy/state engine or evidence verifier exists here.
- Path/traversal/symlink-shaped scope values reject because no path scope exists for this capability. No filesystem canonicalization or symlink resolution is claimed.

## Files and dependencies

Implementation paths are limited to `schemas/aiosd/**`, `crates/aiosd-contracts/**`, `fixtures/aiosd/**`, `docs/contracts/aiosd/**`, and this handoff. Protocol is read-only. Root `Cargo.toml` is unchanged. The package uses existing cached serde/serde_json/protocol/SHA-256 tooling and `jsonschema` as a dev dependency; Cargo ran offline.

The worker Cargo command mechanically added only the new workspace package record to its local root `Cargo.lock`. It was left unstaged and uncommitted under the component lockfile rule; the controller reconciles the canonical lockfile after integration.

## Pre-integration validation

Toolchain: installed Rust `1.98.1`; commands were offline.

- `cargo +1.98.1 fmt --all -- --check` — PASS.
- `cargo +1.98.1 clippy --offline -p zhoor-aiosd-contracts --all-targets -- -D warnings` — PASS.
- `cargo +1.98.1 check --offline -p zhoor-aiosd-contracts --all-targets` — PASS.
- `cargo +1.98.1 test --offline -p zhoor-aiosd-contracts` — PASS (2 unit, 8 contract, 0 doctests).
- `cargo +1.98.1 build --offline -p zhoor-aiosd-contracts` — PASS.
- `cargo +1.98.1 build --offline --release -p zhoor-aiosd-contracts` — PASS.
- Schema tests validate positive request/receipt/postcondition/frame inputs and hostile structural/semantic boundaries against the Rust parsers.
- Task-scoped `git diff --cached --check` — PASS.

## Review and canonical integration

Canonical integration SHA, root lock reconciliation, locked targeted/full validation, exact review candidate, review route/verdict, D-018, closure SHA, and publication SHA are recorded here after those gates complete. SEC-01 remains `ASSIGNED` until canonical validation has passed and the separate `ASSIGNED → REVIEW` governance transition is committed.

## Ownership and residual limits

No authenticator algorithm, key store, trust root, issuer/verifier implementation, daemon, executor, socket transport, peer-credential lookup, replay store, privilege model, multi-user policy, or filesystem enforcement was implemented. No capability was executed. GRANT-01 and all later cards remain blocked unless separately authorized.
