# aiosd contract v1

## Authority and boundary

This document implements owner decision ADR-0004 / D-025 under the existing
SEC-01 authorization D-024. The wire authority is
[`aiosd-v1.schema.json`](../../../schemas/aiosd/aiosd-v1.schema.json); the Rust
contract parser is `zhoor-aiosd-contracts`.

SEC-01 defines and structurally validates contracts. It does not issue or
verify grants, authenticate callers, authorize requests, execute a capability,
open a socket, read the host, persist replay state, or verify host filesystem
containment. A parsed request/grant remains untrusted until the later GRANT-01
verifier accepts its authenticator against a configured trust root. An issuer
ID, schema-valid object, authenticated socket caller, or digest does not prove
grant authenticity.

## Closed v1 request

The only capability is `capabilities.describe`. Its `arguments` value must be
exactly `{}`: no defaults, unknown fields, arbitrary JSON, or path-bearing
inputs. The normalized argument bytes are exactly the two UTF-8 bytes `{}`.
`argument_sha256` is SHA-256 over those bytes, serialized as
`sha256:<64 lowercase hexadecimal characters>`. This digest binds the
normalized argument bytes; it is not an authenticator, signature, trust
statement, or authorization.

The only resource scope is
`registered_capability_descriptor_metadata`. It denotes static registered
capability-descriptor metadata for this introspection operation. It has no
filesystem path meaning. Shell, service, package, network, mutation, and
generic future scopes are not represented.

Evidence uses a closed JSON object keyed by the protocol-owned canonical
`evidence://sha256/<64 lowercase hex>` reference. Each value contains only
`kind`, `trust_tier`, and `source`; at least one reference and a decision hash
are required, with at most 64 entries. Keys make evidence identity explicit
in the schema data model. The bounded Rust parser also rejects duplicate raw
JSON member names after decoding escapes. Grant and request evidence maps are
compared as sets after bytewise ordering by canonical reference; each binding
includes the reference digest, evidence kind, trust tier, and exact UTF-8
source label.

## Grant claims and authenticator envelope

A grant v1 contains these closed claims:

- `grant_version`, canonical lowercase UUID `grant_id`, and bounded `issuer_id`;
- `client_identity` with `kind: "unix_uid"` and a `u32` UID supplied to the
  later verifier from Linux Unix peer credentials;
- the exact capability, normalized argument digest, and fixed resource scope;
- bounded `policy_version`, unsigned `expires_at_unix_ms`, and canonical
  lowercase UUID `trace_id`;
- lowercase SHA-256 `decision_hash` and the complete evidence bindings above;
- required `authenticator: { format, proof }`.

`format` is 1–64 lowercase ASCII letters/digits/dot/underscore/hyphen and
starts with a letter. `proof` is 1–4096 opaque bytes represented by exactly
2–8192 lowercase hexadecimal characters. The JSON schema and parser reject
unknown fields, duplicate JSON object members, empty/malformed values,
unsupported contract versions, and inputs beyond the byte/item bounds. A
structurally valid format is not necessarily supported. Only GRANT-01 can
report that a proof is valid or its issuer trusted; unsupported formats and
missing, malformed, invalid, or
untrusted proofs fail closed there.
The shared `grantVerificationFailure` vocabulary distinguishes missing or
malformed authenticators, unsupported formats, invalid proof, untrusted
issuer, unsupported version, caller/capability/argument/scope/policy mismatch,
expiry, replay, and trace/decision/evidence mismatch. Every value is a
failure; there is no structural `verified` result.

The grant carries an untrusted client UID claim. A later verifier compares it
with the UID supplied separately by the trusted peer-credential layer; the
parsed claim itself is not authenticated caller identity. SEC-01 does not
obtain peer credentials or create GID/PID authorization rules. The accepted
threat model remains one trusted local account.

## Frozen claims transcript

The transcript binds grant/contract version, issuer, client UID, capability,
argument digest, resource scope, policy version, expiry, trace ID, decision
hash, full evidence bindings, and authenticator format. Proof bytes are
excluded to avoid self-reference. Any change to one of those values or the
format changes the transcript. GRANT-01 may choose how its accepted
authenticator operates over these bytes, but may not redefine the encoding or
omit claims.

The exact encoding is:

1. Prefix bytes: ASCII `ZHOOR-AIOSD-GRANT-CLAIMS`, then NUL, then byte `0x01`.
2. Append fields in ascending tag order. Every field is
   `tag:u8 || value_length:u32be || value_bytes`.
3. Tags and value encodings:

| Tag | Value |
|---:|---|
| 1 | Grant contract version ASCII `1` |
| 2 | Grant UUID decoded from lowercase text to 16 bytes |
| 3 | Issuer ID exact ASCII bytes |
| 4 | UID as unsigned 32-bit big-endian |
| 5 | Capability ID exact ASCII bytes |
| 6 | Argument SHA-256 decoded to 32 digest bytes |
| 7 | Resource-scope token exact ASCII bytes |
| 8 | Policy-version token exact ASCII bytes |
| 9 | Expiry as unsigned 64-bit big-endian milliseconds |
| 10 | Trace UUID decoded to 16 bytes |
| 11 | Decision SHA-256 decoded to 32 digest bytes |
| 12 | Evidence count as `u32be`, then entries sorted by canonical reference bytes; each entry is digest bytes (32), followed by kind, trust-tier, and source, each encoded as `u32be byte_length || exact UTF-8 bytes` |
| 13 | Authenticator format exact ASCII bytes |

