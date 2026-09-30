---
title: Freeze the SEC-01 grant-authenticity boundary and defer its mechanism
status: Accepted
accepted: 2026-09-30
deciders: Project owner
amends: ZHOOR Architecture v0.2 section 8; D-024 authorizes SEC-01 only
---

# ADR-0004: Freeze the SEC-01 grant-authenticity boundary and defer its mechanism

## Context

SEC-01 must define the versioned aiosd/grant contract before any grant issuer,
verifier, or enforcement implementation. Its accepted ownership assigns the
wire contract, complete grant claims, normalized argument binding, hostile
fixtures, framing contract, and postcondition contract to SEC-01. GRANT-01
later owns the Policy Engine issuer/verifier, trust-root material and mapping,
expiry/replay enforcement, and key/test-root handling.

The accepted architecture requires a policy-issued, short-lived,
single-operation grant bound to the authenticated client identity and decision
provenance. A Unix peer credential authenticates the connected caller; it does
not by itself establish who issued a serialized grant. SEC-01 therefore needs
to distinguish structural contract validation from the later verifier's
authenticity decision without selecting the GRANT-01 trust-root mechanism.

## Decision

1. The owner selects **Alternative B**. SEC-01 freezes the complete v1 grant
   claims, wire structure, required authenticator envelope, deterministic
   claims-binding transcript, and the security properties every conforming
   verifier must establish. GRANT-01 selects and implements the concrete
   authenticator mechanism and trust-root within that frozen contract.
2. A v1 serialized grant contains a required, bounded, closed authenticator
   envelope with a bounded format identifier and bounded opaque proof
   representation. SEC-01 chooses their exact field names and text/binary
   encoding as deterministic contract details. Unknown fields, empty or
   malformed values, unsupported contract versions, and over-limit inputs
   fail closed.
3. SEC-01 structural validation does not authenticate a proof, establish an
   issuer's trust, or make a grant authorized. A structurally valid grant is
   untrusted input until GRANT-01 successfully verifies its authenticator
   against an accepted trust root. GRANT-01 rejects missing, malformed,
   unsupported, invalid, or untrusted authenticators. Unsupported formats
   fail closed.
4. The authenticator/verifier must bind the entire frozen claims transcript,
   including the contract version, grant ID/nonce, issuer ID, authenticated
   client identity, exact capability, normalized argument binding, resource
   scope, policy version, expiry, originating trace ID, decision hash, and
   evidence binding. No conforming mechanism may authenticate only a subset.
   Changing any bound claim or the authenticator format invalidates
   verification. SEC-01 freezes the transcript encoding; GRANT-01 may not
   redefine it.
5. An issuer identifier, SHA-256 digest, authenticated socket caller, or
   schema-valid grant alone does not establish that the grant was
   policy-issued. Caller authentication and grant authenticity are distinct
   properties.
6. The SEC-01 v1 capability is only `capabilities.describe` with exactly the
   empty argument object `{}`. Its normalized bytes are the two UTF-8 bytes
   `{}`; the SHA-256 digest binds those bytes and is not an authenticator.
   The sole resource scope is registered capability-descriptor metadata for
   this operation. No v1 argument or scope carries a filesystem path.
7. Grant expiry is represented as unsigned Unix epoch milliseconds and is
   valid only while `trusted_now < expires_at`; equality means expired.
   SEC-01 defines the representation and comparison; GRANT-01 provides the
   trusted time source and enforcement.
8. SEC-01 defines the unique grant-ID syntax, binding, single-operation
   semantics, explicit replay validation context, and typed consumed/replayed
   failure. GRANT-01 owns generation, uniqueness enforcement, consumption
   history, and atomic one-use rejection.
9. SEC-01 defines only the bounded frame/version, message kind, correlation,
   and complete-message contract. Socket implementation, peer-credential
   lookup, and transport lifecycle remain later transport work.
10. Receipts record execution/observation facts and do not prove postconditions.
    Only deterministic verification can satisfy required postconditions;
    failed or unknown/unverifiable results do not satisfy the promotion
    prerequisite, and only all required satisfied results may satisfy it.
11. This decision records no new SEC-01 authorization and authorizes no
    downstream card, grant issuance, execution, daemon, or executor.

## Consequences

- The v1 schema reserves the authenticator envelope now, so GRANT-01 can
  choose its concrete mechanism without adding or reshaping SEC-01 fields.
- SEC-01 can test closed structure, claims binding, substitution, bounds, and
  typed failure context. It cannot claim that opaque proof bytes are valid or
  trusted; tests of actual issuer authenticity belong to GRANT-01.
- A GRANT-01 verifier must implement the frozen transcript and reject every
  unsupported format. A new format may fit the reserved structure, but a
  verifier that does not support it rejects it. Reshaping the envelope or
  changing the transcript requires a later contract/version decision.
- A digest is a binding value only. It is not an authenticator, signature,
  authorization, or proof of provenance.
- Peer identity is externally authenticated by the later Unix peer-credential
  layer. The initial accepted threat model is one trusted local account;
  this ADR adds no multi-user authorization policy.
- Path traversal and symlink-shaped SEC-01 fixtures reject unsupported
  `capabilities.describe` resource-scope representations only. They do not
  establish host filesystem canonicalization or symlink resolution. The
  architecture's path enforcement requirement remains binding if a future
  path-bearing operation is authorized.
- Replay state, socket transport, execution, trust-root material, and
  postcondition state transitions are not implemented by SEC-01.

## Non-goals

- Selecting a concrete authenticator algorithm or implementing cryptography.
- PKI, certificates, key rotation framework, secret platform, or generic
  identity system.
- RBAC, privilege model, or multi-user authorization policy.
- Daemon, executor, socket implementation, replay store, or filesystem
  canonicalization implementation.
- Authorizing GRANT-01, AIOSD-01, AIOSD-02, or any other downstream card.
