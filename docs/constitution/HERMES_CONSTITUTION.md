---
document_id: HC-001
last_updated: 2026-07-05
owner: Hermes AIOS Project
review_cycle: Quarterly
status: Approved
title: Hermes Constitution
version: 1.0.0
---

# Hermes AIOS Constitution

## 1. Purpose

This Constitution is the highest governing document for Hermes AIOS. All
code, documentation, architecture decisions, and future contributions
must conform to it.

## 2. Vision

Build a modular, secure, local-first AI Operating System that can
orchestrate multiple models and tools through capability-based
reasoning.

## 3. Mission

Create an open-source AI platform that is privacy-first, extensible,
provider independent, and suitable for personal and enterprise
deployment.

## 4. Core Principles

1.  Local-first whenever practical.
2.  Security before convenience.
3.  Architecture before implementation.
4.  Interfaces before concrete providers.
5.  Capability-based orchestration.
6.  Multi-agent by architecture; single orchestrator initially.
7.  Modular, replaceable components.
8.  Documentation is part of the product.
9.  Testing is mandatory.
10. Open standards over vendor lock-in.

## 5. Architecture Principles

-   Stable kernel.
-   Provider abstraction layer.
-   Policy engine.
-   Capability router.
-   Workflow planner.
-   Intent classifier.
-   Memory abstraction.
-   Plugin registry.
-   Event bus.
-   Configuration system.

## 6. Security Principles

-   Hardened Command Parser is the trust boundary.
-   Least privilege for every tool.
-   Explicit policy enforcement.
-   Audit every privileged action.
-   Never expose secrets.
-   Validate all model outputs before execution.

## 7. Memory Principles

-   SQLite is the authoritative structured memory.
-   ChromaDB or equivalent vector store may augment semantic retrieval.
-   Memory backends must remain interchangeable.

## 8. Provider Principles

No provider-specific logic outside provider adapters. Supported
providers should include Ollama, OpenAI, Gemini, GLM, OpenRouter and
future adapters.

## 9. Engineering Principles

-   Small commits.
-   One logical change per commit.
-   CI must remain green.
-   Public APIs documented.
-   Type hints preferred.
-   Automated formatting and tests required.

## 10. Documentation Principles

Every major architectural decision must receive an ADR. Repository
documentation is the canonical source of truth.

## 11. Roadmap Principles

Build in this order: 1. Foundation 2. Kernel 3. Memory 4. Routing 5.
Tools 6. Agents 7. Optimization 8. Distributed execution

Infrastructure should never precede a stable core.

## 12. Amendment Process

Changes to this Constitution require: - documented rationale, - ADR if
architectural, - repository review, - version increment.

------------------------------------------------------------------------

Approved for Hermes AIOS Phase 1.
