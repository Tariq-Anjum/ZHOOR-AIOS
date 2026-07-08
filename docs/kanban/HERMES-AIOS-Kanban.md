---

kanban-plugin: basic
created: 2026-07-07
updated: 2026-07-07
owner: Hermes AIOS Project

---

## 🏗️ Backlog

- [ ] **[SYS-001] Consolidate duplicate docs** — `SYSTEM_OVERVIEW.md` root duplicates subdir content. Decide single source or symlink.
- [ ] **[SYS-002] Migrate ChatGPT_Memory_and_Commitment → proper docs** — Conversation archives should move to `docs/decisions/` as ADRs.
- [ ] **[SYS-003] Write ARCHITECTURE.md** — Referenced in SYSTEM_OVERVIEW but missing from repo.
- [ ] **[SYS-004] Write ROADMAP.md** — Referenced but missing. Consolidate Executive_Summary's timeline.
- [ ] **[SYS-005] Create ADR-INDEX.md** — ADR series proposed but not created.
- [ ] **[SYS-006] .env.example** — Add template for API keys, DB paths, config.
- [ ] **[SYS-007] Expand README.md** — Currently 2 lines. Needs install, quickstart, architecture diagram.
- [ ] **[SYS-008] Python project scaffolding** — `pyproject.toml`, `src/hermes_aios/`, basic package structure.
- [ ] **[SYS-009] Editor config** — `.editorconfig`, `.vscode/settings.json`, `ruff.toml` or equivalent linter config.
- [ ] **[SYS-010] Pre-commit hooks** — lint, format, type-check before every commit.

## 🔨 In Progress

*No items in progress*

## ✅ Done (Phase 0 — Documentation)

- [x] **[DOC-001] Constitution (HC-001)** — docs/constitution/HERMES_CONSTITUTION.md v1.0.0 ✓
- [x] **[DOC-002] System Overview (SO-001)** — docs/architecture/SYSTEM_OVERVIEW/00_METADATA.md v1.0.0 ✓
- [x] **[DOC-003] Executive Summary** — docs/architecture/SYSTEM_OVERVIEW/01_EXECUTIVE_SUMMARY.md v1.0.0 ✓
- [x] **[DOC-004] Layered Architecture** — docs/architecture/SYSTEM_OVERVIEW/02_LAYERED_ARCHITECTURE.md v1.0.0 ✓
- [x] **[DOC-005] Core Components** — docs/architecture/SYSTEM_OVERVIEW/03_CORE_COMPONENTS.md v1.0.0 ✓
- [x] **[DOC-006] MIT License** — LICENSE ✓
- [x] **[DOC-007] Gitignore** — .gitignore ✓

## ✅ Phase 1 — Architecture Docs (Step 3B)

- [x] **[3B.1] Hermes Kernel** — Core orchestrator: intent → plan → execute loop ✓
- [x] **[3B.2] Capability Router** — Capability-based provider/model selection ✓
- [x] **[3B.3] Workflow Planner** — DAG-based execution graph planning ✓
- [x] **[3B.4] Policy Engine** — Security policy enforcement before execution ✓
- [x] **[3B.5] Provider Adapter** — Unified provider abstraction layer ✓
- [x] **[3B.6] Memory System** — 5-layer memory (working/episodic/semantic/procedural/long-term) ✓
- [x] **[3B.7] Observability** — Telemetry, tracing, metrics, cost tracking ✓
- [x] **[OBS-001] Obsidian vault + Kanban** — `.obsidian/` config + boards ✓

## ✅ Phase 1 — Architecture Docs (Complete)

- [x] **[3B.8] Tool Runtime** — Execution environment, sandboxing, lifecycle, permissions ✓
- [x] **[3B.9] Event Bus** — Inter-component pub/sub messaging ✓
- [x] **[3B.10] Plugin Framework** — Third-party plugin system with sandboxed isolation ✓
- [x] **[3B.11] Configuration System** — YAML/TOML config, env vars, secrets management ✓

## ✅ Phase 2 — Infrastructure Docs (Complete)

- [x] **[4.0] Infrastructure Overview** — Deployment architecture, runtime processes, storage layout ✓
- [x] **[4.1] Deployment Architecture** — Profiles, topology, startup sequence, health checks ✓
- [x] **[4.2] Storage Architecture** — SQLite, ChromaDB, object storage, cache, checkpoints ✓
- [x] **[4.3] Networking Architecture** — IPC, API gateway, security zones, provider connectivity ✓
- [x] **[4.4] Security Architecture** — Trust zones, auth, encryption, plugin security, audit ✓
- [x] **[4.5] Backup and Recovery** — Backup types, restore order, RPO/RTO, integrity verification ✓
- [x] **[4.6] Monitoring and Operations** — Health, alerts, dashboards, capacity, runbooks ✓
- [x] **[4.7] Scaling Strategy** — Vertical/horizontal scaling, storage/provider/event bus scaling ✓

## ✅ Phase 3 — Implementation Planning (Complete)

