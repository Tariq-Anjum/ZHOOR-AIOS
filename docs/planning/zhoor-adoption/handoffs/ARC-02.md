# ARC-02 evidence handoff

RUN_ID: `ZHOOR-20260928-ARC02-TOOLCHAIN-01`

## Candidate and state

- Starting/base SHA: `4c0893e44eb7b38e74ccb504409cfc9c8be60749`
- Evidence content commit: `dfaa17974410158e497a67e2386a9c7ec94294e3` (`docs(arc-02): add toolchain evidence spike`)
- Current handoff is being added after that evidence commit; the independent review packet will pin the exact resulting SHA.
- Evidence author lane: Herdr Pi 0.87.1, `openrouter/cohere/north-mini-code:free` (OpenRouter free tier); the final comparison source and results were completed and rerun by the Codex orchestrator after the worker's partial draft hit shared-capacity limits and contained a Python syntax error. No OpenAI or Anthropic model was routed through OpenRouter.
- Board has not been changed during evidence collection/commit. Current card state remains ARC-02 READY; ARC-03 BLOCKED.

## Environment

CachyOS rolling Linux, kernel `7.2.8-1-cachyos`, x86_64; AMD Ryzen 7 7840U, 16 logical CPUs, 15,649,688 kB total RAM and 8,107,732 kB available at baseline. `rustc 1.98.1`, Cargo `1.98.1`, Python `3.14.7`, GCC `16.2.1`, `pkg-config 3.0.7`. `hyperfine`, `/usr/bin/time`, and `uv` were absent. Full record: `docs/planning/zhoor-adoption/evidence/ARC-02/environment.md`.

## Reproduction commands

From repository root, with no installation/network access:

```sh
ARC02_OUTDIR=/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01/review-replay \
  python3 docs/planning/zhoor-adoption/evidence/ARC-02/scripts/reproduce.py
```

This builds the Rust spike offline, compiles/runs both peer-credential probes, compares all fixture responses across the three options, and records 30 fresh-process runs plus 120 warm requests per option. Run 3 completed successfully: 12 valid and 14 invalid fixtures; fixture verification took 0.088 seconds; fresh Rust build took 4.612 seconds. Its measurements are recorded in `results.md`; cold-start run-to-run spread is also shown there.

Existing Rust empirical context, separately from the spike:

```sh
CARGO_NET_OFFLINE=true \
CARGO_TARGET_DIR=/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01/system-info-target \
cargo test --offline --manifest-path crates/system-info/Cargo.toml

CARGO_NET_OFFLINE=true \
CARGO_TARGET_DIR=/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01/system-info-target \
cargo build --offline --release --manifest-path crates/system-info/Cargo.toml
```

Observed: 11 tests passed in 5.008 seconds; release build 4.707 seconds, artifact 620,512 bytes. This crate was not treated as the Rust spike or as a language decision.

## Measurements and dependency inventory

| Class | Cold process p50/p95 (run 3) | Warm p50/p95 (run 3) | Idle / peak RSS (run 3) | Runtime deps and build requirements |
|---|---:|---:|---:|---|
| Rust-first | 1.158 / 1.811 ms | 0.0098 / 0.0764 ms | 2,580 / 2,600 kB | 2 direct crates (`serde`, `serde_json`), 11 dependency packages in lockfile; Rust/Cargo/linker and cached crates for offline build; 641,592-byte release binary |
| Python-first | 22.985 / 44.724 ms | 0.0163 / 0.1198 ms | 13,260 / 13,328 kB | Python 3 stdlib only; no third-party dependency or compiled app artifact; runtime source 2,303 bytes |
| Bounded mixed | 44.217 / 84.376 ms | 0.0252 / 0.2843 ms | Python 16,136/16,136 kB; Rust child 2,560/2,644 kB | Same Rust core plus Python stdlib harness; Rust/Cargo at build and Python at run; two processes; harness 707 bytes |

The timing method includes process/pipe overhead. Throughput is sequential only. RSS is process-level `/proc` data, not cgroup accounting. See the full required comparison matrix, test ergonomics, packaging details, security/dependency surface, and run-to-run measurements in `evidence/ARC-02/results.md`; methodology is in `evidence/ARC-02/methodology.md`.

## Unix socket and peer credentials

Both local AF_UNIX probes retrieved PID/UID/GID on this Linux workstation. Python used only stdlib `socket.getsockopt(SOL_SOCKET, SO_PEERCRED)` and `struct.unpack`, without unsafe code. Rust's `UnixStream::peer_cred()` failed to compile as unstable on the installed compiler; its probe uses one unsafe `getsockopt(SOL_SOCKET=1, SO_PEERCRED=17)` FFI declaration and Linux `ucred` layout, with no additional crate. The mixed runtime identity boundary remains in Rust; Python's evaluation harness uses JSONL stdin/stdout and makes no peer-authentication claim.

## Evidence paths

- `docs/planning/zhoor-adoption/evidence/ARC-02/environment.md`
- `docs/planning/zhoor-adoption/evidence/ARC-02/methodology.md`
- `docs/planning/zhoor-adoption/evidence/ARC-02/results.md`
- `docs/planning/zhoor-adoption/evidence/ARC-02/{rust,python,mixed,fixtures,scripts}/`

All executable/source artifacts are marked `NON-PRODUCTION ARC-02 SPIKE`. Build outputs and measurements are only under `/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01/`.

## Recommendation and limits

For ARC-03 to consider, the measured startup/RSS evidence favors a Rust deterministic core on this target, with Python for offline evaluation and fixtures. The bounded mixed design avoids duplicating decision logic but adds a process and had the slowest cold start in this exercise. This remains explicitly non-binding; ZHOOR has not selected a production toolchain.

Main uncertainty: the exercise is tiny, single-threaded, and stdio-based; runs show workstation noise; peer-credential probes do not cover adversarial clients or service lifecycle; agent implementation effort was not controlled. ARC-03 must adjudicate whether these observations generalize to the actual protocol and enforcement boundary.

## Validation

`git diff --cached --check` passed for the evidence commit. `reproduce.py` passed three recorded fresh-target runs; all three options returned identical outputs for the shared fixtures. No protected canonical paths were staged or edited. The handoff is not a board transition and does not begin ARC-03.
