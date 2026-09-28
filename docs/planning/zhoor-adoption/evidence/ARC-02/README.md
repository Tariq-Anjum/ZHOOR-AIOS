# ARC-02 evidence area

NON-PRODUCTION ARC-02 comparison only. The Rust-first and Python-first programs implement the same small JSONL selection workload with signed 64-bit integer scores. The bounded mixed option keeps deterministic validation and selection in Rust; its Python harness forwards JSONL over stdin/stdout and owns only test/developer tooling. No production language or package decision is made here.

Build artifacts, benchmark output, and disposable experiments belong under `/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01/`. No model, network call, OS mutation, privileged operation, or `aiosd` is part of the workload.

Reproduce from the repository root:

```sh
ARC02_OUTDIR=/tmp/zhoor-arc02/ZHOOR-20260928-ARC02-TOOLCHAIN-01 python3 docs/planning/zhoor-adoption/evidence/ARC-02/scripts/reproduce.py
```

The script builds Rust offline, compiles and runs local socket probes, verifies identical responses for the shared fixtures, measures 30 cold starts and 120 warm requests per option, and writes measurements only under `ARC02_OUTDIR`.
