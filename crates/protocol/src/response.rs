//! Version 1 response envelope: parsing, typed model, and deterministic
//! semantic validation, including cross-envelope rules against a validated
//! request.

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::error::ProtocolError;
use crate::evidence::EvidenceRef;
use crate::request::{Operation, ProtocolVersion, Request};
use crate::util::{
    array_field, f64_field, is_unit_interval_number, is_uuid, object_fields, object_fields_map,
    optional_str_field, str_field, uint_field,
};

/// Maximum uncertainty text length in characters.
pub const MAX_UNCERTAINTY_CHARS: usize = 4096;
/// Maximum error message length in characters.
pub const MAX_ERROR_MESSAGE_CHARS: usize = 512;
/// Maximum number of evidence references in a response.
pub const MAX_RESPONSE_EVIDENCE_REFS: usize = 64;
/// Maximum number of choices in a choice result.
pub const MAX_RESULT_CHOICES: usize = 64;
/// Maximum number of extracted fields.
pub const MAX_RESULT_FIELDS: usize = 64;
/// Maximum extracted field name length in characters.
pub const MAX_FIELD_NAME_CHARS: usize = 128;
/// Maximum extracted scalar string length in characters.
pub const MAX_FIELD_VALUE_CHARS: usize = 4096;
/// Inclusive upper bound for each latency component in milliseconds.
pub const MAX_LATENCY_MS: u64 = 600_000;
/// Maximum identifier length accepted before the strict UUID check.
const MAX_ID_CHARS: usize = 64;

const RESPONSE_KNOWN: &[&str] = &[
    "protocol_version",
    "request_id",
    "result",
    "error",
    "confidence",
    "disposition",
    "uncertainty",
    "evidence_refs",
    "backend",
    "latency",
    "trace_id",
];

const RESPONSE_REQUIRED: &[&str] = &[
    "protocol_version",
    "request_id",
    "confidence",
    "disposition",
    "uncertainty",
    "evidence_refs",
    "backend",
    "latency",
    "trace_id",
];

const CONFIDENCE_FIELDS: &[&str] = &["raw", "calibrated", "calibration_ref"];
const LATENCY_FIELDS: &[&str] = &["queue_ms", "load_ms", "inference_ms"];
const RESULT_FIELDS: &[&str] = &["kind", "value"];
const CHOICE_VALUE_FIELDS: &[&str] = &["choices"];
const SCORE_VALUE_FIELDS: &[&str] = &["scale", "score", "threshold"];
const EXTRACT_VALUE_FIELDS: &[&str] = &["fields"];
const EXTRACTED_FIELD_FIELDS: &[&str] = &["name", "value", "evidence"];
const VERDICT_VALUE_FIELDS: &[&str] = &["verdict"];
const ERROR_FIELDS: &[&str] = &["code", "message"];

/// Disposition of a decision. Dispositions are not result kinds; they record
/// how the deterministic plane handled the proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    /// A typed result was produced and is the response payload.
    Resolved,
    /// No result was produced; the worker declined to fabricate an answer.
    Abstained,
    /// No result was produced; the request was handed upward for review.
    Escalated,
    /// No result was produced; the request was rejected with a typed error.
    Denied,
}

impl Disposition {
    /// Parses a wire disposition string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<Disposition> {
        match value {
            "resolved" => Some(Disposition::Resolved),
            "abstained" => Some(Disposition::Abstained),
            "escalated" => Some(Disposition::Escalated),
            "denied" => Some(Disposition::Denied),
            _ => None,
        }
    }
}

/// Backend class that produced the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    /// Deterministic rule or schema-level backend.
    Deterministic,
    /// Registry or index lookup.
    Index,
    /// Local semantic backend.
    Local,
    /// Remote backend.
    Remote,
}

impl Backend {
    /// Parses a wire backend string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<Backend> {
        match value {
            "deterministic" => Some(Backend::Deterministic),
            "index" => Some(Backend::Index),
            "local" => Some(Backend::Local),
            "remote" => Some(Backend::Remote),
            _ => None,
        }
    }
}

