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

## Original blocker disposition

At the original stop, PRO-01 follow-up was required because the protocol-owned descriptor identity/version/hash contract was missing. PRO-02 has since defined and published that contract, and the owner has separately authorized REG-01 resumption under D-022. The original blocked attempt remains historical evidence and was not rewritten.

## Owner-authorized resumption after PRO-02

- Starting published checkpoint: `e09341a45658c1c7f2fd5bba113e14f4f883f98a` (local and remote matched).
- PRO-02 is DONE and published. Its accepted contract is recorded in D-021 and `handoffs/PRO-02.md`.
- D-022 records the new owner decision to resume only REG-01 and transition `BLOCKED → READY`. The earlier blocked attempt above remains part of the history.
- The post-authorization gate confirmed REG-01 was the sole READY implementation card; it was then moved `READY → ASSIGNED` in a separate controller commit.
- Implementation must start from that exact assignment commit in a fresh isolated worktree. No REG-01 implementation source has been started in this resumed run.
- POL-01, SEC-01, WRK-01, and WRK-02 remain BLOCKED. No other card is authorized.
