---
title: Permit one rootless system-information capability slice
status: Accepted
accepted: 2026-09-28
deciders: Project owner via ZHOOR Execution Amendment v1.1 (2026-09-27)
amends: ADR-0001 decision 3 and the initial phase ordering only for this slice
---

# ADR-0002: Permit one rootless system-information capability slice

## Context

ADR-0001 sequences deterministic decision workers, protocol fixtures, and
replayable traces before capability work. The project owner has explicitly
selected one bounded milestone to validate the rootless execution path:
`system.info.read`, from a typed request through registry, policy, execution,
verification, and a compact ledger event.

## Decision

1. Permit exactly one local, rootless, read-only capability, `system.info.read`,
   as an isolated implementation slice before the decision-worker baseline.
2. Keep input validation, fixed registry lookup, deterministic policy, the
   provider, result verification, and ledger-event creation as distinct logical
   steps. The implementation is a local library/CLI; it is not `aiosd`.
3. The provider may read only `/proc/sys/kernel/osrelease` and compile-time OS
   and architecture identifiers. It accepts no paths or commands, invokes no
   shell, performs no mutation or network access, and denies execution for UID
   zero or unknown effective UID.
4. Return one versioned compact event without raw input or observed system
   values. Durable ledger storage and replay remain later work.
5. This exception does not authorize model or worker execution, other
   capabilities, privileged actions, consent/grant bypass, or canary promotion.
   `aiosd` remains the planned privileged execution boundary; its contract and
   existing promotion gates remain in force.
6. Rust in this isolated package does not settle the repository-wide
   language/toolchain decision.

## Consequences

The slice proves a narrowly scoped deterministic path without changing the
authority of models or broadening the capability allowlist. Future capability
or policy expansion still requires the existing architecture and promotion
gates.
