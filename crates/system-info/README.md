# ZHOOR system information capability

This small rootless Rust package exposes one read-only capability, `system.info.read`. It accepts a versioned JSON request on stdin, validates the request, resolves the fixed capability registry entry, applies policy, reads only the kernel release from `/proc/sys/kernel/osrelease`, verifies the result, and returns a compact ledger event with the response. It never invokes a shell or accepts paths/commands as arguments. The policy denies execution when the effective UID is root or cannot be determined.

The ledger event is returned to the caller and omits the observed system values and raw request. Durable append storage is intentionally outside this milestone.

Run the deterministic suite offline:

```sh
cargo test --offline --manifest-path crates/system-info/Cargo.toml
```

Example request:

```json
{"schema_version":1,"request_id":"local-1","intent":{"capability_id":"system.info.read","arguments":{}}}
```

Pipe one request to `cargo run --offline --manifest-path crates/system-info/Cargo.toml --bin zhoor-system-info`. The process accepts no command-line arguments; errors are returned as structured JSON and produce a nonzero exit status.
