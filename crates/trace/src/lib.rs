//! Sanitized, versioned traces for protocol-level decisions.
//!
//! Trace records contain typed metadata and content hashes. They omit request
//! text, evidence source labels, result bodies, uncertainty text, and error
//! messages. Replay accepts the original request JSON from the caller, checks
//! it through zhoor-protocol, and verifies its normalized hash before return.

#![forbid(unsafe_code)]

use std::fmt;
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zhoor_protocol::{
    Backend, DecisionResult, Disposition, EvidenceKind, EvidenceRef, Operation, ProtocolVersion,
    Request, Response, TrustTier, WireErrorCode, WorkerKind,
};

/// Current serialized trace format.
pub const TRACE_VERSION: &str = "1";
const INPUT_HASH_DOMAIN: &[u8] = b"zhoor-trace-input-v1\0";
const RESPONSE_HASH_DOMAIN: &[u8] = b"zhoor-trace-response-v1\0";
const MAX_LABEL: usize = 128;

/// Caller-supplied runtime versions and receipt metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceContext {
    pub worker_version: String,
    pub model_version: Option<String>,
    pub registry_version: Option<String>,
    pub recorded_at_unix_ms: u64,
    pub receipt_ref: Option<String>,
}

/// Sanitized content-addressed evidence metadata. The source label is omitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraceEvidenceRef {
    pub reference: String,
    pub kind: TraceEvidenceKind,
    pub trust_tier: TraceTrustTier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceEvidenceKind {
    UserInput,
    Registry,
    ToolResult,
    Memory,
    ExternalDocument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceTrustTier {
    Untrusted,
    Observed,
    Verified,
    Policy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceOperation {
    Select,
    Score,
    Extract,
    Verify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceWorkerKind {
    SkillRouter,
    RiskJudge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceRoute {
    Deterministic,
    Index,
    Local,
    Remote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceDisposition {
    Resolved,
    Abstained,
    Escalated,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceResultKind {
    Choice,
    Score,
    Extract,
    Verdict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceErrorCode {
    UnknownOperation,
    UnknownWorkerKind,
    UnknownResultKind,
    InvalidDisposition,
    UnknownTrustTier,
    UnknownBackend,
    InvalidConfidence,
    InvalidBudget,
    InvalidDeadline,
    InvalidId,
    MissingField,
    UnknownField,
    InvalidEvidenceReference,
    OverBudget,
    InconsistentEnvelope,
    InvalidResult,
    MalformedJson,
    DeniedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraceConfidence {
    pub raw: f64,
    pub calibrated: f64,
    pub calibration_ref_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraceLatency {
    pub queue_ms: u64,
    pub load_ms: u64,
    pub inference_ms: u64,
}

/// Privacy-preserving protocol decision record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionTrace {
    pub trace_version: String,
    pub trace_id: String,
    pub request_id: String,
    pub protocol_version: String,
    pub operation: TraceOperation,
    pub worker_kind: TraceWorkerKind,
    pub worker_version: String,
    pub model_version: Option<String>,
    pub policy_version_hash: String,
    pub registry_version: Option<String>,
    pub input_hash: String,
    pub response_hash: String,
    pub route: TraceRoute,
    pub result_kind: Option<TraceResultKind>,
    pub confidence: TraceConfidence,
    pub disposition: TraceDisposition,
    pub evidence_refs: Vec<TraceEvidenceRef>,
    pub latency: TraceLatency,
    pub error_code: Option<TraceErrorCode>,
    pub receipt_ref: Option<String>,
    pub recorded_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceError {
    InvalidProtocolEnvelope,
    InvalidContext,
    InvalidTrace,
    UnsupportedTraceVersion,
    ReplayMismatch,
    SerializationFailure,
}

impl fmt::Display for TraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidProtocolEnvelope => "protocol envelope is invalid",
            Self::InvalidContext => "trace context is invalid",
            Self::InvalidTrace => "trace record is invalid",
            Self::UnsupportedTraceVersion => "trace version is unsupported",
            Self::ReplayMismatch => "replay input does not match trace identity or hash",
            Self::SerializationFailure => "trace serialization failed",
        })
    }
}

impl std::error::Error for TraceError {}

impl DecisionTrace {
    /// Creates a trace after checking request/response consistency.
    ///
    /// Hashes cover canonical typed protocol serialization. Free-form request
    /// and response content is never copied into the trace.
    pub fn create(
        request: &Request,
        response: &Response,
        context: TraceContext,
    ) -> Result<Self, TraceError> {
        // Protocol structs expose public fields, so callers can construct or
        // mutate values without going through their validating JSON parsers.
        let request_value =
            serde_json::to_value(request).map_err(|_| TraceError::InvalidProtocolEnvelope)?;
        let request =
            Request::from_value(&request_value).map_err(|_| TraceError::InvalidProtocolEnvelope)?;
        let response_value =
            serde_json::to_value(response).map_err(|_| TraceError::InvalidProtocolEnvelope)?;
        let response = Response::from_value(&response_value)
            .map_err(|_| TraceError::InvalidProtocolEnvelope)?;
        response
            .validate_against(&request)
            .map_err(|_| TraceError::InvalidProtocolEnvelope)?;
        validate_context(&context)?;

        let calibration_ref_hash = digest_bytes(
            b"zhoor-trace-calibration-ref-v1\0",
            response.confidence.calibration_ref.as_bytes(),
        );
        let trace = Self {
            trace_version: TRACE_VERSION.to_owned(),
            trace_id: response.trace_id.clone(),
            request_id: request.request_id.clone(),
            protocol_version: protocol_version(request.protocol_version).to_owned(),
            operation: request.operation.into(),
            worker_kind: request.worker_kind.into(),
            worker_version: context.worker_version,
            model_version: context.model_version,
            policy_version_hash: digest_bytes(
                b"zhoor-trace-policy-version-v1\0",
                request.policy_ref.as_bytes(),
            ),
            registry_version: context.registry_version,
            input_hash: digest(INPUT_HASH_DOMAIN, &request)?,
            response_hash: digest(RESPONSE_HASH_DOMAIN, &response)?,
            route: response.backend.into(),
            result_kind: response.result.as_ref().map(result_kind),
            confidence: TraceConfidence {
                raw: response.confidence.raw,
                calibrated: response.confidence.calibrated,
                calibration_ref_hash,
            },
            disposition: response.disposition.into(),
            evidence_refs: response
                .evidence_refs
                .iter()
                .map(TraceEvidenceRef::from)
                .collect(),
            latency: TraceLatency {
                queue_ms: response.latency.queue_ms,
                load_ms: response.latency.load_ms,
                inference_ms: response.latency.inference_ms,
            },
            error_code: response.error.as_ref().map(|error| error_code(error.code)),
            receipt_ref: context.receipt_ref,
            recorded_at_unix_ms: context.recorded_at_unix_ms,
        };
        trace.validate()?;
        Ok(trace)
    }

    pub fn to_json_string(&self) -> Result<String, TraceError> {
        self.validate()?;
        serde_json::to_string(self).map_err(|_| TraceError::SerializationFailure)
    }

    pub fn from_json_str(text: &str) -> Result<Self, TraceError> {
        // Preserve the original decimal spelling long enough to reject values
        // outside [0, 1] before conversion to f64 can round them onto a bound.
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|_| TraceError::InvalidTrace)?;
        if !required_nullable_fields_present(&value)
            || !json_confidence_in_unit_interval(&value, "raw")
            || !json_confidence_in_unit_interval(&value, "calibrated")
        {
            return Err(TraceError::InvalidTrace);
        }
        let trace: Self = serde_json::from_value(value).map_err(|_| TraceError::InvalidTrace)?;
        trace.validate()?;
        Ok(trace)
    }

    /// Reconstructs a normalized request from caller-supplied replay JSON.
    /// The source input remains outside the persisted trace by default.
    pub fn reconstruct_replay_input(
        &self,
        replay_request_json: &str,
    ) -> Result<Request, TraceError> {
        self.validate()?;
        let request = Request::from_json_str(replay_request_json)
            .map_err(|_| TraceError::InvalidProtocolEnvelope)?;
        if protocol_version(request.protocol_version) != self.protocol_version
            || request.request_id != self.request_id
            || TraceOperation::from(request.operation) != self.operation
            || TraceWorkerKind::from(request.worker_kind) != self.worker_kind
            || digest_bytes(
                b"zhoor-trace-policy-version-v1\0",
                request.policy_ref.as_bytes(),
            ) != self.policy_version_hash
            || digest(INPUT_HASH_DOMAIN, &request)? != self.input_hash
        {
            return Err(TraceError::ReplayMismatch);
        }
        Ok(request)
    }

    pub fn validate(&self) -> Result<(), TraceError> {
        if self.trace_version != TRACE_VERSION {
            return Err(TraceError::UnsupportedTraceVersion);
        }
        if self.protocol_version != "1"
            || !is_uuid(&self.trace_id)
            || !is_uuid(&self.request_id)
            || !is_hash(&self.input_hash)
            || !is_hash(&self.response_hash)
            || !valid_token(&self.worker_version)
            || !is_hash(&self.policy_version_hash)
            || !optional_valid_token(self.model_version.as_deref())
            || !optional_valid_token(self.registry_version.as_deref())
            || !valid_confidence(self.confidence.raw)
            || !valid_confidence(self.confidence.calibrated)
            || !is_hash(&self.confidence.calibration_ref_hash)
            || self.evidence_refs.len() > 64
            || self.latency.queue_ms > 600_000
            || self.latency.load_ms > 600_000
            || self.latency.inference_ms > 600_000
            || (self.disposition == TraceDisposition::Resolved) != self.result_kind.is_some()
            || (self.disposition == TraceDisposition::Denied) != self.error_code.is_some()
            || self
                .result_kind
                .is_some_and(|kind| !operation_matches_result(self.operation, kind))
            || !optional_valid_receipt(self.receipt_ref.as_deref())
        {
            return Err(TraceError::InvalidTrace);
        }
        for evidence in &self.evidence_refs {
            if !EvidenceRef::is_canonical_reference(&evidence.reference) {
                return Err(TraceError::InvalidTrace);
            }
        }
        Ok(())
    }
}

impl From<Operation> for TraceOperation {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Select => Self::Select,
            Operation::Score => Self::Score,
            Operation::Extract => Self::Extract,
            Operation::Verify => Self::Verify,
        }
    }
}

impl From<WorkerKind> for TraceWorkerKind {
    fn from(value: WorkerKind) -> Self {
        match value {
            WorkerKind::SkillRouter => Self::SkillRouter,
            WorkerKind::RiskJudge => Self::RiskJudge,
        }
    }
}

impl From<Backend> for TraceRoute {
    fn from(value: Backend) -> Self {
        match value {
            Backend::Deterministic => Self::Deterministic,
            Backend::Index => Self::Index,
            Backend::Local => Self::Local,
            Backend::Remote => Self::Remote,
        }
    }
}

impl From<Disposition> for TraceDisposition {
    fn from(value: Disposition) -> Self {
        match value {
            Disposition::Resolved => Self::Resolved,
            Disposition::Abstained => Self::Abstained,
            Disposition::Escalated => Self::Escalated,
            Disposition::Denied => Self::Denied,
        }
    }
}

impl From<&EvidenceRef> for TraceEvidenceRef {
    fn from(value: &EvidenceRef) -> Self {
        Self {
            reference: value.reference.clone(),
            kind: value.kind.into(),
            trust_tier: value.trust_tier.into(),
        }
    }
}

impl From<EvidenceKind> for TraceEvidenceKind {
    fn from(value: EvidenceKind) -> Self {
        match value {
            EvidenceKind::UserInput => Self::UserInput,
            EvidenceKind::Registry => Self::Registry,
            EvidenceKind::ToolResult => Self::ToolResult,
            EvidenceKind::Memory => Self::Memory,
            EvidenceKind::ExternalDocument => Self::ExternalDocument,
        }
    }
}

impl From<TrustTier> for TraceTrustTier {
    fn from(value: TrustTier) -> Self {
        match value {
            TrustTier::Untrusted => Self::Untrusted,
            TrustTier::Observed => Self::Observed,
            TrustTier::Verified => Self::Verified,
            TrustTier::Policy => Self::Policy,
        }
    }
}

fn result_kind(result: &DecisionResult) -> TraceResultKind {
    match result {
        DecisionResult::Choice { .. } => TraceResultKind::Choice,
        DecisionResult::Score { .. } => TraceResultKind::Score,
        DecisionResult::Extract { .. } => TraceResultKind::Extract,
        DecisionResult::Verdict { .. } => TraceResultKind::Verdict,
    }
}

fn operation_matches_result(operation: TraceOperation, result: TraceResultKind) -> bool {
    matches!(
        (operation, result),
        (TraceOperation::Select, TraceResultKind::Choice)
            | (TraceOperation::Score, TraceResultKind::Score)
            | (TraceOperation::Extract, TraceResultKind::Extract)
            | (TraceOperation::Verify, TraceResultKind::Verdict)
    )
}

fn error_code(code: WireErrorCode) -> TraceErrorCode {
    use WireErrorCode as P;
    match code {
        P::UnknownOperation => TraceErrorCode::UnknownOperation,
        P::UnknownWorkerKind => TraceErrorCode::UnknownWorkerKind,
        P::UnknownResultKind => TraceErrorCode::UnknownResultKind,
        P::InvalidDisposition => TraceErrorCode::InvalidDisposition,
        P::UnknownTrustTier => TraceErrorCode::UnknownTrustTier,
        P::UnknownBackend => TraceErrorCode::UnknownBackend,
        P::InvalidConfidence => TraceErrorCode::InvalidConfidence,
        P::InvalidBudget => TraceErrorCode::InvalidBudget,
        P::InvalidDeadline => TraceErrorCode::InvalidDeadline,
        P::InvalidId => TraceErrorCode::InvalidId,
        P::MissingField => TraceErrorCode::MissingField,
        P::UnknownField => TraceErrorCode::UnknownField,
        P::InvalidEvidenceReference => TraceErrorCode::InvalidEvidenceReference,
        P::OverBudget => TraceErrorCode::OverBudget,
        P::InconsistentEnvelope => TraceErrorCode::InconsistentEnvelope,
        P::InvalidResult => TraceErrorCode::InvalidResult,
        P::MalformedJson => TraceErrorCode::MalformedJson,
        P::DeniedByPolicy => TraceErrorCode::DeniedByPolicy,
    }
}

fn protocol_version(value: ProtocolVersion) -> &'static str {
    match value {
        ProtocolVersion::V1 => "1",
    }
}

fn validate_context(context: &TraceContext) -> Result<(), TraceError> {
    if !valid_token(&context.worker_version)
        || !optional_valid_token(context.model_version.as_deref())
        || !optional_valid_token(context.registry_version.as_deref())
        || !optional_valid_receipt(context.receipt_ref.as_deref())
    {
        return Err(TraceError::InvalidContext);
    }
    Ok(())
}

fn digest<T: Serialize>(domain: &[u8], value: &T) -> Result<String, TraceError> {
    let normalized = serde_json::to_vec(value).map_err(|_| TraceError::SerializationFailure)?;
    Ok(digest_bytes(domain, &normalized))
}

fn digest_bytes(domain: &[u8], bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    let bytes = hasher.finalize();
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut output, "{byte:02x}").expect("String formatting cannot fail");
    }
    output
}

