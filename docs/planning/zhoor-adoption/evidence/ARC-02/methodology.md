# NON-PRODUCTION ARC-02 spike methodology

## Assumptions and workload

The target is this 16 GB-class CachyOS workstation. No toolchain, package, or benchmark utility was installed. The common version-1 request has exactly `schema_version`, `request_id`, `operation`, `goal`, and `candidates`. A score is a signed 64-bit integer in `[-9223372036854775808, 9223372036854775807]`; values outside that range are invalid. The rule chooses the highest score, resolving ties by lexicographically smallest candidate ID; an empty candidate list returns typed abstention. Unknown keys, wrong types, unsupported versions/operations, duplicate/empty IDs, and malformed JSON return typed errors.

The 14 valid and 16 invalid fixtures are shared by all three options. They cover both valid signed 64-bit score endpoints and values just outside the range. Rust-first is one typed Rust process using Serde and Serde JSON. Python-first is one stdlib Python process using dataclasses and `json`. Bounded mixed runs the same Rust process behind a Python stdio harness; the harness has no decision logic. It introduces one subprocess boundary and no FFI/RPC framework.

## Measurement

`reproduce.py` builds Rust in release mode with Cargo offline and writes the target directory beneath `ARC02_OUTDIR`. Cold start is 30 separately launched processes per option, each handling one valid request; p50 and inclusive p95 (linear interpolation over `statistics.quantiles(method="inclusive")`) are calculated from Python `time.perf_counter` elapsed time, including process launch and one pipe round trip. Warm latency is 120 requests sent one at a time through a single long-lived process (or Python parent + Rust child); p50, p95, and reciprocal mean latency are reported. `/proc/<pid>/status` supplies sampled `VmRSS` before requests and `VmHWM` after the batch. Mixed parent and child are reported separately because summed RSS double counts shared mappings. These are process-level observations, not cgroup or whole-service memory guarantees.

Unix-domain socket probes create a temporary local socket and read Linux peer credentials. The Rust probe invokes `getsockopt(SOL_SOCKET, SO_PEERCRED)` through one narrowly scoped unsafe libc FFI declaration; it uses no third-party crate. The Python probe uses stdlib `socket.getsockopt` and `struct.unpack`. These probes establish API/mechanism availability, not a production security review.
