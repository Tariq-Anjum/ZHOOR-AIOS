# REG-01 handoff — blocked on descriptor contract

## Run state

- Run: `ZHOOR-20260930T074813Z-REG-01`
- Starting publication SHA: `2e73ef44181bc74b55fad25e7e2fd5f1dac9eaa1`
- Authorization SHA: `10879b85faedd3d6448093d1f6f9ec8a428c81b1`
- Assignment SHA: `b8d52d14c7d9a8afbe4c23bdbf0749bec30fbed1`
- Implementation branch/worktree: `work/reg-01-20260930074813` at `/tmp/zhoor-reg-01/ZHOOR-20260930T074813Z-REG-01/worker`
- Implementation source changes: none
- Status: blocked; no validation run

## Blocker

The live REG-01 acceptance text requires validation of descriptor `version/hash` “as approved by protocol” and explicitly says to stop for a PRO-01 follow-up if protocol lacks a required descriptor/version field.

Protocol v1 does not define that descriptor contract:

- `crates/protocol/src/request.rs` represents request candidates as `Vec<String>`; the request envelope has `protocol_version`, but no registry descriptor type or descriptor version/hash field.
- `schemas/protocol/request-v1.schema.json` constrains candidates to strings and has only the envelope `protocol_version`.
- `docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md` §4 gives the compact descriptor fields `id`, `description`, `capabilities`, `requires`, `risk_level`, and `activation`, but defines no descriptor version/hash semantics.

Implementing version/hash validation now would require REG-01 to invent contract semantics that the card assigns to protocol authority. No protocol, schema, fixture, or implementation changes were made.

## Next action

Owner/PRO-01 follow-up must define whether descriptor identity/version/hash fields are required and their exact v1 contract. Keep REG-01 BLOCKED until that contract is accepted; then resume REG-01 from a fresh assignment SHA and worktree. POL-01, SEC-01, WRK-01, and WRK-02 remain BLOCKED and were not started.
