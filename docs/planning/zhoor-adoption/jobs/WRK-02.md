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

- `risk_judge` worker package/module.
- Worker-specific fixtures/tests/docs and `docs/planning/zhoor-adoption/handoffs/WRK-02.md`.
- ARC-03 must replace this description with exact path globs and validation commands before `READY`.

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
