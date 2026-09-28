# ZHOOR AIOS

This is the canonical repository for **ZHOOR AIOS**, a local-first decision fabric for agents and AIOS clients. The former HERMES-AIOS repository was renamed in place to ZHOOR-AIOS.

> Minimum context. Minimum tokens. Minimum code. Minimum model. Maximum useful work.

## Current status

- ZHOOR is the accepted architectural direction.
- The repository currently contains architecture and planning documents; product implementation has not started.
- The first release is deliberately narrow: typed decision contracts, deterministic `skill_router` and `risk_judge`, sanitized traces, and offline evaluation.
- No initial worker may execute tools or OS capabilities.
- Jev is not a dependency. Needle or another semantic backend is considered only after deterministic evaluation gates pass.

## Start here

- **Architecture:** [`docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md`](docs/architecture/ZHOOR_ARCHITECTURE-v0.2.md)
- **Adoption decision:** [`docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md`](docs/architecture/adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md)
- **Agent plan and job board:** [`docs/planning/zhoor-adoption/README.md`](docs/planning/zhoor-adoption/README.md)
- **Constitution:** [`docs/constitution/HERMES_CONSTITUTION.md`](docs/constitution/HERMES_CONSTITUTION.md)

All earlier HERMES architecture, execution, infrastructure, and implementation documents remain available as historical/reference material. Where they conflict with the accepted ADR or ZHOOR architecture, they are non-normative.

## Architecture rule

Models may recommend, classify, extract, or score. Deterministic policy controls authorization, and the separate `aiosd` boundary controls any future allowlisted execution. Confidence is never permission.

## Contributing

Do not start from the legacy Kanban. Select only an owner-assigned `READY` card in `docs/planning/zhoor-adoption/jobs/`, use an isolated worktree for concurrent work, obey the card's write scope, run its validation, and leave a handoff. Do not commit or push unless the owner explicitly assigns that operation.
