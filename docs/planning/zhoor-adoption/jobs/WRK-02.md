# WRK-02 — Deterministic risk judge


**Depends on:** POL-01, TRC-01  
**Recommended:** GLM 5.3 Flash  
**Review:** Qwen3.8 Max  
**Timebox:** one implementation session

## Objective

Implement the Level 0 `risk_judge`: apply hard risk rules to a typed request, produce a bounded non-authoritative score with reasons, evidence requirements, and scrutiny/escalation recommendation, and emit a trace. It cannot authorize or execute.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read ZHOOR architecture sections 3–5, 8, and 9 plus accepted protocol/policy/trace interfaces.
- Karpathy application: do not generalize a policy engine or add a model classifier.

## Write scope

- `crates/risk-judge/**` — `zhoor-risk-judge` deterministic non-authoritative worker, tests, and package-local docs.
- `fixtures/risk-judge/**`.
- `docs/planning/zhoor-adoption/handoffs/WRK-02.md`.
- Policy, protocol, and trace paths are read-only dependencies. No Python/model/backend implementation or authoritative grant/deny behavior.


## Path ownership and validation

Targeted: `./scripts/validate.sh --package zhoor-risk-judge` and `cargo test --offline --locked -p zhoor-risk-judge --test risk_judge_contract`.

Full: `./scripts/validate.sh`.

Lockfile timing: component workers do not edit or commit the root `Cargo.lock`. Before integration, run the package checks from `docs/development/ZHOOR_TOOLCHAIN.md` without `--locked` (keep `--offline`) and report any generated lockfile diff without staging it. After integrating the package source/manifests, the controller reconciles and reviews the single root lockfile update, then runs the locked targeted and full commands above on the integration candidate. This card is complete only after those post-integration checks pass.

## Acceptance

- Hard risk rules are deterministic and versioned.
- Unknown/malformed/insufficient-evidence inputs abstain or recommend escalation explicitly; authoritative denial remains Policy Engine behavior.
- Risk output can only inform scrutiny and a later Policy Engine deny/escalate decision; it cannot itself deny, authorize, emit grants, or create executable requests.
- Adversarial text cannot override policy fields or trust tiers.
- Emits sanitized traces for all outcomes.
- No network/model/tool execution dependency.
- Targeted and full checks pass.

## Stop

The global two-attempt rule applies. Any request to authorize or deny a capability authoritatively is an architecture violation. Record it and stop rather than extending scope.
