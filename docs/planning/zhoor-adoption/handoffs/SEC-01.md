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

## Review correction r1

- Initial exact-SHA candidate `cf4dd5228bb363f41abefa0ffc065078a4637dc7` received `CHANGES_REQUIRED` from fresh contained Pi / OpenAI-Codex / GPT-6 Luna / xhigh review; immutable findings are recorded in `docs/planning/zhoor-adoption/handoffs/reviews/SEC-01-luna-r1.md`.
- Corrected duplicate raw JSON member handling by invoking typed serde parsing directly after byte/array preflight, before generic `Value` parsing; added duplicate issuer/expiry regressions. Updated the UID claim documentation to distinguish the serialized claim from separately supplied peer identity.
- Added a 375-byte spec-derived frozen claims-transcript golden vector, evidence-order invariance coverage, and transcript change/rejection checks for each variable or closed v1 claim.
- Correction worker validation on Rust 1.98.1, offline: fmt check, clippy, check, tests (2 unit + 12 contract), debug build, release build, and task-scoped diff check — PASS.
- Correction worker Cargo.lock unchanged; canonical lock reconciliation remains controller-owned.
- Corrected canonical integration commit: `ebfcc243b66bf720f1ee75c82160aa88b940fe7e`.
- The first independent review lane was stopped after its report. Its three findings were corrected in `ebfcc243b66bf720f1ee75c82160aa88b940fe7e`; final exact-SHA review evidence is recorded below.

## Review correction r2

- Fresh exact-SHA candidate `4456113a13143f95cd086c36ecf71a2257f5242a` received `CHANGES_REQUIRED` from the contained Pi / OpenAI-Codex / GPT-6 Luna / xhigh reviewer. Immutable findings are in `docs/planning/zhoor-adoption/handoffs/reviews/SEC-01-luna-r3.md` (report commit `2f4e68c36a24a2bfa77e21cffcb23000d27ed07c`).
- Finding 1: JSON Schema `uniqueItems` compared complete evidence objects while Rust rejected duplicate evidence references. Changed SEC v1 evidence wire values to a closed object keyed by canonical protocol evidence reference; schema key grammar and 1–64 bounds now match typed parsing. The bounded Rust map visitor rejects malformed keys, duplicate decoded raw keys (including escaped duplicates), unknown metadata fields, and more than 64 entries before vector growth. Internal evidence bindings and the frozen claims transcript remain sorted by canonical reference.
- Finding 2: schema descriptor descriptions rejected ASCII controls while Rust rejected all Unicode controls. Extended the schema control range through U+009F and added a U+0085 schema/parser regression.
- Updated positive fixtures and contract documentation for evidence maps. Added 65-entry and duplicate raw-key cases. No dependency or protocol-owned changes.
- Correction worktree `/tmp/zhoor-sec-01/ZHOOR-20260930T140229Z-SEC-01/correction-r2`, based on report commit `2f4e68c36a24a2bfa77e21cffcb23000d27ed07c`; task-owned files only. Root Cargo.lock remained unchanged at SHA-256 `817767a6295e13774cb4f23264ca1fe001e434841e2df79241c7aa03a21ceae1`.
- Worker validation with Rust 1.98.1, offline: fmt check, clippy, check, tests (2 unit + 12 contract), debug build, release build — PASS. Task-scoped `git diff --check` — PASS.
- This correction invalidated all prior PASS results; the fresh exact-SHA r5 review and its verdict are recorded below.

## Containment recovery and final review

