# BASE-01 — Minimal project scaffold


**Depends on:** ARC-03  
**Recommended:** GLM 5.3 Flash  
**Review:** GPT-5.6 Luna  
**Timebox:** one implementation session

## Objective

Create only the toolchain, package, test, and CI skeleton required to run an empty deterministic ZHOOR library/CLI according to the accepted toolchain ADR.

## Required context and skills

- Load `skill-router` and `karpathy-guidelines`.
- Read the ARC-03-accepted toolchain ADR, `docs/development/ZHOOR_TOOLCHAIN.md`, `TARGET_SCAFFOLD.md`, and `AGENT_OPERATING_RULES.md`.
- Karpathy application: do not create future subsystem directories or add optional dependencies.

## Write scope

- Root manifests/lock/config files selected by ARC-02.
- Minimal source package identity and version only.
- Minimal smoke test.
- One CI workflow for canonical checks.
- Root `.gitignore` additions strictly required by toolchain.
- `docs/planning/zhoor-adoption/handoffs/BASE-01.md`.

## Acceptance

- Clean install/build from declared lock state.
- Format, lint, type/static check, unit test, and package/build commands pass.
- No provider, model, database, web, plugin, workflow, or execution dependencies.
- CI runs the same canonical commands.

## Stop

After two failures of the same setup/check, record evidence and stop. Do not change the ARC-03 decision, switch toolchains, or add workaround frameworks.
