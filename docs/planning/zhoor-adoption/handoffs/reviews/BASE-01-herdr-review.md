# BASE-01 review by herdr-review

## Verdict

`CHANGES_REQUIRED` for the first candidate; focused re-review pending.

## Review ownership

- Output path: `docs/planning/zhoor-adoption/handoffs/reviews/BASE-01-herdr-review.md`
- Reviewer token: `herdr-review`
- Role: independent reviewer
- Runtime session: fresh Pi session `base01-review-luna` in Herdr pane `wG:p3`
- Model / harness: GPT-6 Luna / Pi via Herdr; observed provider token `codex`; launch arguments set `--thinking xhigh`. The reviewer could not introspect reasoning effort; xhigh is the controller-observed launch setting.
- Implementation and implementation handoff edited by reviewer: no

## Scope reviewed

- Exact candidate: `7d253e3feb6d326ac454aac38361f343c6b254cb`
- Detached checkout: `/tmp/zhoor-base01/ZHOOR-20260929-BASE01-SCAFFOLD-01/reviewer-luna`
- Reviewed BASE-01 implementation, handoff, accepted ADR-0003, toolchain contract, workflow, validation script, root workspace/lockfile, and existing `crates/system-info` workspace compatibility.
- Initial and final checkout status were clean. Candidate diff contained only expected BASE-01/control/handoff paths.

## Blocking findings

1. `[medium] .github/workflows/zhoor.yml push.paths and pull_request.paths` — the script conditionally runs the documented Python evaluation checks when `tools/eval/` exists, but neither workflow path filter included `tools/eval/**`. Future evaluation-only changes could therefore skip the canonical validation workflow. The controller confirmed this finding and added `tools/eval/**` to both filters; focused exact-SHA re-review is pending.

## Non-blocking findings

- The CI workflow uses `dtolnay/rust-toolchain@master`. Its explicit `toolchain: "1.98.1"` input matches ADR-0003, but the action reference itself is mutable. No network lookup or unapproved action pin change was made in BASE-01.

## Invariants checked

- [x] Root workspace uses `crates/*`, currently resolves `zhoor` and `zhoor-system-info`, and leaves the existing system-info crate unchanged.
- [x] Root package has no runtime dependencies; no future subsystem placeholders or scope violations were found in the candidate diff.
- [x] Rust 1.98.1 pin, `rustfmt` and `clippy` components, offline/locked Cargo validation, package mode, malformed argument rejection, smoke behavior, and lockfile stability were checked.
- [x] Reviewer did not manipulate the two `/tmp/opencode` paths. Exact creation provenance remains unknown as recorded in the implementation handoff.

## Validation independently run

| Command/check | Result |
|---|---|
| `pwd`; `git status --short --branch`; `git rev-parse HEAD`; clean-status assertion | PASS; exact candidate SHA and clean detached checkout |
| `git diff --name-status 9573708047dde640cd83c0edccc459ac5aa819e7..7d253e3feb6d326ac454aac38361f343c6b254cb` | PASS; 10 expected BASE-01/control/handoff paths |
| `git diff --check 9573708047dde640cd83c0edccc459ac5aa819e7..7d253e3feb6d326ac454aac38361f343c6b254cb` | PASS before and after validation |
| `sha256sum Cargo.lock` before and after locked validation | PASS; both `b64220e8b71db2463235b296335a01d117b376d4d653baea8872f6ce901b4c40` |
| `rustc --version`; `cargo --version` | PASS; rustc 1.98.1 (48a229cea 2026-09-01), cargo 1.98.1 (797e8a9bc 2026-08-05) |
| `cargo metadata --offline --locked --format-version 1` | PASS; workspace members exactly `zhoor` and `zhoor-system-info` |
| `cargo fmt --all -- --check && cargo check --workspace --all-targets --offline --locked && cargo test --workspace --offline --locked` | PASS; smoke 1/1, system-info 11/11 |
| `./scripts/validate.sh` | PASS; format, Clippy, check, tests/doctests, debug and release builds; Python branch skipped because `tools/eval/` is absent |
| `./scripts/validate.sh --package zhoor-system-info` | PASS; package lint/check/tests/builds |
| `./scripts/validate.sh --unknown`; `./scripts/validate.sh --package`; `./scripts/validate.sh --package nonexistent-pkg` | PASS; expected status 2, status 2, and propagated Cargo status 101 respectively |
| `./target/debug/zhoor` | PASS; exit 0 with empty output |
| Final `git status --short --branch`; `git status --porcelain` | PASS; detached checkout clean |

## Residual risk

- Focused re-review must confirm the new `tools/eval/**` entries in both workflow filters.
- The mutable `dtolnay/rust-toolchain@master` action reference remains a reproducibility risk even though the Rust toolchain input is pinned.
- Existing system-info compatibility was verified by workspace metadata and successful workspace checks/tests; no separate `build.rs` or `tests/` file was present to inspect.
- The two recorded orchestration deviations are not repository defects established by this review; prior contents/ownership of the removed scratch paths remain unknown.
