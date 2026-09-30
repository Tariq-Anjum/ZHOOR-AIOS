# PRO-02 handoff — owner contract choice required

## Run state

- Starting local SHA: `d15138727c43877049f51326d05aba2987d88e0f`
- Starting remote SHA: `2e73ef44181bc74b55fad25e7e2fd5f1dac9eaa1`
- PRO-02 governance/authorization SHA: `431024a356f9d3c80f39a961c73f10ac4d2abc0d`
- PRO-02 assignment SHA: `2cb8d54560079d1a012ee5014e1b96dd9f15f1a0`
- Status: `BLOCKED — OWNER_CONTRACT_DECISION_REQUIRED`
- Implementation source/schema/fixture changes: none
- Validation: not run; design gate stopped before source mutation
- Exact-SHA architecture challenge: `OWNER_CONTRACT_DECISION_REQUIRED` at assignment SHA `2cb8d54560079d1a012ee5014e1b96dd9f15f1a0`
- Reviewer route: contained Pi 0.87.1 / configured OpenAI-Codex / GPT-6 Luna xhigh / read-only; `AGENT_CONTAINMENT=PASS`; result captured and reviewer process stopped
- Challenge evidence: `/tmp/zhoor-pro-02/ZHOOR-20260930T081737Z-PRO-02/architecture-challenge-result.md`

## Evidence

Protocol v1 candidates are `Vec<String>` and no existing request/response contract defines registry descriptor identity. Architecture §4 gives compact routing fields but no descriptor version placement. The local generated catalog has a root `schema_version`, which is compatibility evidence only and does not choose the ZHOOR contract.

The provisional proposal separates descriptor format version from skill artifact release version; proposes no artifact release-version field in v1 because the accepted architecture/local compact index provide no stable convention; hashes exact raw `SKILL.md` bytes as lowercase `sha256:<64 hex>`; and treats `registry_version` as a root snapshot label supplied against a caller's expected pin. It preserves request-v1/response-v1 and makes no descriptor-metadata hash or security-authority claim.

The independent challenge found that:

1. `registry_version` needs a defined non-reuse scope to identify one snapshot for architecture cache keys; the expected value must come from caller configuration/API if snapshot pinning is required.
2. Repository authority does not specify whether `descriptor_format_version` is carried by each descriptor or once at the registry root. Both are reasonable, and they produce different REG-01 schema/API behavior.
3. The proposed artifact digest covers `SKILL.md` bytes only, not descriptor metadata or companion files; a later fetch must verify bytes before claiming a match. It does not authenticate the index.

## Owner contract choice required

Both choices retain these common semantics: `descriptor_format_version` means descriptor field-contract version `"1"`, not skill artifact release version; artifact release version is not represented in v1; `artifact_sha256` covers exact raw `SKILL.md` bytes with lowercase `sha256:<64 hex>` text; `registry_version` is a root-level snapshot label, unique and never reused within the configured registry identity, with exact caller-pin mismatch rejected; and request-v1/response-v1 remain unchanged. The digest is content identity/integrity metadata only.

Choose one:

1. **Per-entry descriptor format version.** Every descriptor carries `descriptor_format_version: "1"` alongside `artifact_sha256`; the registry root carries `registry_version`. REG-01 validates each descriptor version. This permits each descriptor to declare its own contract version but repeats the same value in a uniform v1 registry. No protocol-v1 envelope changes. Existing registry input would need the new field.
2. **Root-level descriptor format version.** The registry root carries `descriptor_format_version: "1"` and `registry_version`; each descriptor carries `artifact_sha256`. REG-01 validates the format once and applies it to every entry. This avoids per-entry duplication but does not represent mixed descriptor formats in one registry snapshot. No protocol-v1 envelope changes. Existing registry input would need the new root field.

In either option, the content digest does not cover descriptor metadata or companion files and proves no provenance, authentication, trust, or authorization. Keep PRO-02 and REG-01 BLOCKED until the owner selects/clarifies the contract. After that decision, resume only PRO-02; REG-01 remains blocked pending a separate owner authorization.
