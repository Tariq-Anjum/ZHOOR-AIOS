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

- [ ] **[OBS-001] Obsidian vault** — `.obsidian/` config + Kanban board. First working review board.

## ✅ Done (Phase 0 — Documentation)

- [x] **[DOC-001] Constitution (HC-001)** — docs/constitution/HERMES_CONSTITUTION.md v1.0.0 ✓
- [x] **[DOC-002] System Overview (SO-001)** — docs/architecture/SYSTEM_OVERVIEW/00_METADATA.md v1.0.0 ✓
- [x] **[DOC-003] Executive Summary** — docs/architecture/SYSTEM_OVERVIEW/01_EXECUTIVE_SUMMARY.md v1.0.0 ✓
- [x] **[DOC-004] Layered Architecture** — docs/architecture/SYSTEM_OVERVIEW/02_LAYERED_ARCHITECTURE.md v1.0.0 ✓
- [x] **[DOC-005] Core Components** — docs/architecture/SYSTEM_OVERVIEW/03_CORE_COMPONENTS.md v1.0.0 ✓
- [x] **[DOC-006] MIT License** — LICENSE ✓
- [x] **[DOC-007] Gitignore** — .gitignore ✓

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