All integers use network/big-endian byte order. Text is exact UTF-8/ASCII as
specified, with no Unicode normalization, locale rules, whitespace, JSON
serialization, map iteration, or host input in the transcript. Evidence
reference digests are unique; duplicate references reject. The fixed domain
prefix and version prevent cross-contract use. The transcript itself is not
authenticated until GRANT-01 verifies a supported authenticator.

Raw JSON objects with duplicate member names reject in the Rust parser before
generic JSON-value inspection, so different JSON decoders cannot select
different grant claims.

## Expiry and replay contract

`expires_at_unix_ms` is unsigned Unix epoch milliseconds. A validator receives
trusted time explicitly; SEC-01 has no hidden clock read. A grant is expired
when `trusted_now_unix_ms >= expires_at_unix_ms`; it is valid at the preceding
instant only. V1 also caps remaining lifetime at 30,000 ms so a far-future
expiry is not treated as a short-lived grant. GRANT-01 owns its trusted time
source and enforcement.

`grant_id` uses the existing lowercase `8-4-4-4-12` UUID syntax. It is bound
into the transcript and means one operation. Its syntax does not prove
unpredictability, uniqueness, or authenticity. A validator context may state
that the ID was already consumed; that context produces the typed
`replayed_or_consumed` result. SEC-01 does not store or infer consumption.
GRANT-01 owns ID generation, uniqueness enforcement, state, and atomic
single-use consumption.

## Framing contract

One complete frame is a four-byte unsigned big-endian payload length followed
by exactly that many UTF-8 JSON bytes. The length excludes the prefix, must be
between 1 and 65,536 bytes, and the in-memory complete frame must contain no
trailing bytes. The JSON envelope has `frame_version: "1"`, `message_kind`
(`request` or `response`), a lowercase UUID `correlation_id`, and exactly the
matching typed body. For a request, correlation equals `request_id`; for a
response, it equals the receipt's `request_id`. Unknown versions/kinds,
missing/extra bodies, partial frames, oversized frames, and correlation
mismatches reject. SEC-01 implements only complete-buffer parsing; it does not
implement socket reads/writes, retry, listener, client, lifecycle, or peer
credential lookup.

## Receipt and postcondition contract

A bounded receipt binds request/grant IDs, peer UID, validated capability,
argument digest, closed outcome, safe result reference, start/completion
timestamps, trace, decision hash, and evidence. The result reference is a
content-addressed `result://sha256/<64 lowercase hexadecimal characters>`
label; it is not proof that the referenced content is authentic or safe. A
receipt records what was executed or observed. It does not prove that any
postcondition holds.

A postcondition verification record contains a condition ID, deterministic
verification-owner ID, verdict, request/trace/grant/receipt linkage, and at
least one deterministic evidence reference. The closed verdicts are
`satisfied`, `failed`, and `unknown`. Failed and unknown/unverifiable results
do not make a workflow eligible. Only all required postconditions being
satisfied can satisfy this promotion prerequisite. Semantic/model output may
be evidence or advice but never replaces mandatory deterministic checks.
The separate `PostconditionRequirement` type names each mandatory condition
and its deterministic verification owner. A verification record must match
that requirement and link to a receipt that matches the request. SEC-01
validates linkage but does not verify the evidence or aggregate workflow
eligibility.
SEC-01 defines these records and effects; it does not implement an execution,
verification, policy, or promotion state engine.

## Path-shaped hostile inputs and limits

Traversal-like, absolute-path-like, and symlink-like values are rejected as
unsupported `capabilities.describe` scopes. These fixtures test the closed
scope contract only. They do not test filesystem traversal resolution,
canonicalization, symlink resolution, or allowlisted-root containment. Those
architecture requirements remain binding for a future authorized
path-bearing operation and later aiosd enforcement.

The parser rejects JSON payloads over 65,536 bytes before parsing. Evidence
maps are capped at 64 entries and use the protocol-owned bounded reference
shape; the descriptor array is capped at one. Authenticator proof bytes are
capped at 4,096. All owned strings have schema and Rust length/grammar checks.
Evidence-map and array count bounds are enforced by custom deserializers before
vector growth beyond the limit.

Descriptor descriptions reject Unicode control characters in both schema and
Rust validation, including the C1 range U+0080–U+009F.

## Explicit non-goals

No authenticator verification, cryptographic primitive, issuer/root storage,
grant issuer, daemon, executor, socket transport, peer-credential lookup,
privilege mechanism, multi-user policy, replay store, filesystem access, or
capability execution is implemented here.
