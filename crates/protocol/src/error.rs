//! Typed protocol errors. Every validation failure fails closed with one of
//! these variants; they mirror the wire error codes in the canonical response
//! schema.

use std::fmt;

/// Coarse error classification used by contract tests to pin which typed
/// error each invalid fixture must produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// `protocol_version` is present but not supported by this validator.
    UnsupportedVersion,
    /// `operation` is not a known v1 operation kind.
    UnknownOperation,
    /// `worker_kind` is not a known bounded v1 worker kind.
    UnknownWorkerKind,
    /// `result.kind` is not a known v1 result kind.
    UnknownResultKind,
    /// `disposition` is not a known disposition.
    InvalidDisposition,
    /// A `trust_tier` is not one of the four defined tiers.
    UnknownTrustTier,
    /// `backend` is not a known backend class.
    UnknownBackend,
    /// A confidence value is outside `[0, 1]` or non-finite.
    InvalidConfidence,
    /// A budget field is missing bounds compliance.
    InvalidBudget,
    /// A deadline or latency field violates bounds or deadline accounting.
    InvalidDeadline,
    /// A required identifier is malformed.
    InvalidId,
    /// A required field is absent.
    MissingField,
    /// A field not defined by the canonical schema is present.
    UnknownField,
    /// An evidence reference is malformed, duplicated, or unlinked.
    InvalidEvidenceReference,
    /// The envelope content exceeds its declared input token budget.
    OverBudget,
    /// The envelope contradicts itself or its paired envelope.
    InconsistentEnvelope,
    /// A result payload violates its kind contract.
    InvalidResult,
    /// The JSON text or field shape is malformed.
    MalformedJson,
}

/// Typed protocol validation error. Fail-closed contract: any deviation from
/// the canonical v1 schemas or the cross-envelope rules yields one of these
/// variants, never a lenient default.
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolError {
    /// `protocol_version` is present but unsupported.
    UnsupportedVersion {
        /// The unsupported version string as found on the wire.
        found: String,
    },
    /// The operation kind is unknown to protocol v1.
    UnknownOperation {
        /// The unknown operation string as found on the wire.
        found: String,
    },
    /// The worker kind is unknown to protocol v1.
    UnknownWorkerKind {
        /// The unknown worker kind string as found on the wire.
        found: String,
    },
    /// The result kind is unknown to protocol v1.
    UnknownResultKind {
        /// The unknown result kind string as found on the wire.
        found: String,
    },
    /// The disposition is not one of the four defined dispositions.
    InvalidDisposition {
        /// The unknown disposition string as found on the wire.
        found: String,
    },
    /// The evidence trust tier is not one of the four defined tiers.
    UnknownTrustTier {
        /// The unknown trust tier string as found on the wire.
        found: String,
    },
    /// The backend class is unknown to protocol v1.
    UnknownBackend {
        /// The unknown backend string as found on the wire.
        found: String,
    },
    /// A confidence value is outside `[0, 1]` or non-finite.
    InvalidConfidence {
        /// The offending confidence field name.
        field: &'static str,
        /// The offending value rendered for diagnostics.
        found: String,
    },
    /// A budget field violates its bounds.
    InvalidBudget {
        /// Static explanation of the budget violation.
        reason: &'static str,
    },
    /// A deadline or latency field violates its bounds or the deadline
    /// accounting rules.
    InvalidDeadline {
        /// Static explanation of the deadline violation.
        reason: &'static str,
    },
    /// A required identifier is malformed.
    InvalidId {
        /// The offending identifier field name.
        field: &'static str,
        /// The malformed identifier as found on the wire.
        found: String,
    },
    /// A required field is absent.
    MissingField {
        /// The absent field name.
        field: &'static str,
    },
    /// A field not defined by the canonical schema is present.
    UnknownField {
        /// The unknown field name.
        field: String,
    },
    /// An evidence reference is malformed, duplicated, or unlinked.
    InvalidEvidenceReference {
        /// Static explanation of the evidence reference violation.
        reason: &'static str,
    },
    /// The envelope content exceeds its declared input token budget.
    OverBudget {
        /// Explanation of the budget overrun.
        reason: String,
    },
    /// The envelope contradicts itself or its paired envelope.
    InconsistentEnvelope {
        /// Explanation of the inconsistency.
        reason: String,
    },
    /// A result payload violates its kind contract.
    InvalidResult {
        /// Static explanation of the result violation.
        reason: &'static str,
    },
    /// The JSON text or a field shape is malformed.
    MalformedJson {
        /// Static explanation of the malformed shape.
        reason: &'static str,
    },
}

