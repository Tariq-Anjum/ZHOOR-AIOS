# Errors, blockers, and approaches not to repeat

## Current blockers

- The audited checkout is detached. Do not begin parallel implementation until the owner creates a named integration branch/worktrees without losing this planning tranche.
- Language/toolchain is intentionally unresolved until `ARC-02`; do not infer Python from legacy docs or Rust from raw discussions.

## Rejected approaches

- Implementing the legacy HERMES roadmap from M1 through M9.
- Scaffolding a generic kernel, workflow engine, memory database, event bus, plugins, UI, and distributed runtime before the vertical slice.
- Installing Needle/Jev/OpenSpec/BMAD pre-emptively.
- Treating schema-valid model output, confidence, or model consensus as correctness.
- Giving `risk_judge`, an inference adapter, MCP client, or web stack authorization/execution authority.
- Repeatedly retrying failed installs/tests or weakening gates to make progress appear green.
- Letting every agent update the central work board or shared decision log.

## Historical issue resolved by this tranche

The earlier external ZHOOR plan reported that the HERMES-AIOS checkout could not be found. It is now present at `AI-tools/zhoor-repo-review`. Future work must use the repository-local architecture and adoption workspace rather than treating repository identity as unavailable.
