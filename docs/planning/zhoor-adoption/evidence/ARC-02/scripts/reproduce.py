#!/usr/bin/env python3
# NON-PRODUCTION ARC-02 SPIKE reproduction and bounded comparison.
import json
import os
import statistics
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
EVIDENCE = HERE.parent
REPO = Path(__file__).resolve().parents[6]
RUN_ID = "ZHOOR-20260928-ARC02-TOOLCHAIN-01"
OUT = Path(os.environ.get("ARC02_OUTDIR", f"/tmp/zhoor-arc02/{RUN_ID}"))
TARGET = OUT / "build" / "rust-target"
ENV = dict(os.environ, CARGO_NET_OFFLINE="true", CARGO_TARGET_DIR=str(TARGET))
RUST_MANIFEST = EVIDENCE / "rust" / "Cargo.toml"
RUST_BIN = TARGET / "release" / "arc02-rust-spike"
SOCKET_BIN = OUT / "rust-socket-probe"
PYTHON = EVIDENCE / "python" / "runtime.py"
MIXED = EVIDENCE / "mixed" / "harness.py"
VALID = (EVIDENCE / "fixtures" / "valid.jsonl").read_text().splitlines()
INVALID = (EVIDENCE / "fixtures" / "invalid.jsonl").read_text().splitlines()


def run(args, **kwargs):
    return subprocess.run(args, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=ENV, **kwargs)


def pctl(xs, p):
    return statistics.quantiles(xs, n=100, method="inclusive")[p - 1] if len(xs) >= 2 else xs[0]


def status(pid):
    out = {}
    try:
        for line in Path(f"/proc/{pid}/status").read_text().splitlines():
            if line.startswith(("VmRSS:", "VmHWM:")):
                k, v = line.split(":", 1)
                out[k] = int(v.strip().split()[0])
    except (FileNotFoundError, ProcessLookupError):
        pass
    return out


def direct_children(pid):
    try:
        return [int(x) for x in Path(f"/proc/{pid}/task/{pid}/children").read_text().split()]
    except (FileNotFoundError, ProcessLookupError):
        return []


def command_set():
    return {
        "rust-first": [str(RUST_BIN)],
        "python-first": [sys.executable, str(PYTHON)],
        "bounded-mixed": [sys.executable, str(MIXED), "--core", str(RUST_BIN)],
    }


def verify():
    cases = [(line, "abstained" if json.loads(line)["candidates"] == [] else "resolved") for line in VALID]
    cases += [(line, "error") for line in INVALID]
    outputs = {}
    start = time.perf_counter()
    for name, cmd in command_set().items():
        p = run(cmd, input="\n".join(line for line, _ in cases) + "\n", cwd=REPO)
        if p.returncode != 0:
            raise RuntimeError(f"{name} exited {p.returncode}: {p.stderr}")
        rows = [json.loads(x) for x in p.stdout.splitlines()]
        if len(rows) != len(cases):
            raise RuntimeError(f"{name}: expected {len(cases)} responses, got {len(rows)}; stderr={p.stderr!r}")
        for i, (row, (_, expected)) in enumerate(zip(rows, cases)):
            if row["status"] != expected:
                raise RuntimeError(f"{name}: case {i} expected {expected}, got {row}")
        outputs[name] = rows
    if outputs["rust-first"] != outputs["python-first"] or outputs["rust-first"] != outputs["bounded-mixed"]:
        raise RuntimeError("option outputs differ for the same fixtures")
    return time.perf_counter() - start, len(VALID), len(INVALID)


def cold(command, sample, n=30):
    values = []
    payload = sample + "\n"
    for _ in range(n):
        t0 = time.perf_counter()
        p = run(command, input=payload, cwd=REPO)
        dt = (time.perf_counter() - t0) * 1000
        if p.returncode != 0 or not p.stdout.strip():
            raise RuntimeError(f"cold invocation failed: {command}: {p.stderr}")
        values.append(dt)
    return {"runs": n, "p50_ms": statistics.median(values), "p95_ms": pctl(values, 95)}


