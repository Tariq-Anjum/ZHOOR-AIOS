# ARC-02 — Language and toolchain evidence spike


**Depends on:** ARC-01  
**Recommended:** Qwen3.8 Max  
**Review:** GPT-5.6 Sol in ARC-03  
**Timebox:** one evidence-producing spike session; no decision or product implementation

## Objective

Produce reproducible evidence comparing one minimal Rust spike, one minimal Python spike, and a bounded mixed option for the Decision Runtime/deterministic slice. Do not accept the architecture decision in this card.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read ADR-0001, ZHOOR architecture sections 3–8 and 10–12, `TARGET_SCAFFOLD.md`, `ARCHITECTURE_AUDIT.md`, and only the legacy development/toolchain docs cited by the audit.
- Karpathy application: state hardware/toolchain assumptions; choose the smallest sufficient stack; do not scaffold implementation.

## Write scope

- Isolated spike/evidence files under an ARC-03-pinned evaluation area; no production package scaffold.
- `docs/planning/zhoor-adoption/handoffs/ARC-02.md` with commands, measurements, dependency inventory, and comparison table.

## Required comparison

Evaluate at least: Rust-first runtime, Python-first runtime, and a deliberately bounded mixed option. Compare type/schema safety, Unix-socket/peer-credential support, packaging, test/eval ergonomics, dependency/security burden, cold-start/resource behavior, agent competency, and risk of duplicated logic.

## Success condition

All three options have reproducible minimal evidence and a concise recommendation. No option is accepted, no production layout is scaffolded, and no model/backend package is introduced.

## Stop

If a required compiler/runtime is unavailable or two spike attempts fail, do not install broad toolchains without approval. Record commands/errors for ARC-03. Architecture acceptance is out of scope.
