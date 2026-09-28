# NON-PRODUCTION ARC-02 comparison results

This is evidence for ARC-03 adjudication only. It is not a ZHOOR language/toolchain selection.

## Common exercise and validation

All three options process the same versioned JSONL request: `schema_version=1`, nonempty request ID and goal, operation `select`, and a candidate array of string IDs with signed 64-bit integer scores (`-9223372036854775808` through `9223372036854775807`). Values outside that range are invalid. The deterministic rule chooses the highest score, then the lexicographically smallest ID; empty candidates produce an explicit abstention. No model, network access, OS mutation, privileged operation, or `aiosd` is involved.

The shared fixture set has **14 valid** and **16 invalid** requests. Valid fixtures include both signed 64-bit score endpoints; invalid fixtures include one value above the maximum and one below the minimum, plus unknown version/operation, missing and wrong-typed fields, invalid score/ID, unexpected fields, duplicate candidate IDs, and malformed JSON. All 84 outputs (28 cases × 3 options) matched across the three implementations. This corrected the numeric-domain divergence found in the first independent review: Python now rejects scores outside the same signed 64-bit range enforced by Rust's `i64` deserializer.

## Measurements

### Cold process start

Each value is a separate run of 30 fresh processes, one valid request per process. p50 and p95 are milliseconds from Python `time.perf_counter`, including process startup, JSON handling, and one pipe round trip. The OS page cache was not flushed; this is fresh-process latency, not cold disk-cache latency.

| Option | Run 1 p50 / p95 | Run 2 p50 / p95 | Run 3 p50 / p95 |
|---|---:|---:|---:|
| Rust-first | 1.149 / 1.456 ms | 0.822 / 1.272 ms | 1.158 / 1.811 ms |
| Python-first | 32.486 / 76.811 ms | 22.252 / 23.602 ms | 22.985 / 44.724 ms |
| Bounded mixed | 99.113 / 144.677 ms | 35.433 / 41.828 ms | 44.217 / 84.376 ms |

The variation between runs is material, especially for Python and mixed startup. No CPU isolation or load control was used. Treat these as evidence about this workstation and this tiny CLI exercise, not stable service SLOs.

### Correction verification run

After the numeric-domain correction, a fourth complete reproduction used a separate fresh target directory. The offline Rust build took 5.228 seconds; fixture comparison took 0.078 seconds. It compared 14 valid and 16 invalid fixtures across all three options. The additional performance sample is noisy and does not replace the three initial runs.

| Option | Run 4 cold p50 / p95 | Run 4 warm p50 / p95 | Run 4 idle RSS / peak HWM |
|---|---:|---:|---:|
| Rust-first | 1.009 / 1.486 ms | 0.0193 / 0.0355 ms | 2,516 / 2,600 kB |
| Python-first | 71.276 / 105.875 ms | 0.0213 / 0.1254 ms | 13,248 / 13,316 kB |
| Bounded mixed | 129.931 / 191.034 ms | 0.0360 / 0.2268 ms | Python 16,244 / 16,244 kB; Rust child 2,616 / 2,636 kB |

The Run 4 socket probes again returned peer PID/UID/GID on this host. The Rust release executable remained 641,592 bytes; the socket probe was 4,556,176 bytes. The reproduction command and the full machine-readable Run 4 output are recorded in the ARC-02 handoff and `/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01/corrected-rerun-2/measurements.json` respectively; the `/tmp` output is disposable and is not committed.

### Warm request path and process memory

One process handled 120 sequential requests over stdin/stdout. Per-request latency includes Python parent-side write/flush/read overhead. Throughput is the reciprocal of mean sequential request latency, not a concurrent-load benchmark. Idle RSS was sampled after 50 ms of startup settling; `VmHWM` was sampled after the batch. Two valid 120-request measurement runs are shown; run-to-run p95 variation is substantial.

| Option | Run 2 p50 / p95 | Run 3 p50 / p95 | Idle RSS run 2 → run 3 | Peak `VmHWM` run 2 → run 3 |
|---|---:|---:|---:|---:|
| Rust-first | 0.0060 / 0.0163 ms | 0.0098 / 0.0764 ms | 2,608 → 2,580 kB | 2,628 → 2,600 kB |
| Python-first | 0.0136 / 0.1518 ms | 0.0163 / 0.1198 ms | 13,244 → 13,260 kB | 13,312 → 13,328 kB |
| Bounded mixed | 0.0177 / 0.0766 ms | 0.0252 / 0.2843 ms | Python 16,236 → 16,136 kB; Rust child 2,572 → 2,560 kB | Python 16,236 → 16,136 kB; Rust child 2,656 → 2,644 kB |

Mixed values list both processes separately; summing RSS is not a unique physical-memory measurement because shared mappings may be counted twice. This is process RSS, not cgroup or whole-service memory.

### Build and artifact size