- Root cause: installed Pi v0.99.1 routes interactive `!` commands through `user_bash`, separately from ordinary `tool_call`. The canonical read-only guard in `scripts/zhoor_pi_readonly_guard.ts` intercepted only `tool_call`; therefore `--tools read,grep,find,ls` did not block Pi's interactive shell path. `scripts/zhoor_herdr.py` copies that guard. Neither canonical script was modified in this SEC review.
- RED reproduction: with the original guard in a fresh disposable checkout containing no SEC source, `! /usr/bin/printf ZHOOR_USER_BASH_SENTINEL` executed and printed the sentinel; the checkout stayed clean.
- GREEN: an out-of-repository controller copy added only a `user_bash` handler returning `ZHOOR read-only review containment forbids user shell execution` and exit 126. The same harmless sentinel was blocked with no output. Read/grep/find/ls inside the test checkout succeeded; outside-worktree read was blocked; write and bash model tools were absent.
- Final review used the immutable disposable guard at `/tmp/zhoor-audit/ZHOOR-20260930T140229Z-SEC-01/review-tools-r5/zhoor_pi_readonly_guard.ts`, SHA-256 `fc5f4ad477bb389eec6550996d6c9dda4950132b3b26dbc46aae1d8c6ecd51b9`, mode 0400. Its actual reviewer-lane sentinel probe was blocked before source packet. Containment evidence: `/tmp/zhoor-audit/ZHOOR-20260930T140229Z-SEC-01/reviewer-scratch-r5/containment-evidence.md`; scratch mode 0700.
- One attempted r4 lane was stopped without a verdict after its architecture read exceeded the bounded section 8 range. It is not review evidence and was replaced by fresh r5.
- Final source candidate `7783d20b350290a22f197d13923e127767c13ab5` received `PASS`, no findings, from fresh Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only independent review. Immutable report: `docs/planning/zhoor-adoption/handoffs/reviews/SEC-01-luna-r5.md`. The reviewer's process was stopped and verified absent; Herdr lane recorded `COMPLETED` with `AGENT_CONTAINMENT=PASS`. The reviewer did not run Cargo/Git or claim independent SHA verification.
- Fresh canonical validation on the exact source candidate: `./scripts/validate.sh --package zhoor-aiosd-contracts` — PASS; `cargo +1.98.1 test --offline --locked -p zhoor-aiosd-contracts --test aiosd_contract` — PASS (12 tests); `./scripts/validate.sh` — PASS, exit 0, all workspace checks locked/offline. The active compiler was Rust 1.98.1.
- Candidate source diff check `git diff --check 4456113a13143f95cd086c36ecf71a2257f5242a..7783d20b350290a22f197d13923e127767c13ab5 -- schemas/aiosd crates/aiosd-contracts fixtures/aiosd docs/contracts/aiosd docs/planning/zhoor-adoption/handoffs/SEC-01.md` — PASS.
- D-018 after validation: PASS, 28 expected/current, 0 mismatches, 0 extras. Cargo.lock remained `817767a6295e13774cb4f23264ca1fe001e434841e2df79241c7aa03a21ceae1` after locked validation. Board remains SEC-01 `REVIEW`; WRK-01, WRK-02, EVAL-00, GRANT-01, AIOSD-01, AIOSD-02 and all later cards remain `BLOCKED`.
- Residual orchestration debt: permanently harden the canonical Pi read-only containment helper to block `user_bash` with regression coverage in a separately scoped controller/operations task. Do not fold that tooling change into the reviewed SEC candidate or start that task automatically.

## Canonical integration and validation

