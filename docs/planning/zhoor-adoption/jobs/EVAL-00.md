# EVAL-00 — Deterministic evaluation harness


**Depends on:** WRK-01, WRK-02  
**Recommended:** DeepSeek V4.1 Flash  
**Review:** GPT-5.6 Luna  
**Timebox:** one implementation session

## Objective

Build a reusable offline evaluation harness and versioned case/report contract for both initial workers without creating the labeled datasets themselves.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read ZHOOR architecture section 10 plus accepted worker/protocol/trace public interfaces.
- Karpathy application: implement only metrics required by the two deterministic workers; do not add model judging or an experiment platform.

## Write scope

- Evaluation harness module/tool.
- Dataset case/provenance and report schemas.
- Harness unit/integration tests and small synthetic smoke fixtures only.
- Evaluation docs and `docs/planning/zhoor-adoption/handoffs/EVAL-00.md`.
- ARC-03 must replace this description with exact path globs and validation commands before `READY`.

## Acceptance

- Runs versioned cases against either worker and verifies expected result/disposition/reason codes.
- Validates label source, reviewer, registry/policy version, and provenance.
- Supports schema validity, correctness/error rates, abstention metrics, latency, and regression diffs.
- Supports worker-specific metrics without embedding policy in the harness.
- Repeated runs are deterministic; traces link to case IDs without raw secret/transcript storage.
- Rejects unlabeled or model-only-ground-truth cases from promotion reports.
- Targeted and full checks pass.

## Stop

Do not generate production datasets or add semantic evaluation/model judges. After two worker-interface failures, stop and document the mismatch rather than changing public contracts unilaterally.