/// Score scale fixed by protocol v1. The unit interval is the only defined
/// scale; other scales require a protocol version change.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ScoreScale {
    /// Score and threshold in the closed interval `[0, 1]`.
    #[serde(rename = "unit_interval")]
    UnitInterval,
}

/// Verdict value for the `verdict` result kind. The `verdict` kind is kept
/// distinct from a three-option `choice` because its evidence and safety
/// obligations differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictValue {
    /// The claim is supported by eligible evidence.
    Supported,
    /// The claim is contradicted by eligible evidence.
    Unsupported,
    /// No eligible evidence supports or contradicts the claim.
    Insufficient,
}

impl VerdictValue {
    /// Parses a wire verdict string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<VerdictValue> {
        match value {
            "supported" => Some(VerdictValue::Supported),
            "unsupported" => Some(VerdictValue::Unsupported),
            "insufficient" => Some(VerdictValue::Insufficient),
            _ => None,
        }
    }
}

/// One extracted field: a bounded scalar name/value pair linked to source
/// evidence.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ExtractedField {
    /// Field name.
    pub name: String,
    /// Extracted scalar value (string, number, boolean, or null).
    pub value: Value,
    /// Source evidence reference for this field.
    pub evidence: EvidenceRef,
}

/// Typed result payload. The four result kinds remain distinct; dispositions
/// never masquerade as results.
#[derive(Debug, Clone, PartialEq)]
pub enum DecisionResult {
    /// One or more identifiers selected from the supplied candidates.
    Choice {
        /// Selected candidate identifiers.
        choices: Vec<String>,
    },
    /// Bounded numeric score on the fixed unit-interval scale with its
    /// decision threshold.
    Score {
        /// Score in `[0, 1]`.
        score: f64,
        /// Defined scale, fixed to the unit interval in v1.
        scale: ScoreScale,
        /// Threshold in `[0, 1]`.
        threshold: f64,
    },
    /// Schema-bounded fields, each linked to source evidence.
    Extract {
        /// Extracted fields.
        fields: Vec<ExtractedField>,
    },
    /// `supported`, `unsupported`, or `insufficient`, with evidence
    /// references carried by the response.
    Verdict {
        /// The verdict value.
        verdict: VerdictValue,
    },
}

#[derive(Serialize)]
struct ChoiceValue<'a> {
    choices: &'a [String],
}

#[derive(Serialize)]
struct ScoreValue {
    scale: ScoreScale,
    score: f64,
    threshold: f64,
}

#[derive(Serialize)]
struct ExtractValue<'a> {
    fields: &'a [ExtractedField],
}

#[derive(Serialize)]
struct VerdictValueWire {
    verdict: VerdictValue,
}

impl Serialize for DecisionResult {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            DecisionResult::Choice { choices } => {
                map.serialize_entry("kind", "choice")?;
                map.serialize_entry("value", &ChoiceValue { choices })?;
            }
            DecisionResult::Score {
                score,
                scale,
                threshold,
            } => {
                map.serialize_entry("kind", "score")?;
                map.serialize_entry(
                    "value",
                    &ScoreValue {
                        scale: *scale,
                        score: *score,
                        threshold: *threshold,
                    },
                )?;
            }
            DecisionResult::Extract { fields } => {
                map.serialize_entry("kind", "extract")?;
                map.serialize_entry("value", &ExtractValue { fields })?;
            }
            DecisionResult::Verdict { verdict } => {
                map.serialize_entry("kind", "verdict")?;
                map.serialize_entry("value", &VerdictValueWire { verdict: *verdict })?;
            }
        }
        map.end()
    }
}