- [x] **[5.0] Implementation Roadmap** — Milestones, tech stack, CI/CD, risks ✓
- [x] **[5.1] Repository Structure** — Package layout, naming, dependency rules ✓
- [x] **[5.2] Module Dependency Graph** — Layered dependencies, initialization order, forbidden imports ✓
- [x] **[5.3] API Contracts** — Request/response models, versioning, streaming, error model ✓
- [x] **[5.4] Database Schema** — Entities, ChromaDB collections, migrations, indexing ✓
- [x] **[5.5] Plugin SDK** — Manifest, lifecycle, capability/tool registration, permissions ✓
- [x] **[5.6] Testing Handbook** — Test pyramid, coverage, CI gates, regression, release validation ✓
- [x] **[5.7] Contributor Guide** — Workflow, branching, ADRs, coding standards, governance ✓

## 📋 Phase 4 — Execution Architecture (In Progress)

- [x] **[6.0] Execution Architecture** — Request lifecycle, runtime components, state machine ✓
- [x] **[6.1] Kernel Execution Lifecycle** — Startup, supervision, shutdown, recovery ✓
- [x] **[6.2] Workflow Execution Engine** — DAG, task states, scheduling, checkpointing ✓
- [x] **[6.3] Capability Routing Runtime** — Intent, complexity, provider scoring, local-first ✓
- [x] **[6.4] Memory Execution Pipeline** — Retrieval, ranking, context assembly, embeddings ✓
- [x] **[6.5] Tool Execution Lifecycle** — Discovery, sandbox, streaming, audit, cleanup ✓
- [x] **[6.6] Provider Execution Pipeline** — Discovery, health, failover, normalization, telemetry ✓
- [x] **[6.7] Event Processing Runtime** — Lifecycle, persistence, replay, backpressure, dead-letter ✓
- [ ] **[6.8] Failure Recovery Runtime** — Failure classification, retry, checkpoint restoration, circuit breakers

## 🧪 Review

*No items in review*

## 📋 Phase 1 — Foundation (Now)

- [ ] **[P1-001] Project Scaffolding** — `pyproject.toml`, `src/hermes_aios/__init__.py`, `setup.cfg`, `.env.example`
- [ ] **[P1-002] CLI Application** — `argparse`/`click`-based CLI, `hermes-aios` command entry point
- [ ] **[P1-003] Agent Kernel Loop** — Core orchestrator: read goal → call LLM → parse response → execute → loop
- [ ] **[P1-004] Basic Memory (SQLite)** — Session persistence, conversation history store
- [ ] **[P1-005] LLM Connector** — Common interface, OpenAI provider first, pluggable adapter pattern
- [ ] **[P1-006] Structured Logging** — Python `logging` module, per-session log files, JSON structured output
- [ ] **[P1-007] CI/CD (GitHub Actions)** — Lint + test on push/PR, `setup-python` for 3.14
- [ ] **[P1-008] Unit Tests** — `pytest` for core: LLM response parser, config, CLI commands
- [ ] **[P1-009] Security (.env)** — `python-dotenv`, secret isolation, `.env.example` template
- [ ] **[P1-010] README + Quickstart** — Install, configure, run first goal. Architecture diagram in README

## 🔭 Phase 2 — Enhancement (Near-term)

- [ ] **[P2-001] Embedding Memory** — Add vector embeddings to SQLite memory for semantic recall
- [ ] **[P2-002] Multi-Provider Support** — Ollama, OpenRouter, Gemini adapters behind common interface
- [ ] **[P2-003] Tool Registry** — Register, discover, and execute tools via capability routing
- [ ] **[P2-004] Workflow Planner** — DAG-based execution graph for complex multi-step tasks
- [ ] **[P2-005] Comprehensive Tests** — Integration tests with mock LLM, 80%+ coverage target
- [ ] **[P2-006] Enhanced Logging** — Log aggregation, structured JSON, configurable verbosity
- [ ] **[P2-007] Full ADR Series** — ADR-0001 through ADR-0006, formally approved and merged

## 🚀 Phase 3 — Expansion (Later)

- [ ] **[P3-001] Web UI** — Dashboard (Streamlit or lightweight framework)
- [ ] **[P3-002] Vector DB (ChromaDB)** — Replace basic vector store with ChromaDB for semantic memory
- [ ] **[P3-003] Multi-Agent Bus** — Inter-agent pub/sub message bus (Redis Streams or HTTP+SQLite)
- [ ] **[P3-004] Docker Deployment** — Dockerfile + docker-compose for single-node deployment
- [ ] **[P3-005] Policy Engine** — Explicit policy enforcement before every privileged action
- [ ] **[P3-006] Plugin Registry** — Third-party plugin system with sandboxed execution
- [ ] **[P3-007] E2E Tests** — Full pipeline tests with mock LLM orchestrations

## ☁️ Phase 4 — Distribution (Future)

- [ ] **[P4-001] Distributed Execution** — Multi-node orchestration, workload distribution
- [ ] **[P4-002] Cloud Deployment** — Terraform/Pulumi for cloud provisioning, autoscaling
- [ ] **[P4-003] Enterprise Auth** — OAuth, SSO, Vault for secrets management
- [ ] **[P4-004] Federated Memory** — Cross-instance memory synchronization
- [ ] **[P4-005] Observability Stack** — Metrics (Prometheus), tracing (OpenTelemetry), alerting
- [ ] **[P4-006] DevContainer** — VS Code Dev Container / GitHub Codespaces for zero-setup contribution

---

**Labels:** `priority:high` `priority:medium` `priority:low` `area:docs` `area:core` `area:infra` `area:testing` `area:ui`

**Legend:**
- `[ ]` — Not started
- `[-]` — Blocked / on hold  
- `[/]` — In progress
- `[x]` — Done