impl ProtocolError {
    /// Returns the coarse classification for this error.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self {
            ProtocolError::UnsupportedVersion { .. } => ErrorKind::UnsupportedVersion,
            ProtocolError::UnknownOperation { .. } => ErrorKind::UnknownOperation,
            ProtocolError::UnknownWorkerKind { .. } => ErrorKind::UnknownWorkerKind,
            ProtocolError::UnknownResultKind { .. } => ErrorKind::UnknownResultKind,
            ProtocolError::InvalidDisposition { .. } => ErrorKind::InvalidDisposition,
            ProtocolError::UnknownTrustTier { .. } => ErrorKind::UnknownTrustTier,
            ProtocolError::UnknownBackend { .. } => ErrorKind::UnknownBackend,
            ProtocolError::InvalidConfidence { .. } => ErrorKind::InvalidConfidence,
            ProtocolError::InvalidBudget { .. } => ErrorKind::InvalidBudget,
            ProtocolError::InvalidDeadline { .. } => ErrorKind::InvalidDeadline,
            ProtocolError::InvalidId { .. } => ErrorKind::InvalidId,
            ProtocolError::MissingField { .. } => ErrorKind::MissingField,
            ProtocolError::UnknownField { .. } => ErrorKind::UnknownField,
            ProtocolError::InvalidEvidenceReference { .. } => ErrorKind::InvalidEvidenceReference,
            ProtocolError::OverBudget { .. } => ErrorKind::OverBudget,
            ProtocolError::InconsistentEnvelope { .. } => ErrorKind::InconsistentEnvelope,
            ProtocolError::InvalidResult { .. } => ErrorKind::InvalidResult,
            ProtocolError::MalformedJson { .. } => ErrorKind::MalformedJson,
        }
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::UnsupportedVersion { found } => {
                write!(formatter, "unsupported protocol version: {found:?}")
            }
            ProtocolError::UnknownOperation { found } => {
                write!(formatter, "unknown operation kind: {found:?}")
            }
            ProtocolError::UnknownWorkerKind { found } => {
                write!(formatter, "unknown worker kind: {found:?}")
            }
            ProtocolError::UnknownResultKind { found } => {
                write!(formatter, "unknown result kind: {found:?}")
            }
            ProtocolError::InvalidDisposition { found } => {
                write!(formatter, "invalid disposition: {found:?}")
            }
            ProtocolError::UnknownTrustTier { found } => {
                write!(formatter, "unknown evidence trust tier: {found:?}")
            }
            ProtocolError::UnknownBackend { found } => {
                write!(formatter, "unknown backend: {found:?}")
            }
            ProtocolError::InvalidConfidence { field, found } => {
                write!(formatter, "confidence {field} out of bounds: {found}")
            }
            ProtocolError::InvalidBudget { reason } => {
                write!(formatter, "invalid budget: {reason}")
            }
            ProtocolError::InvalidDeadline { reason } => {
                write!(formatter, "invalid deadline: {reason}")
            }
            ProtocolError::InvalidId { field, found } => {
                write!(formatter, "invalid identifier in {field}: {found:?}")
            }
            ProtocolError::MissingField { field } => {
                write!(formatter, "missing required field: {field}")
            }
            ProtocolError::UnknownField { field } => {
                write!(
                    formatter,
                    "unknown field not permitted by the canonical schema: {field:?}"
                )
            }
            ProtocolError::InvalidEvidenceReference { reason } => {
                write!(formatter, "invalid evidence reference: {reason}")
            }
            ProtocolError::OverBudget { reason } => {
                write!(formatter, "over budget: {reason}")
            }
            ProtocolError::InconsistentEnvelope { reason } => {
                write!(formatter, "inconsistent envelope: {reason}")
            }
            ProtocolError::InvalidResult { reason } => {
                write!(formatter, "invalid result payload: {reason}")
            }
            ProtocolError::MalformedJson { reason } => {
                write!(formatter, "malformed JSON: {reason}")
            }
        }
    }
}

impl std::error::Error for ProtocolError {}

#[cfg(test)]
mod tests {
    use super::{ErrorKind, ProtocolError};

    #[test]
    fn kinds_are_distinct_and_display_is_populated() {
        let samples = [
            ProtocolError::UnsupportedVersion {
                found: "2".to_owned(),
            },
            ProtocolError::UnknownOperation {
                found: "rank".to_owned(),
            },
            ProtocolError::InvalidBudget { reason: "test" },
            ProtocolError::MalformedJson { reason: "test" },
        ];
        let mut seen = Vec::new();
        for error in &samples {
            assert!(!seen.contains(&error.kind()));
            assert!(!error.to_string().is_empty());
            seen.push(error.kind());
        }
        assert_eq!(seen.len(), 4);
        assert_eq!(samples[0].kind(), ErrorKind::UnsupportedVersion);
    }
}