fn valid_token(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= MAX_LABEL
        && bytes[0].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
}

fn optional_valid_token(value: Option<&str>) -> bool {
    value.is_none_or(valid_token)
}

fn is_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && [8, 13, 18, 23].iter().all(|index| bytes[*index] == b'-')
        && bytes.iter().enumerate().all(|(index, byte)| {
            [8, 13, 18, 23].contains(&index)
                || (byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
}

fn is_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_confidence(value: f64) -> bool {
    value.is_finite() && (0.0..=1.0).contains(&value)
}

fn json_confidence_in_unit_interval(value: &serde_json::Value, field: &str) -> bool {
    value
        .get("confidence")
        .and_then(|confidence| confidence.get(field))
        .and_then(serde_json::Value::as_number)
        .is_some_and(|number| decimal_in_unit_interval(&number.to_string()))
}

fn required_nullable_fields_present(value: &serde_json::Value) -> bool {
    value.as_object().is_some_and(|object| {
        [
            "model_version",
            "registry_version",
            "result_kind",
            "error_code",
            "receipt_ref",
        ]
        .iter()
        .all(|field| object.contains_key(*field))
    })
}

// The input is already a syntactically valid JSON number. Compare its exact
// decimal value against 0 and 1 without converting it to a floating-point type.
fn decimal_in_unit_interval(number: &str) -> bool {
    let (negative, unsigned) = number
        .strip_prefix('-')
        .map_or((false, number), |unsigned| (true, unsigned));
    let (mantissa, exponent) = unsigned
        .find(['e', 'E'])
        .map_or((unsigned, 0_i64), |index| {
            let exponent = unsigned[index + 1..].parse::<i64>().unwrap_or_else(|_| {
                if unsigned[index + 1..].starts_with('-') {
                    i64::MIN
                } else {
                    i64::MAX
                }
            });
            (&unsigned[..index], exponent)
        });
    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{integer}{fraction}");
    let leading_zeroes = digits.bytes().take_while(|byte| *byte == b'0').count();
    if leading_zeroes == digits.len() {
        return true;
    }
    if negative {
        return false;
    }

    let decimal_position = (integer.len() as i64)
        .saturating_add(exponent)
        .saturating_sub(leading_zeroes as i64);
    if decimal_position < 1 {
        return true;
    }
    if decimal_position > 1 {
        return false;
    }

    let significant = &digits[leading_zeroes..];
    significant.as_bytes()[0] == b'1'
        && significant.as_bytes()[1..].iter().all(|byte| *byte == b'0')
}

fn optional_valid_receipt(value: Option<&str>) -> bool {
    value.is_none_or(|reference| {
        reference
            .strip_prefix("receipt://sha256/")
            .is_some_and(is_hash)
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{DecisionTrace, TraceContext, TraceError};
    use zhoor_protocol::{Request, Response};

    const REQUEST: &str = include_str!("../../../fixtures/protocol/valid/request-score.json");
    const RESPONSE: &str = include_str!("../../../fixtures/protocol/valid/response-score.json");

    fn context() -> TraceContext {
        TraceContext {
            worker_version: "risk-judge-v1".to_owned(),
            model_version: Some("model-v2".to_owned()),
            registry_version: Some("registry-v3".to_owned()),
            recorded_at_unix_ms: 1_759_000_000_000,
            receipt_ref: None,
        }
    }

    fn create_trace() -> (Request, Response, DecisionTrace) {
        let request = Request::from_json_str(REQUEST).expect("valid request fixture");
        let response = Response::from_json_str(RESPONSE).expect("valid response fixture");
        let trace = DecisionTrace::create(&request, &response, context()).expect("trace");
        (request, response, trace)
    }

    #[test]
    fn identical_normalized_inputs_have_stable_hashes() {
        let (request, response, first) = create_trace();
        let second = DecisionTrace::create(&request, &response, context()).expect("trace");
        assert_eq!(first.input_hash, second.input_hash);
        assert_eq!(first.response_hash, second.response_hash);
    }

    #[test]
    fn trace_omits_free_form_request_and_response_content() {
        let (_, _, trace) = create_trace();
        let serialized = trace.to_json_string().expect("serialize trace");
        assert!(!serialized.contains("Assess the operational risk"));
        assert!(!serialized.contains("tool:disk-usage"));
        assert!(!serialized.contains("score within the guard band"));
        assert!(!serialized.contains("0.42"));
        assert!(!serialized.contains("risk-policy-v1"));
        assert!(serialized.contains(&trace.input_hash));
        assert!(serialized.contains(&trace.response_hash));
        assert!(serialized.contains("queue_ms"));
        assert!(serialized.contains("inference_ms"));
    }

    #[test]
    fn replay_reconstructs_only_matching_protocol_input() {
        let (request, _, trace) = create_trace();
        let replay = trace
            .reconstruct_replay_input(REQUEST)
            .expect("matching request reconstructs");
        assert_eq!(replay.request_id, request.request_id);

        let changed = REQUEST.replace("operational risk", "different risk");
        assert_eq!(
            trace.reconstruct_replay_input(&changed),
            Err(TraceError::ReplayMismatch)
        );
    }

    #[test]
    fn parser_rejects_unsupported_version_and_unknown_fields() {
        let (_, _, trace) = create_trace();
        let value: serde_json::Value =
            serde_json::from_str(&trace.to_json_string().expect("serialize")).expect("JSON");
        let mut wrong_version = value.clone();
        wrong_version["trace_version"] = json!("2");
        assert_eq!(
            DecisionTrace::from_json_str(&wrong_version.to_string()),
            Err(TraceError::UnsupportedTraceVersion)
        );

        let mut unknown_field = value;
        unknown_field["hidden_reasoning"] = json!("must not be accepted");
        assert_eq!(
            DecisionTrace::from_json_str(&unknown_field.to_string()),
            Err(TraceError::InvalidTrace)
        );
    }
}