impl DecisionResult {
    /// Returns the result kind discriminator for this payload.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            DecisionResult::Choice { .. } => "choice",
            DecisionResult::Score { .. } => "score",
            DecisionResult::Extract { .. } => "extract",
            DecisionResult::Verdict { .. } => "verdict",
        }
    }

    fn from_value(value: &Value) -> Result<DecisionResult, ProtocolError> {
        let object = object_fields(value, RESULT_FIELDS, RESULT_FIELDS)?;
        let kind_raw = str_field(object, "kind", 1, 16)?;
        let value_object = object
            .get("value")
            .ok_or(ProtocolError::MissingField { field: "value" })?
            .as_object()
            .ok_or(ProtocolError::MalformedJson {
                reason: "result value must be a JSON object",
            })?;
        match kind_raw.as_str() {
            "choice" => {
                let choices_object =
                    object_fields_map(value_object, CHOICE_VALUE_FIELDS, CHOICE_VALUE_FIELDS)?;
                let items = array_field(choices_object, "choices")?;
                if items.is_empty() || items.len() > MAX_RESULT_CHOICES {
                    return Err(ProtocolError::InvalidResult {
                        reason: "choice count out of bounds",
                    });
                }
                let mut choices = Vec::with_capacity(items.len());
                for item in items {
                    let text = item.as_str().ok_or(ProtocolError::InvalidResult {
                        reason: "choice must be a JSON string",
                    })?;
                    let len = text.chars().count();
                    if !(1..=256).contains(&len) {
                        return Err(ProtocolError::InvalidResult {
                            reason: "choice length out of range",
                        });
                    }
                    if choices.iter().any(|choice| choice == text) {
                        return Err(ProtocolError::InvalidResult {
                            reason: "choice values must be unique",
                        });
                    }
                    choices.push(text.to_owned());
                }
                Ok(DecisionResult::Choice { choices })
            }
            "score" => {
                let score_object =
                    object_fields_map(value_object, SCORE_VALUE_FIELDS, SCORE_VALUE_FIELDS)?;
                let scale_raw = str_field(score_object, "scale", 1, 32)?;
                if scale_raw != "unit_interval" {
                    return Err(ProtocolError::InvalidResult {
                        reason: "unknown score scale",
                    });
                }
                let score = Self::bounded_score(score_object, "score")?;
                let threshold = Self::bounded_score(score_object, "threshold")?;
                Ok(DecisionResult::Score {
                    score,
                    scale: ScoreScale::UnitInterval,
                    threshold,
                })
            }
            "extract" => {
                let extract_object =
                    object_fields_map(value_object, EXTRACT_VALUE_FIELDS, EXTRACT_VALUE_FIELDS)?;
                let items = array_field(extract_object, "fields")?;
                if items.is_empty() || items.len() > MAX_RESULT_FIELDS {
                    return Err(ProtocolError::InvalidResult {
                        reason: "extracted field count out of bounds",
                    });
                }
                let mut fields = Vec::with_capacity(items.len());
                for item in items {
                    fields.push(Self::extracted_field(item)?);
                }
                Ok(DecisionResult::Extract { fields })
            }
            "verdict" => {
                let verdict_object =
                    object_fields_map(value_object, VERDICT_VALUE_FIELDS, VERDICT_VALUE_FIELDS)?;
                let verdict_raw = str_field(verdict_object, "verdict", 1, 16)?;
                let verdict =
                    VerdictValue::parse(&verdict_raw).ok_or(ProtocolError::InvalidResult {
                        reason: "unknown verdict value",
                    })?;
                Ok(DecisionResult::Verdict { verdict })
            }
            _ => Err(ProtocolError::UnknownResultKind { found: kind_raw }),
        }
    }

    fn bounded_score(
        object: &serde_json::Map<String, Value>,
        field: &'static str,
    ) -> Result<f64, ProtocolError> {
        let number = f64_field(object, field)?;
        if !object.get(field).is_some_and(is_unit_interval_number) {
            return Err(ProtocolError::InvalidResult {
                reason: "score value outside the unit interval",
            });
        }
        Ok(number)
    }

    fn extracted_field(value: &Value) -> Result<ExtractedField, ProtocolError> {
        let object = object_fields(value, EXTRACTED_FIELD_FIELDS, EXTRACTED_FIELD_FIELDS)?;
        let name = str_field(object, "name", 1, MAX_FIELD_NAME_CHARS)?;
        let raw_value = object
            .get("value")
            .ok_or(ProtocolError::MissingField { field: "value" })?;
        let scalar_ok = raw_value.is_string()
            || raw_value.is_number()
            || raw_value.is_boolean()
            || raw_value.is_null();
        if !scalar_ok {
            return Err(ProtocolError::InvalidResult {
                reason: "extracted field value must be a JSON scalar",
            });
        }
        if raw_value
            .as_str()
            .is_some_and(|text| text.chars().count() > MAX_FIELD_VALUE_CHARS)
        {
            return Err(ProtocolError::InvalidResult {
                reason: "extracted field value too long",
            });
        }
        let evidence = EvidenceRef::from_value(
            object
                .get("evidence")
                .ok_or(ProtocolError::MissingField { field: "evidence" })?,
        )?;
        Ok(ExtractedField {
            name,
            value: raw_value.clone(),
            evidence,
        })
    }
}

