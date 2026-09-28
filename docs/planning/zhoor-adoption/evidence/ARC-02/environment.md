# NON-PRODUCTION ARC-02 environment baseline

Observed on 2026-09-28 on the target workstation; this is a local measurement, not a server-class projection.

| Item | Observed value |
|---|---|
| Distribution | CachyOS Linux, rolling (`ID=cachyos`, `ID_LIKE=arch`) |
| Kernel | `7.2.8-1-cachyos` |
| Architecture | `x86_64` |
| CPU | AMD Ryzen 7 7840U w/ Radeon 780M Graphics |
| Logical CPUs | 16 |
| Physical memory | 15,649,688 kB reported by `/proc/meminfo` (about 14.93 GiB; marketed/design class 16 GB) |
| Available memory at baseline | 8,107,732 kB (about 7.73 GiB), observed 2026-09-28 13:34 UTC |
| Rust compiler | `rustc 1.98.1 (48a229cea 2026-09-01)`, host `x86_64-unknown-linux-gnu`, LLVM 22.1.8 |
| Cargo | `cargo 1.98.1 (797e8a9bc 2026-08-05)` |
| Python | `Python 3.14.7` |
| C compiler/linker driver | `cc` is GCC 16.2.1 |
| `pkg-config` | 3.0.7 |
| `hyperfine` | absent |
| `/usr/bin/time` | absent |
| `uv` | absent |

No toolchain, benchmark suite, package, or Python framework was installed. Measurements use Python `time.perf_counter`, `/proc/<pid>/status`, and the already installed Rust/Python toolchains. The design target remains approximately 16 GB physical RAM.

## Existing Rust empirical context

The separate existing `crates/system-info/` package was used only as supporting empirical Rust evidence. Its manifest directly depends on Serde and Serde JSON. Its exact offline test command passed 11 tests in 5.008 seconds. An offline release build took 4.707 seconds and produced a 620,512-byte executable. Both commands used target output under `/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01/`. It was not used as the ARC-02 Rust spike and did not determine this comparison.
