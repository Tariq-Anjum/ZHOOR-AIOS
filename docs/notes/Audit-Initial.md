---
title: "Initial Repo Audit — Discrepancies & Flaws"
created: 2026-07-07
auditor: Hermes Agent
status: Open
---

# HERMES-AIOS Repo Audit

Audit date: 2026-07-07  
Repo state: Documentation-only, Phase 0

## Critical

| # | Issue | Severity | Kanban Card |
|---|-------|----------|-------------|
| 1 | **No code exists.** Docs describe a full system but `src/` is empty. After 4 architecture docs and 3 ChatGPT discussion docs, there's zero runnable code. | 🔴 blocking | P1-001 |
| 2 | **README is 2 lines.** A 30+ doc project with a 33-byte README describing nothing. | 🔴 blocking | SYS-007 |
| 3 | **ChatGPT_Memory_and_Commitment/ is mixed into the repo.** These are AI conversation transcripts, not canonical docs. They contain draft proposals mixed with discussion. Belongs in `docs/decisions/` with proper frontmatter, or in `.gitignore`'d scratch. | 🔴 structural | SYS-002 |

## High

| # | Issue | Severity | Kanban Card |
|---|-------|----------|-------------|
| 4 | **ARCHITECTURE.md referenced but missing.** SYSTEM_OVERVIEW.md and ADR.md both reference it. Does not exist. | 🟠 missing | SYS-003 |
| 5 | **ROADMAP.md referenced but missing.** Executive_Summary.md has an 8-week timeline — should be formalized. | 🟠 missing | SYS-004 |
| 6 | **ADR series proposed but empty.** ADR.md lists ADR-0001 through ADR-0006. None created. `docs/architecture/adr/` doesn't exist. | 🟠 missing | SYS-005 |
| 7 | **No `.env.example`.** Project uses API keys — needs a template. | 🟠 missing | SYS-006 |
| 8 | **Root SYSTEM_OVERVIEW.md vs child docs conflict.** `docs/architecture/SYSTEM_OVERVIEW.md` is a summary, while `docs/architecture/SYSTEM_OVERVIEW/` subdir has full detail — same document ID prefix, split across files. Should be one source of truth. | 🟠 structural | SYS-001 |

## Medium

| # | Issue | Severity | Kanban Card |
|---|-------|----------|-------------|
| 9 | **No packaging.** No `pyproject.toml`, no `requirements.txt`, no `setup.cfg`. Can't install, can't run. | 🟡 missing | P1-001 |
| 10 | **No CI/CD.** No `.github/workflows/`. No automated checks at all. | 🟡 missing | P1-007 |
| 11 | **No editor config.** No `.editorconfig`, no `ruff.toml`, no `pyrightconfig.json`. Formatting is undefined. | 🟡 missing | SYS-009 |
| 12 | **No test framework.** No `pytest` config, no test directory, no tests. | 🟡 missing | P1-008 |
| 13 | **No pre-commit hooks.** No lint/format gates before commits. | 🟡 missing | SYS-010 |
| 14 | **`.gitignore` only blocks `*.Zone.Identifier`.** No Python/IDE/OS ignores. Not updated yet - this file will be updated in this PR. | 🟡 weak | *(fixed)* |

## Low

| # | Issue | Severity | Kanban Card |
|---|-------|----------|-------------|
| 15 | **`last_updated` fields in docs are stale.** All show 2026-07-06. | ⚪ stale | — |
| 16 | **Executive_Summary.md duplicates vision/philosophy** already in 01_EXECUTIVE_SUMMARY.md. Different authorship but overlapping. | ⚪ duplicate | SYS-002 |
| 17 | **No architecture diagrams as standalone files.** Mermaid is embedded but there's no `docs/assets/` with rendered/exported diagrams for non-Obsidian readers. | ⚪ nice-to-have | — |

## Summary

**The docs are thorough and well-structured. The project knows what it wants to be. But it has zero code, missing structural docs, and stale conversation artifacts.** Phase 1 should start with cleaning this up — consolidate the docs, scaffold the Python project, then build the kernel loop.
