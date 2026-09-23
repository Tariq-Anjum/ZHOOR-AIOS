---
document_id: SO-001
last_updated: 2026-07-06
owner: Hermes AIOS Project
status: Draft
title: System Overview
version: 1.0.0
---

# Hermes AIOS --- System Overview

> **Legacy reference:** ZHOOR is now the accepted architectural direction. Use `ZHOOR_ARCHITECTURE-v0.2.md`, `adr/ADR-0001-ADOPT-ZHOOR-DIRECTION.md`, and `../planning/zhoor-adoption/README.md` for current work. This document is non-normative where it conflicts with them.
>
> This Version 1.0 overview records the former HERMES-AIOS baseline.

## Executive Summary

Hermes AIOS is a modular, local-first AI Operating System designed to
orchestrate multiple language models, tools, memories, and workflows
through capability-based reasoning rather than provider-specific logic.

The architecture is built around a stable kernel with interchangeable
providers, secure execution, memory abstraction, and extensible plugins.

## Vision

Build an open, secure, provider-independent AI Operating System that
remains useful whether running entirely offline or connected to cloud
providers.

## Core Principles

-   Local-first
-   Security-first
-   Capability-based orchestration
-   Provider abstraction
-   Modular architecture
-   Documentation-driven development
-   Repository as the canonical source of truth

## High-Level Architecture

``` text
User
 │
Interfaces
 │
Hermes Kernel
 ├── Intent Classifier
 ├── Complexity Estimator
 ├── Workflow Planner
 ├── Capability Router
 ├── Policy Engine
 ├── Memory Manager
 ├── Tool Registry
 ├── Plugin Registry
 └── Execution Engine
 │
Provider Layer
 ├── Ollama
 ├── OpenAI
 ├── OpenRouter
 ├── Gemini
 ├── GLM
 └── Future Providers
```

## Request Lifecycle

1.  Receive request
2.  Classify intent
3.  Estimate complexity
4.  Determine required capabilities
5.  Build execution workflow
6.  Select model(s)
7.  Validate through policy engine
8.  Execute tools
9.  Update memory
10. Return response

## Kernel Components

### Intent Classifier

Determines user intent before any model execution.

### Complexity Estimator

Predicts execution cost and determines whether escalation is required.

### Workflow Planner

Produces an execution graph.

### Capability Router

Routes by capability requirements instead of model names.

### Policy Engine

Validates every privileged action.

### Memory Manager

Coordinates short-term, long-term and semantic memory.

### Execution Engine

Runs approved workflows.

## Memory

Authoritative memory: - SQLite

Semantic memory (future): - ChromaDB

Future memory backends must remain interchangeable.

## Security

The Hardened Command Parser forms the trust boundary.

Principles: - Least privilege - Tool allowlists - Prompt injection
resistance - Audit logging - Secret isolation

## Provider Model

Providers are implementation details behind a common interface.

Hermes logic never depends on provider-specific SDKs.

## Multi-Agent Strategy

Architecture: - Multi-agent

Initial implementation: - Single orchestrator

## Roadmap Alignment

Phase 1: - Documentation - Foundation - Provider abstraction -
Configuration - Security

Phase 2: - Kernel - Routing - Memory - Tool execution

Phase 3: - Semantic memory - Observability - Multi-agent execution

Phase 4: - Distributed execution - Cloud deployment - Enterprise
capabilities

## Related Documents

-   HC-001 Hermes Constitution
-   ARCH-001 Architecture Specification (planned)
-   ADR series (planned)

## Revision History

  Version   Date         Notes
  --------- ------------ -------------------------------
  1.0.0     2026-07-06   Initial architecture baseline
