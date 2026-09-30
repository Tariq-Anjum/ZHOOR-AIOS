# SEC-01 independent security review — Luna r3

## Review identity

- Role: independent exact-SHA security reviewer.
- Route: fresh Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only.
- Herdr role/name: `sec-01-final-independent-reviewer-r3` / `sec01reviewr3`.
- Containment: `AGENT_CONTAINMENT=PASS`; disposable user_bash guard exercised in this reviewer session before source packet; one interactive pane and managed Sidebar only; clean detached checkout; model tools limited to `read`, `grep`, `find`, `ls`; no credential-like environment variable names; no credential values inspected.
- Reviewed candidate SHA: `4456113a13143f95cd086c36ecf71a2257f5242a`.
- Verdict: `CHANGES_REQUIRED`.
- Reviewer did not run Cargo or Git.

## Findings

1. **MEDIUM — evidence-reference uniqueness differs between schema and Rust.** `schemas/aiosd/aiosd-v1.schema.json:26-30` uses `uniqueItems`, which permits two evidence entries with the same `ref` but different provenance fields. The test explicitly expects schema acceptance for this case (`crates/aiosd-contracts/tests/aiosd_contract.rs:184-200`), while Rust rejects duplicate references (`crates/aiosd-contracts/src/lib.rs:972-990`). Schema-only consumers can therefore accept conflicting provenance for one content identity while Rust rejects it. Align the wire uniqueness rule across schema and parser; do not preserve this hostile case as schema-accepted.

2. **LOW — descriptor control-character rules differ.** The schema excludes ASCII controls and U+007F (`schemas/aiosd/aiosd-v1.schema.json:124`); Rust's `char::is_control` also rejects U+0080–U+009F (`crates/aiosd-contracts/src/lib.rs:1045,1106`). Schema-only consumers may accept control-bearing descriptions that Rust rejects. Align the rules and add a parity test.

## Other reviewed points

- Typed parsing precedes generic JSON parsing; duplicate-member tests cover issuer and expiry (`crates/aiosd-contracts/src/lib.rs:1238-1250`; `crates/aiosd-contracts/tests/aiosd_contract.rs:402-416`).
- Transcript implementation, documented encoding, golden vector, mutation checks, and evidence-order test agree (`crates/aiosd-contracts/src/lib.rs:187-205`; `crates/aiosd-contracts/tests/aiosd_contract.rs:295-397`).
- UID is documented as an untrusted serialized claim and compared with separately supplied peer UID (`crates/aiosd-contracts/src/lib.rs:57-64,309-316`).
- Bounded authenticator, explicit expiry/replay context, non-path scope, framing contract, receipt/postcondition linkage, and stated implementation boundaries are consistent with SEC-01 scope.

## Disposition

The finding set requires narrow corrections to the schema, contract parser/tests, and contract documentation. This report applies only to `4456113a13143f95cd086c36ecf71a2257f5242a`; a fresh exact-SHA review is required after correction.