/// Bounded raw and calibrated confidence with its calibration reference.
/// Confidence is advice for later calibration and routing; it is never
/// authorization and never evidence trust.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Confidence {
    /// Raw model confidence in `[0, 1]`.
    pub raw: f64,
    /// Calibrated confidence in `[0, 1]`.
    pub calibrated: f64,
    /// Calibration policy version reference.
    pub calibration_ref: String,
}

/// Wire error object carried by a denied response. Codes mirror the typed
/// Rust errors, plus the wire-only `denied_by_policy`. An unsupported
/// protocol version cannot be represented inside a v1 envelope, so no such
/// code exists on the wire; that failure is reported by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireError {
    /// Typed error code.
    pub code: WireErrorCode,
    /// Bounded human-readable message with no secrets or reasoning content.
    pub message: String,
}

/// Typed wire error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WireErrorCode {
    /// Unknown operation kind.
    UnknownOperation,
    /// Unknown worker kind.
    UnknownWorkerKind,
    /// Unknown result kind.
    UnknownResultKind,
    /// Invalid disposition.
    InvalidDisposition,
    /// Unknown evidence trust tier.
    UnknownTrustTier,
    /// Unknown backend class.
    UnknownBackend,
    /// Confidence out of bounds.
    InvalidConfidence,
    /// Budget violation.
    InvalidBudget,
    /// Deadline violation.
    InvalidDeadline,
    /// Malformed identifier.
    InvalidId,
    /// Missing required field.
    MissingField,
    /// Unknown field present.
    UnknownField,
    /// Invalid evidence reference.
    InvalidEvidenceReference,
    /// Envelope over its declared budget.
    OverBudget,
    /// Internally inconsistent envelope.
    InconsistentEnvelope,
    /// Invalid result payload.
    InvalidResult,
    /// Malformed JSON shape.
    MalformedJson,
    /// Denied by policy (wire-only code for fail-closed policy rejection).
    DeniedByPolicy,
}

impl WireErrorCode {
    /// Parses a wire error code string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<WireErrorCode> {
        match value {
            "unknown_operation" => Some(WireErrorCode::UnknownOperation),
            "unknown_worker_kind" => Some(WireErrorCode::UnknownWorkerKind),
            "unknown_result_kind" => Some(WireErrorCode::UnknownResultKind),
            "invalid_disposition" => Some(WireErrorCode::InvalidDisposition),
            "unknown_trust_tier" => Some(WireErrorCode::UnknownTrustTier),
            "unknown_backend" => Some(WireErrorCode::UnknownBackend),
            "invalid_confidence" => Some(WireErrorCode::InvalidConfidence),
            "invalid_budget" => Some(WireErrorCode::InvalidBudget),
            "invalid_deadline" => Some(WireErrorCode::InvalidDeadline),
            "invalid_id" => Some(WireErrorCode::InvalidId),
            "missing_field" => Some(WireErrorCode::MissingField),
            "unknown_field" => Some(WireErrorCode::UnknownField),
            "invalid_evidence_reference" => Some(WireErrorCode::InvalidEvidenceReference),
            "over_budget" => Some(WireErrorCode::OverBudget),
            "inconsistent_envelope" => Some(WireErrorCode::InconsistentEnvelope),
            "invalid_result" => Some(WireErrorCode::InvalidResult),
            "malformed_json" => Some(WireErrorCode::MalformedJson),
            "denied_by_policy" => Some(WireErrorCode::DeniedByPolicy),
            _ => None,
        }
    }
}

