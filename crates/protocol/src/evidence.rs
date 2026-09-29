//! Evidence references and trust tiers.
//!
//! Evidence trust is separate from confidence and from authorization. The
//! four tiers follow the architecture: `untrusted` is context only,
//! `observed` supports low-risk decisions, `verified` may support
//! policy-defined actions, and `policy` carries authorization rules only. An
//! Evidence Selector may reduce context but can never raise a trust tier, so
//! the protocol has no operation that promotes a tier.

use serde::Serialize;
use serde_json::Value;

use crate::error::ProtocolError;
use crate::util::{object_fields, str_field};

/// Evidence trust tier. Confidence is never evidence trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustTier {
    /// User text, web content, or tool output not independently validated;
    /// context only.
    Untrusted,
    /// Runtime or tool observation with an auditable receipt; supports
    /// low-risk decisions.
    Observed,
    /// Validated against known state, a signed source, or an approved test
    /// oracle; may support policy-defined actions.
    Verified,
    /// Local policy or configuration; authorization rules only.
    Policy,
}

impl TrustTier {
    /// Parses a wire trust tier string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<TrustTier> {
        match value {
            "untrusted" => Some(TrustTier::Untrusted),
            "observed" => Some(TrustTier::Observed),
            "verified" => Some(TrustTier::Verified),
            "policy" => Some(TrustTier::Policy),
            _ => None,
        }
    }
}

/// Evidence kind class from the canonical schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// Text supplied by the user.
    UserInput,
    /// Registry or index record.
    Registry,
    /// Tool output.
    ToolResult,
    /// Durable memory record.
    Memory,
    /// Retrieved external document.
    ExternalDocument,
}

impl EvidenceKind {
    /// Parses a wire evidence kind string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<EvidenceKind> {
        match value {
            "user_input" => Some(EvidenceKind::UserInput),
            "registry" => Some(EvidenceKind::Registry),
            "tool_result" => Some(EvidenceKind::ToolResult),
            "memory" => Some(EvidenceKind::Memory),
            "external_document" => Some(EvidenceKind::ExternalDocument),
            _ => None,
        }
    }
}

/// Content-addressed evidence reference with its kind, trust tier, and source
/// label. References are `evidence://sha256/<64 lowercase hex>` and never
/// carry content payloads.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EvidenceRef {
    /// Content hash reference (`ref` on the wire).
    #[serde(rename = "ref")]
    pub reference: String,
    /// Evidence kind class.
    pub kind: EvidenceKind,
    /// Evidence trust tier.
    pub trust_tier: TrustTier,
    /// Bounded source label.
    pub source: String,
}

/// Prefix and hash length for canonical evidence references.
const REFERENCE_PREFIX: &str = "evidence://sha256/";
/// Canonical SHA-256 hex digest length.
const REFERENCE_HASH_LEN: usize = 64;
/// Exact wire length of a canonical evidence reference.
pub(crate) const REFERENCE_LEN: usize = REFERENCE_PREFIX.len() + REFERENCE_HASH_LEN;

const EVIDENCE_REF_FIELDS: &[&str] = &["ref", "kind", "trust_tier", "source"];

impl EvidenceRef {
    /// Reports whether a string is a canonical
    /// `evidence://sha256/<64 lowercase hex>` reference.
    #[must_use]
    pub fn is_canonical_reference(value: &str) -> bool {
        let bytes = value.as_bytes();
        if bytes.len() != REFERENCE_LEN || !value.starts_with(REFERENCE_PREFIX) {
            return false;
        }
        bytes[REFERENCE_PREFIX.len()..]
            .iter()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
    }

    /// Parses and validates one evidence reference object, failing closed on
    /// unknown fields, missing fields, malformed references, unknown kinds,
    /// and unknown trust tiers.
    pub fn from_value(value: &Value) -> Result<EvidenceRef, ProtocolError> {
        let object = object_fields(value, EVIDENCE_REF_FIELDS, EVIDENCE_REF_FIELDS)?;
        let reference = str_field(object, "ref", 1, REFERENCE_LEN)?;
        if !EvidenceRef::is_canonical_reference(&reference) {
            return Err(ProtocolError::InvalidEvidenceReference {
                reason: "ref must be evidence://sha256/<64 lowercase hex>",
            });
        }
        let kind_raw = str_field(object, "kind", 1, 32)?;
        let kind = EvidenceKind::parse(&kind_raw).ok_or(ProtocolError::MalformedJson {
            reason: "unknown evidence kind",
        })?;
        let tier_raw = str_field(object, "trust_tier", 1, 16)?;
        let trust_tier =
            TrustTier::parse(&tier_raw).ok_or_else(|| ProtocolError::UnknownTrustTier {
                found: tier_raw.clone(),
            })?;
        let source = str_field(object, "source", 1, 256)?;
        Ok(EvidenceRef {
            reference,
            kind,
            trust_tier,
            source,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{EvidenceKind, EvidenceRef, TrustTier};

    const HASH: &str = "e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0";

    #[test]
    fn canonical_reference_is_accepted() {
        let reference = format!("evidence://sha256/{HASH}");
        assert!(EvidenceRef::is_canonical_reference(&reference));
        assert_eq!(reference.len(), 82);
    }

    #[test]
    fn malformed_references_are_rejected() {
        assert!(!EvidenceRef::is_canonical_reference("evidence://md5/abc"));
        assert!(!EvidenceRef::is_canonical_reference(
            "evidence://sha256/xyz"
        ));
        assert!(!EvidenceRef::is_canonical_reference(&format!(
            "evidence://sha256/{HASH}X"
        )));
    }

    #[test]
    fn trust_tier_parse_covers_all_tiers() {
        assert_eq!(TrustTier::parse("untrusted"), Some(TrustTier::Untrusted));
        assert_eq!(TrustTier::parse("observed"), Some(TrustTier::Observed));
        assert_eq!(TrustTier::parse("verified"), Some(TrustTier::Verified));
        assert_eq!(TrustTier::parse("policy"), Some(TrustTier::Policy));
        assert_eq!(TrustTier::parse("trusted"), None);
    }

    #[test]
    fn evidence_kind_parse_is_fail_closed() {
        assert_eq!(
            EvidenceKind::parse("external_document"),
            Some(EvidenceKind::ExternalDocument)
        );
        assert_eq!(EvidenceKind::parse("document"), None);
    }

    #[test]
    fn unknown_trust_tier_fails_closed() {
        let value = json!({
            "ref": format!("evidence://sha256/{HASH}"),
            "kind": "user_input",
            "trust_tier": "trusted",
            "source": "user prompt"
        });
        let error = EvidenceRef::from_value(&value).expect_err("unknown tier must fail");
        assert!(matches!(
            error,
            crate::error::ProtocolError::UnknownTrustTier { .. }
        ));
    }
}