def warm(command, samples, n=120):
    p = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1, env=ENV, cwd=REPO)
    try:
        deadline = time.monotonic() + 10
        kids = []
        while time.monotonic() < deadline:
            kids = direct_children(p.pid)
            if kids or "harness.py" not in " ".join(command):
                break
            time.sleep(0.01)
        # Let the runtime finish module loading before sampling its idle resident set.
        time.sleep(0.05)
        idle_parent = status(p.pid)
        idle_child = status(kids[0]) if kids else {}
        latencies = []
        for i in range(n):
            line = samples[i % len(samples)]
            t0 = time.perf_counter()
            p.stdin.write(line + "\n")
            p.stdin.flush()
            result = p.stdout.readline()
            latencies.append((time.perf_counter() - t0) * 1000)
            if not result:
                raise RuntimeError(f"{command} exited during warm batch")
            json.loads(result)
        peak_parent = status(p.pid)
        peak_child = status(kids[0]) if kids else {}
        p.stdin.close()
        p.wait(timeout=10)
        if p.returncode != 0:
            raise RuntimeError(f"warm process failed: {p.stderr.read()}")
        return {
            "requests": n,
            "p50_ms": statistics.median(latencies),
            "p95_ms": pctl(latencies, 95),
            "throughput_req_s": 1000 * n / sum(latencies),
            "idle_parent_rss_kb": idle_parent.get("VmRSS"),
            "peak_parent_hwm_kb": peak_parent.get("VmHWM"),
            "idle_child_rss_kb": idle_child.get("VmRSS") if kids else None,
            "peak_child_hwm_kb": peak_child.get("VmHWM") if kids else None,
        }
    finally:
        if p.poll() is None:
            p.kill()
            p.wait()


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    t0 = time.perf_counter()
    build = run(["cargo", "build", "--offline", "--release", "--manifest-path", str(RUST_MANIFEST)], cwd=REPO)
    build_seconds = time.perf_counter() - t0
    if build.returncode != 0:
        raise RuntimeError(build.stdout + build.stderr)
    compile_probe = run(["rustc", "--edition=2024", str(EVIDENCE / "rust" / "socket_probe.rs"), "-o", str(SOCKET_BIN)], cwd=REPO)
    if compile_probe.returncode != 0:
        raise RuntimeError(compile_probe.stdout + compile_probe.stderr)
    py_socket = run([sys.executable, str(EVIDENCE / "python" / "socket_probe.py")], cwd=REPO)
    rust_socket = run([str(SOCKET_BIN)], cwd=REPO)
    if py_socket.returncode or rust_socket.returncode:
        raise RuntimeError(f"socket probe failure: Python={py_socket.stderr!r}; Rust={rust_socket.stderr!r}")
    test_seconds, valid_count, invalid_count = verify()
    commands = command_set()
    one = next(x for x in VALID if json.loads(x)["candidates"])
    cold_results = {name: cold(cmd, one, 30) for name, cmd in commands.items()}
    warm_results = {name: warm(cmd, VALID, 120) for name, cmd in commands.items()}
    cargo_tree = run(["cargo", "tree", "--offline", "--manifest-path", str(RUST_MANIFEST), "--prefix", "none"], cwd=REPO)
    result = {
        "host": {"os": "see environment.md", "target": "this workstation; one fresh-process 30-run sample and one warm 120-request batch"},
        "build_seconds": build_seconds,
        "test_seconds": test_seconds,
        "valid_fixtures": valid_count,
        "invalid_fixtures": invalid_count,
        "cold_start": cold_results,
        "warm_batch": warm_results,
        "socket_probe": {"python": py_socket.stdout.strip(), "rust": rust_socket.stdout.strip()},
        "rust_release_binary_bytes": RUST_BIN.stat().st_size,
        "rust_socket_probe_bytes": SOCKET_BIN.stat().st_size,
        "cargo_tree": cargo_tree.stdout.splitlines(),
        "measurement_notes": "perf_counter measures CLI/process and pipe overhead; /proc VmRSS/VmHWM sampled process values, not cgroup totals. Mixed RSS reports parent and child separately; do not sum as unique physical pages.",
    }
    path = OUT / "measurements.json"
    path.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