/// Queue, load, and inference latency accounting in milliseconds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Latency {
    /// Queue wait in milliseconds.
    pub queue_ms: u64,
    /// Load time in milliseconds.
    pub load_ms: u64,
    /// Inference time in milliseconds.
    pub inference_ms: u64,
}

/// Version 1 response envelope. Responses carry typed results or typed
/// errors, bounded confidence, and latency accounting; they never carry
/// authorization, secrets, hidden reasoning, or unrestricted transcripts.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Response {
    /// Protocol version.
    pub protocol_version: ProtocolVersion,
    /// Canonical lowercase identifier of the answered request.
    pub request_id: String,
    /// Typed result payload; present exactly when the disposition requires
    /// it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<DecisionResult>,
    /// Typed error payload; present exactly when the disposition requires
    /// it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<WireError>,
    /// Bounded confidence.
    pub confidence: Confidence,
    /// Disposition of the decision.
    pub disposition: Disposition,
    /// Optional bounded uncertainty note, or `null`.
    pub uncertainty: Option<String>,
    /// Evidence references supporting the response; every entry must be
    /// present in the answered request.
    pub evidence_refs: Vec<EvidenceRef>,
    /// Backend class that produced the response.
    pub backend: Backend,
    /// Latency accounting.
    pub latency: Latency,
    /// Canonical lowercase trace identifier.
    pub trace_id: String,
}

impl Response {
    /// Parses and validates a response envelope from JSON text, failing
    /// closed with typed errors. Cross-envelope rules are checked separately
    /// by [`Response::validate_against`].
    pub fn from_json_str(text: &str) -> Result<Response, ProtocolError> {
        let value: Value =
            serde_json::from_str(text).map_err(|_| ProtocolError::MalformedJson {
                reason: "invalid JSON text",
            })?;
        Response::from_value(&value)
    }