- Implementation commit: `5bcc7750763d7cdf7b5977a083fc9e4931480125`; task-owned SEC paths only.
- Controller lock reconciliation: `43789c473b700e0f03b971ba8bdb9621c206d5b2`; Cargo.lock only. Before hash was `2ead4aae23cd06bfb70b681acd6635a1fae685991e1d974a9fa9b14eda6c7f50`; after hash is `817767a6295e13774cb4f23264ca1fe001e434841e2df79241c7aa03a21ceae1`. The diff adds only the `zhoor-aiosd-contracts` package record; dependency versions were already locked.
- `./scripts/validate.sh --package zhoor-aiosd-contracts` — PASS; formatting, clippy, check, unit/contract/doc tests, debug/release builds, locked/offline.
- `cargo +1.98.1 test --offline --locked -p zhoor-aiosd-contracts --test aiosd_contract` — PASS; 12 contract tests after the review correction.
- `./scripts/validate.sh` — PASS; full locked/offline workspace validation, including all tests and debug/release builds.
- Cargo.lock SHA-256 after locked targeted validation and full validation: `817767a6295e13774cb4f23264ca1fe001e434841e2df79241c7aa03a21ceae1` (unchanged).
- Task-scoped `git diff --check` — PASS. The whole-worktree diff check continues to report pre-existing trailing whitespace only in protected dirty job/control files; those files remain byte-identical and unstaged.
- D-018: PASS, 28 expected/current, zero mismatches, zero extras after integration and validation.
- Board after validation and review assignment: SEC-01 `REVIEW`; WRK-01, WRK-02, EVAL-00, GRANT-01, AIOSD-01, AIOSD-02 remain `BLOCKED`.
- First exact review candidate: `cf4dd5228bb363f41abefa0ffc065078a4637dc7`, verdict `CHANGES_REQUIRED`; immutable report `docs/planning/zhoor-adoption/handoffs/reviews/SEC-01-luna-r1.md`. Findings were duplicate raw JSON object members being collapsed before typed parsing, insufficient golden-transcript/mutation coverage, and ambiguous UID-claim wording. All three were corrected in `ebfcc243b66bf720f1ee75c82160aa88b940fe7e` and covered by the added regression tests and documentation.
- Corrected canonical validation was rerun after `ebfcc243b66bf720f1ee75c82160aa88b940fe7e`: targeted package validation PASS; locked contract test PASS (12 tests); full workspace validation PASS. Cargo.lock remained at SHA-256 `817767a6295e13774cb4f23264ca1fe001e434841e2df79241c7aa03a21ceae1`.
- D-018 recheck after correction: PASS, 28 expected/current, zero mismatches, zero extras. The final exact-SHA review is recorded in `SEC-01-luna-r5.md` below.

## Ownership and residual limits

No authenticator algorithm, key store, trust root, issuer/verifier implementation, daemon, executor, socket transport, peer-credential lookup, replay store, privilege model, multi-user policy, or filesystem enforcement was implemented. No capability was executed. GRANT-01 and all later cards remain blocked unless separately authorized.

## Closure

- SEC-01 transitioned `REVIEW → DONE` after the exact candidate `7783d20b350290a22f197d13923e127767c13ab5` passed fresh independent review, targeted package validation, the locked 12-test contract integration, full offline/locked workspace validation, scoped diff checking, D-018 reconciliation, and the Cargo.lock gate.
- Implementation/source commits: `5bcc7750763d7cdf7b5977a083fc9e4931480125`, `43789c473b700e0f03b971ba8bdb9621c206d5b2` (Cargo.lock reconciliation), `ebfcc243b66bf720f1ee75c82160aa88b940fe7e`, and `7783d20b350290a22f197d13923e127767c13ab5`. Review evidence commits: `2f4e68c36a24a2bfa77e21cffcb23000d27ed07c` and `6b61976e45bd886c4cb314cc31a88d73f1aba874`.
- Final reviewed candidate: `7783d20b350290a22f197d13923e127767c13ab5`; final report: `docs/planning/zhoor-adoption/handoffs/reviews/SEC-01-luna-r5.md`; verdict `PASS`, no findings. Reviewer route was fresh Pi / OpenAI-Codex / GPT-6 Luna / xhigh, read-only with `AGENT_CONTAINMENT=PASS`.
- Canonical validations after the final correction: targeted package validation PASS; `cargo +1.98.1 test --offline --locked -p zhoor-aiosd-contracts --test aiosd_contract` PASS (12); full workspace validation PASS with exit 0. D-018 PASS (28/28, zero mismatches/extras). Cargo.lock remained SHA-256 `817767a6295e13774cb4f23264ca1fe001e434841e2df79241c7aa03a21ceae1`.
- Post-publication SHA will be recorded in the controller's durable checkpoint. No downstream card was started; WRK-01, WRK-02, EVAL-00, GRANT-01, AIOSD-01, AIOSD-02, and all later cards remain BLOCKED.
