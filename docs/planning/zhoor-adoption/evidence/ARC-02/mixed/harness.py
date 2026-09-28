# NON-PRODUCTION ARC-02 SPIKE
"""Offline Python harness; forwards JSONL unchanged to the Rust deterministic core."""
import argparse
import subprocess
import sys

p = argparse.ArgumentParser()
p.add_argument("--core", required=True)
args = p.parse_args()
child = subprocess.Popen([args.core], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)
try:
    for line in sys.stdin:
        child.stdin.write(line)
        child.stdin.flush()
        result = child.stdout.readline()
        if not result:
            raise RuntimeError("Rust core exited before returning a response")
        sys.stdout.write(result)
        sys.stdout.flush()
finally:
    child.stdin.close()
    child.wait()
