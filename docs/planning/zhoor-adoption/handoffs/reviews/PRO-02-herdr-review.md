# PRO-02 independent exact-SHA review

- Role: independent read-only reviewer
- Harness: Pi
- Provider route: OpenAI/Codex
- Model/reasoning observed: GPT-6 Luna / xhigh
- Herdr tab/pane: `wH:t9` / `wH:pH`
- `AGENT_CONTAINMENT`: PASS before source-bearing packet
- Exact reviewed candidate SHA: `7100cb6fff25cb49238e393cb55f540e143ff04d`
- Result: **PASS** — no blocking findings remain

## Findings

1. **Contract distinctions:** Descriptor-format version, artifact digest, and registry snapshot version are separate in `jobs/PRO-02.md:22–25` and `crates/protocol/src/registry_identity.rs:14–28, 88–127, 149–155`.
2. **Protocol-v1 compatibility:** The exact diff does not modify existing request/response schemas, envelope implementations, or fixtures. Request candidates remain strings (`schemas/protocol/request-v1.schema.json:22, 34–38`; `crates/protocol/src/request.rs:141–155`).
3. **Hash semantics:** SHA-256 covers supplied raw bytes, is formatted lowercase, and uses the required grammar (`registry_identity.rs:96–127`; `schemas/protocol/registry-snapshot-v1.schema.json:20–23`). Inspected tests cover a known vector, changed bytes, and malformed hashes (`crates/protocol/tests/protocol_contract.rs:1266–1297`).
4. **Registry scope and mismatch:** The `my-ai-skills` source-of-truth repository and runtime-copy distinction ground the canonical-source scope; no global ID is introduced (`handoffs/PRO-02.md:23–25`). Scope is unambiguous. Expected-version mismatches return an explicit error (`registry_identity.rs:192–207`; contract test source `protocol_contract.rs:1250–1263`).
5. **Ownership:** The metadata schema is composable; full root closure, descriptors, and lookup remain REG-01-owned (`schemas/protocol/registry-snapshot-v1.schema.json:5–11`; `registry_identity.rs:159–163`; `handoffs/PRO-02.md:27, 31`).
6. **Fail-closed coverage:** Inspected tests and fixtures cover valid/invalid root metadata, absent/malformed/unsupported format, missing/malformed registry version, hash grammar/byte identity, and expected-version mismatch (`protocol_contract.rs:1196–1297`).
7. **Security claims:** Hash semantics are limited to content integrity; no authentication, trust, authorization, anonymization, or signature claim is made (`registry_identity.rs:86–92`; schema line 23).
8. **Scope leakage:** The exact diff adds no registry implementation or later consumer. Changes remain protocol primitives, schema, fixtures, tests, and authorized lifecycle evidence.

## Residual limitation

Non-reuse of a snapshot label for different contents is a source-owner contract rule. A stateless metadata primitive cannot verify reuse across snapshot history.

The reviewer inspected source, schema, fixtures, and test code only. It did not run Cargo or other commands. Controller validation is recorded separately in the implementation handoff.
