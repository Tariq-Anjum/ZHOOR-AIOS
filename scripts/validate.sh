#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

usage() {
  echo "usage: ./scripts/validate.sh [--package <workspace-package>]" >&2
  exit 2
}

if [ "$#" -eq 0 ]; then
  mode=full
elif [ "$#" -eq 2 ] && [ "$1" = "--package" ]; then
  case "$2" in
    ''|-*) usage ;;
    *)
      mode=package
      package="$2"
      ;;
  esac
else
  usage
fi

run() {
  echo "+ $*"
  "$@"
}

if [ "$mode" = full ]; then
  run cargo fmt --all -- --check
  run cargo clippy --workspace --all-targets --offline --locked -- -D warnings
  run cargo check --workspace --all-targets --offline --locked
  run cargo test --workspace --lib --offline --locked
  run cargo test --workspace --tests --offline --locked
  run cargo test --workspace --offline --locked
  run cargo build --workspace --offline --locked
  run cargo build --workspace --release --offline --locked
  if [ -d tools/eval ]; then
    run python3.14 -c 'import ast, pathlib; [ast.parse(p.read_text(), filename=str(p)) for p in pathlib.Path("tools/eval").rglob("*.py")]'
    run env PYTHONDONTWRITEBYTECODE=1 python3.14 -m unittest discover -s tools/eval/tests -v
  fi
else
  run cargo fmt --all -- --check
  run cargo clippy --package "$package" --all-targets --offline --locked -- -D warnings
  run cargo check --package "$package" --all-targets --offline --locked
  run cargo test --package "$package" --offline --locked
  run cargo build --package "$package" --offline --locked
  run cargo build --package "$package" --release --offline --locked
fi