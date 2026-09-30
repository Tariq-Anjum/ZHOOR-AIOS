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

## Resumed implementation

- Assignment base SHA: `ace9810760ed72232e74cf37a92d61a91b25e8c3`; implementation is isolated at `/tmp/zhoor-reg-01/ZHOOR-20260930T110000Z-REG-01/worker` on `work/reg-01-20260930-resumed`.
- The v1 JSON snapshot root is closed and contains `descriptor_format_version`, `registry_version`, and `skills`. The `skills` collection name follows the canonical local compact index. Each descriptor is closed and requires `id`, `description`, `capabilities`, `requires`, `activation`, `risk_level`, and `artifact_sha256`.
- `descriptor_format_version` and `registry_version` are checked through `RegistrySnapshotMetadata` before descriptor deserialization. Optional expected-version mismatch becomes the explicit `RegistryVersionMismatch` error.
- `artifact_sha256` is deserialized as protocol-owned `ArtifactSha256`, validating only its text grammar. REG-01 does not open, read, recompute, or verify any `SKILL.md` bytes.
- IDs, capabilities, and requirement tokens use exact case-sensitive lowercase ASCII tokens (`[a-z0-9][a-z0-9._-]*`). Capabilities must be non-empty and unique; requirements may be empty and each must match an exact token in caller-supplied `RegistryLoadContext.available_requirements`. There is no host probing or installation.
- Activation values follow the canonical catalog schema: `always`, `contextual`, `explicit`, `manual`, or `dependency`. Risk levels are `low`, `medium`, or `high`.
- Duplicate descriptor IDs fail in Rust. JSON Schema validates the closed document shape but cannot express cross-object uniqueness of the `id` property or caller-context requirement satisfaction; contract tests pin those additional deterministic Rust rules. Other schema-backed fields are tested for JSON Schema/Rust agreement. Protocol metadata schema definitions are parity-tested against PRO-02.
- Valid snapshots are sorted by ascending ID. Exact ID lookup uses binary search; exact capability lookup returns matching descriptors in ascending ID order. Matching is case-sensitive; no fuzzy/model/network path exists.
- Worker-stage checks passed offline on Rust 1.98.1: `cargo fmt --all -- --check`; Clippy with `-D warnings`; `cargo check --offline -p zhoor-registry --all-targets`; `cargo test --offline -p zhoor-registry` (9 contract tests); debug and release package builds.
- Worker resolution generated an unstaged root `Cargo.lock` diff only to add the new `zhoor-registry` package dependency block. Worker `Cargo.lock` SHA-256: `8f7a3f52711204eb3bc5ad0ddb81f5cead9184fee82cb6bc0b9f0669c153275a`; canonical starting lock SHA-256: `4447c1d03b40adb9e0217b7cf20c6845e30986555ae475a9b0fb074b19cc0e32`. The lockfile is excluded from the implementation commit for controller reconciliation.
- No REG-01 implementation commit or independent review candidate has been created yet.