    /// Parses and validates a response envelope from a JSON value, failing
    /// closed with typed errors. Cross-envelope rules are checked separately
    /// by [`Response::validate_against`].
    pub fn from_value(value: &Value) -> Result<Response, ProtocolError> {
        let object = object_fields(value, RESPONSE_KNOWN, RESPONSE_REQUIRED)?;

        let version = str_field(object, "protocol_version", 1, 8)?;
        if version != "1" {
            return Err(ProtocolError::UnsupportedVersion { found: version });
        }

        let request_id = str_field(object, "request_id", 1, MAX_ID_CHARS)?;
        if !is_uuid(&request_id) {
            return Err(ProtocolError::InvalidId {
                field: "request_id",
                found: request_id,
            });
        }

        let trace_id = str_field(object, "trace_id", 1, MAX_ID_CHARS)?;
        if !is_uuid(&trace_id) {
            return Err(ProtocolError::InvalidId {
                field: "trace_id",
                found: trace_id,
            });
        }

        let disposition_raw = str_field(object, "disposition", 1, 16)?;
        let disposition = Disposition::parse(&disposition_raw).ok_or_else(|| {
            ProtocolError::InvalidDisposition {
                found: disposition_raw.clone(),
            }
        })?;

        let has_result = object.contains_key("result");
        let has_error = object.contains_key("error");
        match disposition {
            Disposition::Resolved => {
                if !has_result {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "resolved response requires a result".to_owned(),
                    });
                }
                if has_error {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "resolved response must not carry an error".to_owned(),
                    });
                }
            }
            Disposition::Abstained | Disposition::Escalated => {
                if has_result || has_error {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason:
                            "abstained and escalated responses must not carry a result or error"
                                .to_owned(),
                    });
                }
            }
            Disposition::Denied => {
                if !has_error {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "denied response requires an error".to_owned(),
                    });
                }
                if has_result {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "denied response must not carry a result".to_owned(),
                    });
                }
            }
        }

        let result = if has_result {
            Some(DecisionResult::from_value(
                object
                    .get("result")
                    .ok_or(ProtocolError::MissingField { field: "result" })?,
            )?)
        } else {
            None
        };

        let error = if has_error {
            Some(Self::wire_error(
                object
                    .get("error")
                    .ok_or(ProtocolError::MissingField { field: "error" })?,
            )?)
        } else {
            None
        };

        let confidence_value = object
            .get("confidence")
            .ok_or(ProtocolError::MissingField {
                field: "confidence",
            })?;
        let confidence = Self::confidence(confidence_value)?;

        let uncertainty = optional_str_field(object, "uncertainty", MAX_UNCERTAINTY_CHARS)?;

        let evidence_items = array_field(object, "evidence_refs")?;
        if evidence_items.len() > MAX_RESPONSE_EVIDENCE_REFS {
            return Err(ProtocolError::MalformedJson {
                reason: "too many evidence references",
            });
        }
        let mut evidence_refs = Vec::with_capacity(evidence_items.len());
        for item in evidence_items {
            let reference = EvidenceRef::from_value(item)?;
            if evidence_refs.contains(&reference) {
                return Err(ProtocolError::InvalidEvidenceReference {
                    reason: "duplicate evidence reference",
                });
            }
            evidence_refs.push(reference);
        }

        let backend_raw = str_field(object, "backend", 1, 16)?;
        let backend =
            Backend::parse(&backend_raw).ok_or_else(|| ProtocolError::UnknownBackend {
                found: backend_raw.clone(),
            })?;

        let latency_value = object
            .get("latency")
            .ok_or(ProtocolError::MissingField { field: "latency" })?;
        let latency_object = object_fields(latency_value, LATENCY_FIELDS, LATENCY_FIELDS)?;
        let latency = Latency {
            queue_ms: uint_field(latency_object, "queue_ms")?,
            load_ms: uint_field(latency_object, "load_ms")?,
            inference_ms: uint_field(latency_object, "inference_ms")?,
        };
        if latency.queue_ms > MAX_LATENCY_MS
            || latency.load_ms > MAX_LATENCY_MS
            || latency.inference_ms > MAX_LATENCY_MS
        {
            return Err(ProtocolError::InvalidDeadline {
                reason: "latency component out of bounds",
            });
        }

        Ok(Response {
            protocol_version: crate::request::ProtocolVersion::V1,
            request_id,
            result,
            error,
            confidence,
            disposition,
            uncertainty,
            evidence_refs,
            backend,
            latency,
            trace_id,
        })
    }

    fn confidence(value: &Value) -> Result<Confidence, ProtocolError> {
        let object = object_fields(value, CONFIDENCE_FIELDS, CONFIDENCE_FIELDS)?;
        let raw = f64_field(object, "raw")?;
        if !object.get("raw").is_some_and(is_unit_interval_number) {
            return Err(ProtocolError::InvalidConfidence {
                field: "raw",
                found: format!("{raw}"),
            });
        }
        let calibrated = f64_field(object, "calibrated")?;
        if !object
            .get("calibrated")
            .is_some_and(is_unit_interval_number)
        {
            return Err(ProtocolError::InvalidConfidence {
                field: "calibrated",
                found: format!("{calibrated}"),
            });
        }
        let calibration_ref = str_field(object, "calibration_ref", 1, 256)?;
        Ok(Confidence {
            raw,
            calibrated,
            calibration_ref,
        })
    }

    fn wire_error(value: &Value) -> Result<WireError, ProtocolError> {
        let object = object_fields(value, ERROR_FIELDS, ERROR_FIELDS)?;
        let code_raw = str_field(object, "code", 1, 64)?;
        let code = WireErrorCode::parse(&code_raw).ok_or(ProtocolError::MalformedJson {
            reason: "unknown wire error code",
        })?;
        let message = str_field(object, "message", 1, MAX_ERROR_MESSAGE_CHARS)?;
        Ok(WireError { code, message })
    }

    /// Cross-envelope validation against the answered request: request
    /// linkage, operation/result-kind compatibility, evidence containment,
    /// candidate containment for choice results, evidence linkage for
    /// extract fields, verdict evidence obligations, and latency/deadline
    /// accounting. These rules cannot be expressed in a standalone JSON
    /// Schema document.
    pub fn validate_against(&self, request: &Request) -> Result<(), ProtocolError> {
        if self.request_id != request.request_id {
            return Err(ProtocolError::InconsistentEnvelope {
                reason: "response request_id does not match the request".to_owned(),
            });
        }
        let operation_matches_result = matches!(
            (request.operation, &self.result),
            (Operation::Select, Some(DecisionResult::Choice { .. }))
                | (Operation::Score, Some(DecisionResult::Score { .. }))
                | (Operation::Extract, Some(DecisionResult::Extract { .. }))
                | (Operation::Verify, Some(DecisionResult::Verdict { .. }))
                | (_, None)
        );
        if !operation_matches_result {
            return Err(ProtocolError::InconsistentEnvelope {
                reason: "response result kind does not match the request operation".to_owned(),
            });
        }
        for reference in &self.evidence_refs {
            if !request.evidence.contains(reference) {
                return Err(ProtocolError::InvalidEvidenceReference {
                    reason: "response evidence reference is not present in the request",
                });
            }
        }
        match &self.result {
            Some(DecisionResult::Choice { choices }) => {
                if choices.len() as u64 > request.constraints.max_choices {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "choice count exceeds the request max_choices".to_owned(),
                    });
                }
                for choice in choices {
                    if !request.candidates.contains(choice) {
                        return Err(ProtocolError::InconsistentEnvelope {
                            reason: "choice is not among the supplied candidates".to_owned(),
                        });
                    }
                }
            }
            Some(DecisionResult::Extract { fields }) => {
                for field in fields {
                    if !request.evidence.contains(&field.evidence) {
                        return Err(ProtocolError::InvalidEvidenceReference {
                            reason: "extracted field evidence is not present in the request",
                        });
                    }
                }
            }
            Some(DecisionResult::Verdict { verdict }) => {
                if matches!(verdict, VerdictValue::Supported | VerdictValue::Unsupported)
                    && self.evidence_refs.is_empty()
                {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "supported and unsupported verdicts require evidence references"
                            .to_owned(),
                    });
                }
            }
            Some(DecisionResult::Score { .. }) | None => {}
        }
        if self.latency.queue_ms > request.deadline.queue_ms {
            return Err(ProtocolError::InvalidDeadline {
                reason: "response queue latency exceeds the request queue deadline",
            });
        }
        let total = self.latency.queue_ms + self.latency.load_ms + self.latency.inference_ms;
        let window = request.deadline.queue_ms + request.deadline.execution_ms;
        if total > window {
            return Err(ProtocolError::InvalidDeadline {
                reason: "total response latency exceeds the request deadline envelope",
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{DecisionResult, Response, VerdictValue};

    #[test]
    fn result_kind_discriminators_are_stable() {
        assert_eq!(
            DecisionResult::Verdict {
                verdict: VerdictValue::Insufficient
            }
            .kind(),
            "verdict"
        );
    }

    #[test]
    fn result_serializes_with_kind_and_value() {
        let result = DecisionResult::Verdict {
            verdict: VerdictValue::Insufficient,
        };
        let value = serde_json::to_value(&result).expect("serialize result");
        assert_eq!(value["kind"], json!("verdict"));
        assert_eq!(value["value"]["verdict"], json!("insufficient"));
        assert_eq!(value.as_object().map(|object| object.len()), Some(2));
    }

    #[test]
    fn denied_and_resolved_presence_rules_fail_closed() {
        let text = r#"{
            "protocol_version": "1",
            "request_id": "22222222-2222-4222-8222-222222222222",
            "confidence": { "raw": 0.5, "calibrated": 0.5, "calibration_ref": "c" },
            "disposition": "denied",
            "uncertainty": null,
            "evidence_refs": [],
            "backend": "deterministic",
            "latency": { "queue_ms": 1, "load_ms": 0, "inference_ms": 0 },
            "trace_id": "aaaaaaaa-0008-4000-8000-000000000008"
        }"#;
        let error = Response::from_json_str(text).expect_err("denied without error must fail");
        assert!(matches!(
            error,
            crate::error::ProtocolError::InconsistentEnvelope { .. }
        ));
    }
}
