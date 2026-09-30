# SEC-01 independent security review — Luna r1

## Review identity

- Role: independent exact-SHA security reviewer.
- Route: fresh Pi / OpenAI-Codex / GPT-6 Luna / xhigh / read-only.
- Herdr role/name: `sec-01-final-independent-reviewer` / `sec01reviewfinal`.
- Containment: `AGENT_CONTAINMENT=PASS`; one interactive pane, managed Sidebar only, clean detached checkout, read-only guard hash matched, model tools limited to `read`, `grep`, `find`, `ls`, no Pi child process or credential-like Pi environment key.
- Reviewed candidate SHA: `cf4dd5228bb363f41abefa0ffc065078a4637dc7` (provided to reviewer; reviewer did not independently run Git).
- Verdict: `CHANGES_REQUIRED`.
- Reviewer did not run Cargo and did not claim repository write-scope verification.

## Findings

1. **MEDIUM — duplicate raw JSON members were accepted.** `crates/aiosd-contracts/src/lib.rs` parsed raw JSON into `serde_json::Value` before typed validation. Duplicate keys therefore collapsed to one value. Two decoders could choose different `issuer_id` or `expires_at_unix_ms` values, creating ambiguity in what was verified versus consumed. The JSON Schema operates on a parsed JSON data model and cannot reject duplicate raw members.
2. **MEDIUM — transcript behavior lacked a locked vector and complete mutation coverage.** Tests verified proof exclusion and issuer substitution but did not pin a golden transcript or cover evidence ordering and substitution of each remaining bound claim. The reviewer found the visible encoder consistent with the documentation but noted a regression could omit a bound claim without failing tests.
3. **LOW — UID claim documentation could imply authentication.** `UnixPeerIdentity` documentation said the trusted peer-credential layer supplied the value, while the parser constructs it from the serialized grant claim. A caller could mistake a parsed claim for an authenticated identity despite the explicit comparison in the context validator.

## Reviewer answers

- Structural parsing does not verify authenticity; a fake opaque proof can pass structural and binding checks, but these functions do not claim it is authenticated. GRANT-01 remains the verifier owner.
- Reviewer found the visible transcript to bind version, grant ID, issuer, UID, capability, argument digest, scope, policy version, expiry, trace, decision, evidence, and authenticator format; proof bytes are excluded.
- Replay requires the explicit `grant_already_consumed` validation context; expiry equality is expired at `trusted_now_unix_ms >= expires_at_unix_ms`.
- Path-shaped scope values reject the fixed non-path scope; no filesystem canonicalization or symlink enforcement claim was found.
- Receipt is not postcondition proof; the three verdicts are distinct and unknown does not satisfy eligibility.
- No prohibited execution, issuer, trust-root, replay-store, privilege, multi-user, or filesystem implementation was found in the bounded source inspected.
- Reviewer found duplicate raw JSON member handling to be the material schema/parser ambiguity; did not run Cargo.

## Disposition

The finding set is being corrected narrowly on the assigned SEC-01 paths. This report applies only to `cf4dd5228bb363f41abefa0ffc065078a4637dc7`; a fresh exact-SHA review is required after correction. SEC-01 remains in `REVIEW` during the correction and re-review sequence.
