//! ZHOOR decision protocol v1 contracts and deterministic validation.
//!
//! This crate is the production semantic validator for the ZHOOR decision
//! protocol, version 1. It implements the separation between the
//! Generation/Reasoning Plane and the deterministic Decision Plane:
//!
//! - A model may propose, classify, extract, or score, but never authorizes
//!   execution. Nothing in a request or response grants a capability.
//! - Confidence is bounded advice in `[0, 1]`. It is not authority and it is
//!   not evidence trust. Evidence trust tiers (`untrusted`, `observed`,
//!   `verified`, `policy`) are separate and never derived from confidence.
//! - `choice`, `score`, `extract`, and `verdict` are result kinds.
//!   `resolved`, `abstained`, `escalated`, and `denied` are dispositions, not
//!   result kinds.
//!
//! The canonical language-neutral wire contracts are the JSON Schema Draft
//! 2020-12 documents under `schemas/protocol/` in the repository root:
//! `request-v1.schema.json` and `response-v1.schema.json`. This crate is the
//! only production semantic validator for those contracts. Schema checking is
//! test-only; no production JSON Schema runtime is included here.
//!
//! All validation fails closed: unknown versions, unknown operation/worker
//! kinds, unknown result kinds, malformed or absent required identifiers,
//! malformed evidence references, unknown trust tiers, invalid confidence
//! bounds, invalid budgets and deadlines, over-budget or internally
//! inconsistent envelopes, and unknown fields produce typed
//! [`ProtocolError`] values. Envelopes carry no secret, hidden-reasoning, or
//! unrestricted-transcript fields.
//!
//! Entry points:
//!
//! - [`Request::from_value`] / [`Request::from_json_str`] parse and validate a v1
//!   request envelope.
//! - [`Response::from_value`] / [`Response::from_json_str`] parse and validate a
//!   v1 response envelope on its own.
//! - [`Response::validate_against`] enforces the cross-envelope rules that a
//!   standalone document cannot express: request linkage, evidence
//!   containment, candidate containment, and deadline accounting.
//!
//! Serialization is deterministic: every validated type round-trips through
//! `serde_json` with stable field order and no maps.

#![forbid(unsafe_code)]

pub mod error;
pub mod evidence;
pub mod request;
pub mod response;
mod util;

pub use error::{ErrorKind, ProtocolError};
pub use evidence::{EvidenceKind, EvidenceRef, TrustTier};
pub use request::{
    Constraints, Deadline, Operation, ProtocolVersion, Provenance, Request, WorkerKind,
};
pub use response::{
    Backend, Confidence, DecisionResult, Disposition, ExtractedField, Latency, Response,
    ScoreScale, VerdictValue, WireError, WireErrorCode,
};