- Three fresh-target offline Rust release builds took 5.536, 3.408, and 4.612 seconds. These include compiler invocation and dependency build from the local cache; measurements vary with workstation load. Cargo had cached dependency sources, and all build output was under `/tmp`.
- The Rust release executable was 641,592 bytes. It has two direct runtime dependencies (`serde`, `serde_json`) and 11 dependency packages in the lockfile including proc-macro/build-time dependencies.
- The Python runtime source was 2,303 bytes and has no third-party runtime dependency; it requires an installed Python interpreter. There is no standalone Python executable artifact.
- The mixed harness source was 707 bytes; it uses the same Rust binary plus Python stdlib `subprocess`, so deployment requires both a built Rust core and Python. The spike intentionally does not include a production installer or bundle.
- `reproduce.py` reported 0.058 seconds and 0.088 seconds for shared-fixture checks in runs 2 and 3. The command prints diagnostics at the first divergence.

### Unix sockets and Linux peer credentials

Observed local probes successfully created AF_UNIX streams and retrieved `pid`, `uid`, and `gid` on this Linux host.

- Python uses only stdlib `socket.getsockopt(SOL_SOCKET, SO_PEERCRED)` and `struct.unpack("3i", ...)`; no extra package or unsafe call is needed. `SO_PEERCRED` is Linux-specific; other Unix systems have different APIs/constants.
- Rust's `std::os::unix::net` supports Unix streams. On the installed Rust 1.98.1, compiling `UnixStream::peer_cred()` failed with E0658 (`peer_credentials_unix_socket` is unstable). The reproducible probe therefore calls Linux `getsockopt(SOL_SOCKET=1, SO_PEERCRED=17)` through one narrowly scoped unsafe C FFI declaration and a `repr(C)` `ucred` layout. This needs no crate, but adds unsafe ABI code and is Linux-specific. A safe wrapper crate is another possible design and would add a dependency; it was not added to this spike.
- Mixed mode keeps peer identity in the Rust core. Its Python harness is an offline evaluation/developer process boundary over JSONL stdio and does not claim to authenticate a Unix-socket caller.

## Required comparison matrix

Cell labels distinguish measurements, observations, and inferences.

