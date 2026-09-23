# POL-01 — Deterministic risk policy


**Depends on:** PRO-01  
**Recommended:** GPT-5.6 Luna  
**Review:** Qwen3.8 Max  
**Timebox:** one bounded policy session

## Objective

Define and implement the v1 deterministic risk taxonomy and hard rules consumed by `risk_judge`. This produces scrutiny/escalation recommendations and evidence requirements—not authoritative denial, authorization, or grant issuance.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read ZHOOR architecture sections 4, 8, and 9; protocol interfaces; legacy policy principles only where cited by the audit.
- Karpathy application: state risk assumptions, keep taxonomy small, and do not design future mutable capabilities.

## Write scope

- Policy/risk rule package/module, versioned risk-policy fixtures/tests, and `docs/planning/zhoor-adoption/handoffs/POL-01.md`.
- ARC-03 must replace this description with exact path globs and validation commands before `READY`.

## Acceptance

- Covers unknown capability, shell, package install, service control, network egress, path/resource scope, privilege, mutation, missing evidence, and prompt-injection indicators.
- Semantic ambiguity can only maintain or increase scrutiny, never decrease hard-rule risk.
- Unknowns fail closed to abstention or an advisory escalation recommendation; authoritative `denied` dispositions remain a separate Policy Engine authorization path.
- Rules emit evidence/reason codes and policy version.
- No grant issuance, secret resolution, execution, or model dependency.
- Targeted and full checks pass.

## Stop

Any proposed allow rule for mutation or privilege is out of scope. After two rule/schema conflicts, stop and request architecture review.
