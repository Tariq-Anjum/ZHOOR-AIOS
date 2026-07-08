---
title: "HERMES-AIOS Project Overview"
created: 2026-07-07
owner: Tariq Anjum
description: "Master overview for the HERMES-AIOS build project — AI Operating System"
---

# HERMES-AIOS — Workspace Overview

**Repo:** https://github.com/Tariq-Anjum/HERMES-AIOS  
**Vault:** `~/Desktop/HERMES-AIOS` (this directory)  
**Kanban:** [[HERMES-AIOS-Kanban]]

## What It Is

An open-source AI Operating System — modular, local-first, security-first, provider-independent. Orchestrates multiple LLMs, tools, memory, and workflows through capability-based reasoning.

## Current Status

**Phase 0 — Documentation Baseline ✓**  
All core architecture docs written: Constitution, System Overview, Layered Architecture, Core Components.

**Phase 1 — Architecture (Complete) ✓**  
All 11 core component docs: 03B_1 through 03B_11 (Kernel, Router, Planner, Policy, Provider, Memory, Observability, Tool Runtime, Event Bus, Plugin Framework, Configuration System)

**Phase 2 — Infrastructure (Complete) ✓**  
8 docs: 04 through 04_7 (Infrastructure Overview, Deployment, Storage, Networking, Security, Backup, Monitoring, Scaling)

**Phase 3 — Implementation Planning (Complete) ✓**  
8 docs: 05 through 05_7 (Roadmap, Repository Structure, Dependency Graph, API Contracts, Database Schema, Plugin SDK, Testing Handbook, Contributor Guide)

**Phase 4 — Execution Architecture (In Progress)**  
8 of 9 docs complete: 06 through 06_7. **Next:** 06_8 Failure Recovery Runtime

## How This Workspace Integrates

This repo IS the Obsidian vault. The Kanban board lives at `docs/kanban/HERMES-AIOS-Kanban.md`.  
Hermes (this agent) drives the board — I update it as we build.

## Key Docs

- [[docs/constitution/HERMES_CONSTITUTION]] — Project constitution (highest authority)
- [[docs/architecture/SYSTEM_OVERVIEW/00_METADATA]] — Architecture metadata
- [[docs/architecture/SYSTEM_OVERVIEW/01_EXECUTIVE_SUMMARY]] — Vision & principles
- [[docs/architecture/SYSTEM_OVERVIEW/02_LAYERED_ARCHITECTURE]] — Layer design
- [[docs/architecture/SYSTEM_OVERVIEW/03_CORE_COMPONENTS]] — Component specs
- [[docs/kanban/HERMES-AIOS-Kanban]] — Project Kanban board

## Quick Links

- [GitHub Repo](https://github.com/Tariq-Anjum/HERMES-AIOS)
- [Kanban Board](docs/kanban/HERMES-AIOS-Kanban.md)
- ADR Index *(pending)*
