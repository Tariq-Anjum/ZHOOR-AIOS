# ARC-02 — Language and toolchain evidence spike


**Depends on:** ARC-01
**Recommended:** implementation-low-cost
**Review:** independent-challenger in a separate session after evidence completion and before ARC-02 is DONE
**Timebox:** one evidence-producing spike session; no decision or product implementation

## Objective

Produce reproducible evidence comparing one minimal Rust spike, one minimal Python spike, and a bounded mixed option for the Decision Runtime/deterministic slice. Do not accept the architecture decision in this card.

## Required context and skills

- Load `skill-router`, `karpathy-guidelines`, and `file-based-planning`.
- Read ADR-0001, ZHOOR architecture sections 3–8 and 10–12, `TARGET_SCAFFOLD.md`, `ARCHITECTURE_AUDIT.md`, and only the legacy development/toolchain docs cited by the audit.
- Karpathy application: state hardware/toolchain assumptions; choose the smallest sufficient stack; do not scaffold implementation.

## Write scope

- Committed reproducibility evidence under `docs/planning/zhoor-adoption/evidence/ARC-02/` (available before ARC-03); mark any minimal source used for comparisons as non-production spike material.
- Disposable build/spike working directories under `/tmp/zhoor-arc02/<RUN_ID>/`.
- `docs/planning/zhoor-adoption/handoffs/ARC-02.md` with commands, measurements, dependency inventory, and comparison table.
- No production language/package decision or production scaffold. ARC-02 must compare minimal Rust, minimal Python, and a bounded mixed option; the existing ADR-0002 `system.info.read` crate may be cited as empirical evidence but does not predetermine the selection.

## Required comparison

Evaluate at least: Rust-first runtime, Python-first runtime, and a deliberately bounded mixed option. Compare type/schema safety, Unix-socket/peer-credential support, packaging, test/eval ergonomics, dependency/security burden, cold-start/resource behavior, agent competency, and risk of duplicated logic.

## Success condition

All three options have reproducible minimal evidence and a completed independent review before ARC-02 is marked DONE. ARC-02 records evidence and tradeoffs only; it makes no production decision, scaffolds no production layout, and introduces no model/backend package. ARC-03 consumes the accepted ARC-02 evidence and performs the production adjudication.

## Stop

If a required compiler/runtime is unavailable or two spike attempts fail, do not install broad toolchains without approval. Record commands/errors for ARC-03. Architecture acceptance is out of scope.