| Factor | A — Rust-first | B — Python-first | C — bounded mixed |
|---|---|---|---|
| Type/schema safety | **OBSERVED:** typed request/candidate structs, Serde deserialization, `deny_unknown_fields`; malformed and wrong-typed inputs rejected. | **OBSERVED:** stdlib JSON plus explicit runtime type/key checks and a dataclass for candidates; less compiler enforcement. | **OBSERVED:** Rust owns validation/decision types; Python forwards raw JSONL only. |
| Protocol ergonomics | **OBSERVED:** compact structs map directly to JSON; errors from deserialization are collapsed to a stable typed error. | **OBSERVED:** readable code, but strict schema behavior is handwritten. | **OBSERVED:** simple process boundary and no shared-library ABI; protocol ownership is unambiguous in this spike. |
| Unix-domain sockets | **OBSERVED:** std `UnixListener`/`UnixStream` available on Unix. | **OBSERVED:** stdlib `socket.AF_UNIX` available. | **OBSERVED:** runtime socket belongs to Rust; Python harness uses stdin/stdout. |
| Peer credentials | **OBSERVED:** std `peer_cred()` remains unstable on this compiler; Linux `getsockopt` FFI works but is unsafe and ABI-specific. | **OBSERVED:** stdlib `SO_PEERCRED` works without unsafe or extra packages on this Linux host. | **OBSERVED:** same Rust FFI/identity path as A; no Python identity code in runtime boundary. |
| Packaging/deployment | **OBSERVED:** `cargo build --release --offline`, then run one native executable; build requires Rust/Cargo and cached crates. | **OBSERVED:** `python3 runtime.py`; requires Python, no packaging manager or third-party install for this spike. | **OBSERVED:** compile Rust core and ship Python harness; runtime creates two processes. |
| Test ergonomics | **MEASURED:** shared 28-case set passed; script validates exact output equivalence. | **MEASURED:** same 28 cases passed; stdlib-only invocation. | **MEASURED:** same 28 cases passed end-to-end through subprocess pipes. |
| Evaluation ergonomics | **INFERRED:** can implement an evaluator, but fixture/report tooling is less concise than the Python stdlib route. | **OBSERVED/INFERRED:** easiest direct fit for offline fixture manipulation and report generation in this spike. | **OBSERVED:** Python can own evaluation while invoking Rust; subprocess protocol/diagnostics must be maintained. |
| Runtime dependencies | **OBSERVED:** 2 direct crates; 11 locked packages total including build/proc-macro dependencies. | **OBSERVED:** 0 third-party packages; Python standard library only. | **OBSERVED:** same Rust core dependencies plus Python stdlib; two runtimes/processes. |
| Build dependencies | **OBSERVED:** Rust 1.98.1, Cargo 1.98.1, linker; offline crate sources cached. | **OBSERVED:** Python 3.14.7 only; no build step. | **OBSERVED:** Rust/Cargo and Python 3.14.7. |
| Cold startup | **MEASURED:** p50 0.822 ms, p95 1.272 ms in run 2; corrected run 4 p50/p95 1.009/1.486 ms. | **MEASURED:** p50 22.252 ms, p95 23.602 ms in run 2; corrected run 4 71.276/105.875 ms. | **MEASURED:** p50 35.433 ms, p95 41.828 ms in run 2; corrected run 4 129.931/191.034 ms. |
| Warm request latency | **MEASURED:** run 2 0.0060/0.0163 ms and corrected run 4 0.0193/0.0355 ms p50/p95 over 120 sequential pipe requests. | **MEASURED:** run 2 0.0136/0.1518 ms and corrected run 4 0.0213/0.1254 ms. | **MEASURED:** run 2 0.0177/0.0766 ms and corrected run 4 0.0360/0.2268 ms. |
| Peak/idle memory | **MEASURED:** run 2 2,608/2,628 kB and corrected run 4 2,516/2,600 kB idle RSS/`VmHWM`. | **MEASURED:** run 2 13,244/13,312 kB and corrected run 4 13,248/13,316 kB. | **MEASURED:** corrected run 4 Python 16,244/16,244 kB and Rust child 2,616/2,636 kB idle/peak, reported separately. |
| Artifact/package size | **MEASURED:** 641,592-byte release binary; native ABI/system libraries remain target assumptions. | **MEASURED:** 2,303-byte runtime source, no compiled app artifact; interpreter installed separately. | **MEASURED:** same 641,592-byte core plus 707-byte harness source and Python runtime. |
| Agent implementation complexity | **OBSERVED:** no controlled time/quality comparison. One discarded agent draft had errors; final source was controller-authored. | **OBSERVED:** same; no controlled time/quality comparison. | **OBSERVED:** same; no controlled time/quality comparison. |
| Duplicated-domain-logic risk | **INFERRED:** one language can keep validation and rule in one typed core. | **INFERRED:** one language can keep validation and rule together, though checks are dynamic. | **OBSERVED:** this harness duplicates no rule; **INFERRED:** future edits must preserve the Rust/Python process contract, while avoiding duplicate decision rules. |
| Debuggability | **INFERRED:** compiler and Serde errors help catch type drift; compact runtime diagnostics need explicit design. | **OBSERVED/INFERRED:** Python validation is direct to inspect; malformed cases are easy to reproduce. | **INFERRED:** each side can be debugged separately, with added process/pipe failure paths. |
| Failure isolation | **OBSERVED:** one executable handles the stream; a process crash terminates its requests. | **OBSERVED:** one interpreter handles the stream; a process crash terminates its requests. | **OBSERVED:** Rust child failure ends the bridge; no restart, timeout, or queue was added. **INFERRED:** the boundary gives process separation but not production resilience. |
| Security/enforcement suitability | **INFERRED:** typed validation, low observed footprint, and native socket path fit a small deterministic enforcement-facing core; peer identity still needs audited unsafe wrapper or dependency. | **INFERRED:** safe stdlib peer-credential API is attractive, but manual validation and dynamic runtime require explicit tests and process controls. | **INFERRED:** keeps enforcement-facing rule/identity in Rust and evaluation in Python, but adds an extra runtime process; no enforcement behavior was tested. |

## Agent and model evidence limitation

The assigned low-cost OpenRouter free model was Cohere North Mini Code through Pi. The first OpenCode lane had the wrong project root and was stopped before its partial temporary files were integrated. The Pi worker ran in the isolated worktree but encountered OpenRouter shared-capacity 429s; its partial Python draft contained syntax errors and was discarded. The orchestrator authored the final small comparison sources and reran them. This operational record is not a controlled comparison of Rust-vs-Python agent competency, and no relative agent competency conclusion is claimed.

## Evidence-backed, non-binding recommendation

For ARC-03 to consider, the measured startup and RSS results favor a Rust deterministic runtime core on this 16 GB target, while Python remains the simplest zero-third-party environment for offline fixtures and evaluation. The mixed stdio design avoids duplicated decision logic but had the slowest cold start and two resident processes; this small exercise does not show that a Python process belongs on a production request path. This is a bounded evidence recommendation only; ZHOOR has not selected a toolchain.

## Main remaining uncertainty

The workload is tiny, single-threaded, local, and limited to JSON parsing plus one rule. Cold-start samples vary across runs; warm measurements use sequential stdio, not Unix socket concurrency or a service supervisor. RSS is process sampling rather than cgroup accounting. The peer-credential probes do not test adversarial clients, lifecycle, or production ABI handling. Agent implementation effort was not controlled. ARC-03 must decide whether the observed tradeoffs generalize to the actual protocol, test suite, deployment, and enforcement boundary.
