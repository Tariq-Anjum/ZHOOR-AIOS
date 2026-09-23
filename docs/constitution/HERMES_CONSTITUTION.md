---
document_id: HC-001
last_updated: 2026-09-23
owner: Hermes AIOS Project
review_cycle: Quarterly
status: Approved
title: HERMES-AIOS / ZHOOR Transition Constitution
version: 1.1.0
---

# HERMES-AIOS / ZHOOR Transition Constitution

## 1. Purpose

This Constitution governs the HERMES-AIOS repository during its transition to ZHOOR. ZHOOR is the accepted product and architectural direction; the repository name will change later. All code, documentation, architecture decisions, and contributions must conform to this Constitution and accepted ADRs.

## 2. Vision

Build a secure, local-first decision fabric that helps agents and AIOS clients select bounded actions, evidence, models, skills, and escalation paths without giving probabilistic model output execution authority.

## 3. Mission

Create an open, provider-independent system that uses the cheapest reliable deterministic mechanism first, escalates only with recorded evidence, and separates recommendation, authorization, execution, memory, and evaluation.

## 4. Core Principles

1. Local-first whenever practical.
2. Security before convenience.
3. Deterministic rules and indexes before semantic models.
4. Policy and evidence above model confidence.
5. Architecture and executable contracts before implementation.
6. Interfaces before concrete providers.
7. Minimum context, tokens, code, and model for maximum useful work.
8. Modular, replaceable components and backends.
9. Traces, evaluation, abstention, and replay are part of the architecture.
10. Documentation and tests are part of the product.
11. Open standards over vendor lock-in.
12. Autonomous capability is earned per risk class; it is never implied by model quality.

## 5. Architecture Principles

- One typed Decision Runtime for protocol validation, budgets, routing, scheduling, calibration, cache, and trace capture.
- Logical workers implemented as bounded schemas/configuration over shared replaceable backends.
- Initial workers limited to deterministic `skill_router` and `risk_judge`.
- Cost-ordered escalation: rule, index, tiny local semantic backend, stronger approved backend, reasoning model, human.
- A deterministic Policy Engine separate from inference and execution.
- `aiosd` as the sole planned privileged execution boundary.
- Evidence provenance and trust tiers independent from model confidence.
- Versioned contracts and replayable traces before model or capability expansion.

## 6. Security Principles

- Recommendation, authorization, and execution are separate trust boundaries.
- A model never directly executes a capability or grants permission.
- Policy denies or escalates by default when identity, evidence, scope, or provenance is insufficient.
- `aiosd` authenticates callers, validates short-lived operation-bound grants and normalized arguments, executes only allowlisted capabilities, and emits receipts.
- Least privilege applies to every capability.
- Every privileged action is auditable and independently revalidated.
- Secrets, unrestricted transcripts, and hidden model reasoning are excluded from traces.
- Prompt text and external content cannot override policy or raise evidence trust.

## 7. Memory Principles

- `my-ai-brain` is the canonical durable Markdown memory authority.
- ZHOOR stores only bounded active session state and sanitized trace/evaluation data.
- OB1 may act as an optional governed mirror/evidence substrate, not canonical authority.
- Retrieval and storage backends remain replaceable; a vector database is not a foundation requirement.

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

Build in this order: 1. Governance and toolchain decision 2. Versioned protocol fixtures 3. Deterministic registry, policy, and traces 4. `skill_router` and `risk_judge` 5. Hand-labeled evaluation and audit 6. Optional backend benchmark 7. Shadow operation 8. Non-mutating `aiosd` introspection 9. Human-approved reversible canary proposal.

No model backend, workflow engine, plugin system, UI, canonical memory database, or mutating capability may precede the deterministic evaluated core.

## 12. Amendment Process

Changes to this Constitution require: - documented rationale, - ADR if
architectural, - repository review, - version increment.

------------------------------------------------------------------------

Amended for the HERMES-AIOS to ZHOOR transition under ADR-0001.
