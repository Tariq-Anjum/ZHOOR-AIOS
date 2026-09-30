//! Closed, bounded `aiosd` v1 contracts.
//!
//! This crate parses contracts and checks structural/context bindings. It
//! never verifies authenticator proof bytes, establishes trust, authorizes an
//! operation, opens a socket, reads the host, or executes a capability.

#![forbid(unsafe_code)]

use std::fmt::Write as _;

use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zhoor_protocol::{EvidenceKind, EvidenceRef, TrustTier};

pub const CONTRACT_VERSION: &str = "1";
pub const CAPABILITIES_DESCRIBE: &str = "capabilities.describe";
pub const MAX_JSON_BYTES: usize = 65_536;
pub const MAX_FRAME_PAYLOAD_BYTES: usize = 65_536;
pub const MAX_AUTHENTICATOR_PROOF_BYTES: usize = 4_096;
pub const MAX_EVIDENCE_REFS: usize = 64;
pub const MAX_REMAINING_GRANT_LIFETIME_MS: u64 = 30_000;

const TRANSCRIPT_DOMAIN: &[u8] = b"ZHOOR-AIOSD-GRANT-CLAIMS\0\x01";
const ARGUMENT_BYTES: &[u8; 2] = b"{}";

/// Canonical SHA-256 digest of the exact normalized argument bytes.
///
/// This digest binds bytes; it does not authenticate a grant or its issuer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSha256(String);

impl ArgumentSha256 {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The only v1 resource scope: static registered capability-descriptor
/// metadata. It contains no path and grants no filesystem access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceScope {
    RegisteredCapabilityDescriptorMetadata,
}

impl ResourceScope {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegisteredCapabilityDescriptorMetadata => {
                "registered_capability_descriptor_metadata"
            }
        }
    }
}

/// UID claim copied from the serialized grant.
///
/// This parsed value is untrusted until compared with a UID supplied
/// separately by the trusted Unix peer-credential layer. It does not
/// authenticate its own caller or add multi-user policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnixPeerIdentity {
    uid: u32,
}

impl UnixPeerIdentity {
    #[must_use]
    pub const fn uid(self) -> u32 {
        self.uid
    }
}

/// Mechanism-neutral authenticator envelope. `format` identifies a
/// GRANT-01-supported proof format; `proof` contains its opaque bytes encoded
/// as canonical lowercase hexadecimal. Structural validity is not proof
/// validity or issuer trust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatorEnvelope {
    format: String,
    proof: String,
}

impl AuthenticatorEnvelope {
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    #[must_use]
    pub fn proof_hex(&self) -> &str {
        &self.proof
    }
}

/// Complete claims bound by every conforming authenticator/verifier.
///
/// Fields are private so callers obtain claims only through bounded parsing.
/// A parsed value remains untrusted until a GRANT-01 verifier succeeds.
#[derive(Debug, Clone, PartialEq)]
pub struct GrantClaims {
    grant_id: String,
    issuer_id: String,
    client_identity: UnixPeerIdentity,
    capability_id: String,
    argument_sha256: ArgumentSha256,
    resource_scope: ResourceScope,
    policy_version: String,
    expires_at_unix_ms: u64,
    trace_id: String,
    decision_hash: String,
    evidence: Vec<EvidenceRef>,
    authenticator: AuthenticatorEnvelope,
}

impl GrantClaims {
    #[must_use]
    pub fn grant_id(&self) -> &str {
        &self.grant_id
    }

    #[must_use]
    pub fn issuer_id(&self) -> &str {
        &self.issuer_id
    }

    #[must_use]
    pub const fn client_identity(&self) -> UnixPeerIdentity {
        self.client_identity
    }

    #[must_use]
    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    #[must_use]
    pub fn argument_sha256(&self) -> &ArgumentSha256 {
        &self.argument_sha256
    }

    #[must_use]
    pub const fn resource_scope(&self) -> ResourceScope {
        self.resource_scope
    }

    #[must_use]
    pub fn policy_version(&self) -> &str {
        &self.policy_version
    }

    #[must_use]
    pub const fn expires_at_unix_ms(&self) -> u64 {
        self.expires_at_unix_ms
    }

    #[must_use]
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    #[must_use]
    pub fn decision_hash(&self) -> &str {
        &self.decision_hash
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRef] {
        &self.evidence
    }

    #[must_use]
    pub const fn authenticator(&self) -> &AuthenticatorEnvelope {
        &self.authenticator
    }

    /// Encodes the frozen v1 claims transcript for a future authenticator.
    ///
    /// The transcript uses a fixed domain/version prefix and ordered tagged
    /// fields. Each field is `tag:u8 || length:u32be || value`. UUIDs and
    /// digests are decoded to fixed-size bytes; UID and expiry are fixed-width
    /// big-endian integers. Evidence is sorted by its canonical reference and
    /// encodes reference digest, kind, trust tier, and exact UTF-8 source.
    /// The authenticator format is included; proof bytes are excluded to avoid
    /// self-reference. This function does not authenticate the transcript.
    pub fn claims_transcript(&self) -> Result<Vec<u8>, ContractError> {
        let mut out = Vec::with_capacity(512);
        out.extend_from_slice(TRANSCRIPT_DOMAIN);

        push_field(&mut out, 1, CONTRACT_VERSION.as_bytes())?;
        push_field(&mut out, 2, &uuid_bytes(&self.grant_id)?)?;
        push_field(&mut out, 3, self.issuer_id.as_bytes())?;
        push_field(&mut out, 4, &self.client_identity.uid.to_be_bytes())?;
        push_field(&mut out, 5, self.capability_id.as_bytes())?;
        push_field(&mut out, 6, &sha256_bytes(self.argument_sha256.as_str())?)?;
        push_field(&mut out, 7, self.resource_scope.as_str().as_bytes())?;
        push_field(&mut out, 8, self.policy_version.as_bytes())?;
        push_field(&mut out, 9, &self.expires_at_unix_ms.to_be_bytes())?;
        push_field(&mut out, 10, &uuid_bytes(&self.trace_id)?)?;
        push_field(&mut out, 11, &sha256_bytes(&self.decision_hash)?)?;
        let evidence = evidence_transcript(&self.evidence)?;
        push_field(&mut out, 12, &evidence)?;
        push_field(&mut out, 13, self.authenticator.format.as_bytes())?;
        Ok(out)
    }
}

/// Request for the sole v1 capability. The parsed request and nested grant
/// remain untrusted data; parsing performs no authenticator verification.
#[derive(Debug, Clone, PartialEq)]
pub struct AiosdRequest {
    request_id: String,
    trace_id: String,
    decision_hash: String,
    evidence: Vec<EvidenceRef>,
    grant: GrantClaims,
}

impl AiosdRequest {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        let wire: WireRequest = parse_bounded_json(bytes)?;
        Self::try_from(wire)
    }

    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    #[must_use]
    pub fn grant(&self) -> &GrantClaims {
        &self.grant
    }

    #[must_use]
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    #[must_use]
    pub fn decision_hash(&self) -> &str {
        &self.decision_hash
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRef] {
        &self.evidence
    }
}

/// Explicit context inputs supplied by later trusted layers. The contract
/// checker uses them for deterministic comparisons only; it does not obtain
/// peer credentials, read a clock, or store replay state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationContext {
    pub authenticated_peer_uid: u32,
    pub trusted_now_unix_ms: u64,
    pub grant_already_consumed: bool,
}

/// Resulting failure from checking grant/request/context bindings. Success
/// means only that these deterministic contract checks matched; it is not
/// authenticity verification, authorization, or execution approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractCheckError {
    CallerMismatch,
    Expired,
    ExpiryTooFarInFuture,
    ReplayedOrConsumed,
    CapabilityMismatch,
    ArgumentBindingMismatch,
    ResourceScopeMismatch,
    TraceMismatch,
    DecisionHashMismatch,
    EvidenceBindingMismatch,
}

/// Closed failure vocabulary for a future GRANT-01 verifier. SEC-01 does not
/// produce `verified` or interpret these values as authorization decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantVerificationFailure {
    MissingAuthenticator,
    MalformedAuthenticator,
    UnsupportedAuthenticatorFormat,
    InvalidAuthenticator,
    UntrustedIssuer,
    UnsupportedVersion,
    CallerMismatch,
    CapabilityMismatch,
    ArgumentBindingMismatch,
    ResourceScopeMismatch,
    PolicyVersionMismatch,
    Expired,
    ReplayedOrConsumed,
    TraceMismatch,
    DecisionHashMismatch,
    EvidenceBindingMismatch,
}

/// Checks structural claim bindings against request and explicit validation
/// context. It does not verify the authenticator or maintain replay state.
pub fn check_grant_bindings(
    request: &AiosdRequest,
    context: ValidationContext,
) -> Result<(), ContractCheckError> {
    let grant = &request.grant;
    if grant.client_identity.uid != context.authenticated_peer_uid {
        return Err(ContractCheckError::CallerMismatch);
    }
    if context.grant_already_consumed {
        return Err(ContractCheckError::ReplayedOrConsumed);
    }
    if context.trusted_now_unix_ms >= grant.expires_at_unix_ms {
        return Err(ContractCheckError::Expired);
    }
    if grant.expires_at_unix_ms - context.trusted_now_unix_ms > MAX_REMAINING_GRANT_LIFETIME_MS {
        return Err(ContractCheckError::ExpiryTooFarInFuture);
    }
    if grant.capability_id != CAPABILITIES_DESCRIBE {
        return Err(ContractCheckError::CapabilityMismatch);
    }
    if grant.argument_sha256.as_str() != capabilities_describe_argument_sha256().as_str() {
        return Err(ContractCheckError::ArgumentBindingMismatch);
    }
    if grant.resource_scope != ResourceScope::RegisteredCapabilityDescriptorMetadata {
        return Err(ContractCheckError::ResourceScopeMismatch);
    }
    if grant.trace_id != request.trace_id {
        return Err(ContractCheckError::TraceMismatch);
    }
    if grant.decision_hash != request.decision_hash {
        return Err(ContractCheckError::DecisionHashMismatch);
    }
    if grant.evidence != request.evidence {
        return Err(ContractCheckError::EvidenceBindingMismatch);
    }
    Ok(())
}

/// Computes SHA-256 over the exact two UTF-8 bytes `{}`. This is an argument
/// binding digest only and provides no grant authentication.
#[must_use]
pub fn capabilities_describe_argument_sha256() -> ArgumentSha256 {
    ArgumentSha256(format_sha256(&Sha256::digest(ARGUMENT_BYTES)))
}

/// Closed postcondition verdict semantics. Unknown never aliases success.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostconditionVerdict {
    Satisfied,
    Failed,
    Unknown,
}

/// One deterministic condition a later workflow requires before promotion.
/// Being present in a requirements list makes the condition mandatory; this
/// type does not implement a workflow or promotion engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostconditionRequirement {
    condition_id: String,
    verification_owner: String,
}

impl PostconditionRequirement {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        let wire: WirePostconditionRequirement = parse_bounded_json(bytes)?;
        require_version(&wire.requirement_version)?;
        validate_token(&wire.condition_id, 64)?;
        validate_token(&wire.verification_owner, 64)?;
        Ok(Self {
            condition_id: wire.condition_id,
            verification_owner: wire.verification_owner,
        })
    }

    #[must_use]
    pub fn condition_id(&self) -> &str {
        &self.condition_id
    }

    #[must_use]
    pub fn verification_owner(&self) -> &str {
        &self.verification_owner
    }
}

/// Deterministic postcondition record linked to the operation and receipt.
#[derive(Debug, Clone, PartialEq)]
pub struct PostconditionVerification {
    condition_id: String,
    verification_owner: String,
    verdict: PostconditionVerdict,
    request_id: String,
    trace_id: String,
    grant_id: String,
    receipt_id: String,
    evidence: Vec<EvidenceRef>,
}

impl PostconditionVerification {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        let wire: WirePostcondition = parse_bounded_json(bytes)?;
        validate_postcondition(wire)
    }

    #[must_use]
    pub fn verdict(&self) -> PostconditionVerdict {
        self.verdict
    }

    #[must_use]
    pub fn condition_id(&self) -> &str {
        &self.condition_id
    }

    #[must_use]
    pub fn verification_owner(&self) -> &str {
        &self.verification_owner
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRef] {
        &self.evidence
    }

    /// Checks report linkage only; this does not judge evidence or promote
    /// workflow state.
    pub fn check_bindings(
        &self,
        requirement: &PostconditionRequirement,
        request: &AiosdRequest,
        receipt: &Receipt,
    ) -> Result<(), ContractError> {
        if receipt.check_request_bindings(request).is_err()
            || self.condition_id != requirement.condition_id
            || self.verification_owner != requirement.verification_owner
            || self.request_id != request.request_id
            || self.trace_id != request.trace_id
            || self.grant_id != request.grant.grant_id
            || self.receipt_id != receipt.receipt_id
            || receipt.request_id != request.request_id
            || receipt.grant_id != request.grant.grant_id
            || receipt.trace_id != request.trace_id
        {
            return Err(ContractError::PostconditionBindingMismatch);
        }
        Ok(())
    }
}

/// Capability descriptor returned by the v1 introspection contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptor {
    pub capability_id: String,
    pub description: String,
    pub mutating: bool,
    pub argument_contract: String,
    pub resource_scope: ResourceScope,
}

/// Bounded execution/observation receipt. A receipt is not postcondition
/// proof, trust evidence, or authorization.
#[derive(Debug, Clone, PartialEq)]
pub struct Receipt {
    receipt_id: String,
    request_id: String,
    grant_id: String,
    caller_identity: UnixPeerIdentity,
    capability_id: String,
    argument_sha256: ArgumentSha256,
    outcome: ReceiptOutcome,
    safe_result_ref: String,
    started_at_unix_ms: u64,
    completed_at_unix_ms: u64,
    trace_id: String,
    decision_hash: String,
    evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptOutcome {
    Succeeded,
    Failed,
}

impl Receipt {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        let wire: WireReceipt = parse_bounded_json(bytes)?;
        validate_receipt(wire)
    }

    #[must_use]
    pub fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    #[must_use]
    pub fn grant_id(&self) -> &str {
        &self.grant_id
    }

    #[must_use]
    pub const fn caller_identity(&self) -> UnixPeerIdentity {
        self.caller_identity
    }

    #[must_use]
    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    #[must_use]
    pub fn argument_sha256(&self) -> &ArgumentSha256 {
        &self.argument_sha256
    }

    #[must_use]
    pub fn outcome(&self) -> ReceiptOutcome {
        self.outcome
    }

    #[must_use]
    pub fn safe_result_ref(&self) -> &str {
        &self.safe_result_ref
    }

    #[must_use]
    pub const fn timestamps_unix_ms(&self) -> (u64, u64) {
        (self.started_at_unix_ms, self.completed_at_unix_ms)
    }

    #[must_use]
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    #[must_use]
    pub fn decision_hash(&self) -> &str {
        &self.decision_hash
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRef] {
        &self.evidence
    }

    /// Checks receipt facts against a structurally validated request. Success
    /// does not establish execution or prove a postcondition.
    pub fn check_request_bindings(&self, request: &AiosdRequest) -> Result<(), ContractError> {
        let grant = &request.grant;
        if self.request_id != request.request_id
            || self.grant_id != grant.grant_id
            || self.caller_identity != grant.client_identity
            || self.capability_id != grant.capability_id
            || self.argument_sha256 != grant.argument_sha256
            || self.trace_id != request.trace_id
            || self.decision_hash != request.decision_hash
            || self.evidence != request.evidence
        {
            return Err(ContractError::ReceiptMismatch);
        }
        Ok(())
    }
}

/// Request/response frame contract decoded from one complete in-memory frame.
/// This does not open or read a socket.
#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Request {
        correlation_id: String,
        request: AiosdRequest,
    },
    Response {
        correlation_id: String,
        outcome: ReceiptOutcome,
        descriptors: Vec<CapabilityDescriptor>,
        receipt: Receipt,
    },
}

impl Frame {
    /// Decodes `u32be payload_length || exact JSON payload`. Partial frames,
    /// trailing bytes, unknown kinds/versions, and over-limit payloads reject.
    pub fn from_complete_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        if bytes.len() < 4 {
            return Err(ContractError::MalformedFrame);
        }
        let length = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        if length == 0 || length > MAX_FRAME_PAYLOAD_BYTES {
            return Err(ContractError::FrameTooLarge);
        }
        if bytes.len() != length + 4 {
            return Err(ContractError::IncompleteOrTrailingFrame);
        }
        let wire: WireFrame = parse_bounded_json(&bytes[4..])?;
        if wire.frame_version != CONTRACT_VERSION {
            return Err(ContractError::UnsupportedVersion);
        }
        validate_uuid(&wire.correlation_id)?;
        match (wire.message_kind.as_str(), wire.request, wire.response) {
            ("request", Some(Some(request)), None) => {
                let request = AiosdRequest::try_from(request)?;
                if wire.correlation_id != request.request_id {
                    return Err(ContractError::CorrelationMismatch);
                }
                Ok(Self::Request {
                    correlation_id: wire.correlation_id,
                    request,
                })
            }
            ("response", None, Some(Some(response))) => {
                let receipt = validate_receipt(response.receipt)?;
                if wire.correlation_id != receipt.request_id {
                    return Err(ContractError::CorrelationMismatch);
                }
                let descriptors = response
                    .descriptors
                    .0
                    .into_iter()
                    .map(validate_descriptor)
                    .collect::<Result<Vec<_>, _>>()?;
                let outcome = parse_receipt_outcome(&response.outcome)?;
                if outcome != receipt.outcome {
                    return Err(ContractError::ReceiptMismatch);
                }
                if (outcome == ReceiptOutcome::Succeeded && descriptors.len() != 1)
                    || (outcome == ReceiptOutcome::Failed && !descriptors.is_empty())
                {
                    return Err(ContractError::InvalidField("descriptors"));
                }
                Ok(Self::Response {
                    correlation_id: wire.correlation_id,
                    outcome,
                    descriptors,
                    receipt,
                })
            }
            _ => Err(ContractError::InvalidField("message_kind/payload")),
        }
    }
}

/// Typed contract parse error. Malformed wire details are intentionally not
/// copied into logs or error strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    InputTooLarge,
    MalformedJson,
    InvalidField(&'static str),
    UnsupportedVersion,
    InvalidIdentifier,
    InvalidHash,
    InvalidAuthenticator,
    InvalidEvidence,
    TooManyEvidenceReferences,
    MissingProvenance,
    MalformedFrame,
    FrameTooLarge,
    IncompleteOrTrailingFrame,
    CorrelationMismatch,
    ReceiptMismatch,
    PostconditionBindingMismatch,
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputTooLarge => f.write_str("input exceeds the v1 byte limit"),
            Self::MalformedJson => f.write_str("malformed or non-closed v1 JSON object"),
            Self::InvalidField(field) => write!(f, "invalid v1 field: {field}"),
            Self::UnsupportedVersion => f.write_str("unsupported v1 contract version"),
            Self::InvalidIdentifier => f.write_str("invalid v1 identifier"),
            Self::InvalidHash => f.write_str("invalid lowercase SHA-256 value"),
            Self::InvalidAuthenticator => f.write_str("invalid authenticator envelope structure"),
            Self::InvalidEvidence => f.write_str("invalid evidence reference"),
            Self::TooManyEvidenceReferences => f.write_str("too many evidence references"),
            Self::MissingProvenance => f.write_str("required provenance is missing"),
            Self::MalformedFrame => f.write_str("malformed complete frame"),
            Self::FrameTooLarge => f.write_str("frame exceeds the v1 byte limit"),
            Self::IncompleteOrTrailingFrame => {
                f.write_str("frame is incomplete or has trailing bytes")
            }
            Self::CorrelationMismatch => f.write_str("frame correlation does not match request"),
            Self::ReceiptMismatch => f.write_str("receipt does not match response outcome"),
            Self::PostconditionBindingMismatch => {
                f.write_str("postcondition report linkage does not match request and receipt")
            }
        }
    }
}

impl std::error::Error for ContractError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WireResourceScope {
    RegisteredCapabilityDescriptorMetadata,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireEmptyArguments {}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireClientIdentity {
    kind: String,
    uid: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireAuthenticator {
    format: String,
    proof: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireEvidence {
    #[serde(rename = "ref")]
    reference: String,
    kind: String,
    trust_tier: String,
    source: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireGrant {
    grant_version: String,
    grant_id: String,
    issuer_id: String,
    client_identity: WireClientIdentity,
    capability_id: String,
    argument_sha256: String,
    resource_scope: WireResourceScope,
    policy_version: String,
    expires_at_unix_ms: u64,
    trace_id: String,
    decision_hash: String,
    evidence: BoundedVec<WireEvidence, MAX_EVIDENCE_REFS>,
    authenticator: WireAuthenticator,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRequest {
    request_version: String,
    request_id: String,
    trace_id: String,
    capability_id: String,
    arguments: WireEmptyArguments,
    resource_scope: WireResourceScope,
    decision_hash: String,
    evidence: BoundedVec<WireEvidence, MAX_EVIDENCE_REFS>,
    grant: WireGrant,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireReceipt {
    receipt_version: String,
    receipt_id: String,
    request_id: String,
    grant_id: String,
    caller_identity: WireClientIdentity,
    capability_id: String,
    argument_sha256: String,
    outcome: String,
    safe_result_ref: String,
    started_at_unix_ms: u64,
    completed_at_unix_ms: u64,
    trace_id: String,
    decision_hash: String,
    evidence: BoundedVec<WireEvidence, MAX_EVIDENCE_REFS>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WirePostcondition {
    verification_version: String,
    condition_id: String,
    verification_owner: String,
    verdict: String,
    request_id: String,
    trace_id: String,
    grant_id: String,
    receipt_id: String,
    evidence: BoundedVec<WireEvidence, MAX_EVIDENCE_REFS>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WirePostconditionRequirement {
    requirement_version: String,
    condition_id: String,
    verification_owner: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireDescriptor {
    descriptor_version: String,
    capability_id: String,
    description: String,
    mutating: bool,
    argument_contract: String,
    resource_scope: WireResourceScope,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireResponse {
    outcome: String,
    descriptors: BoundedVec<WireDescriptor, 1>,
    receipt: WireReceipt,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireFrame {
    frame_version: String,
    message_kind: String,
    correlation_id: String,
    #[serde(default)]
    request: Option<Option<WireRequest>>,
    #[serde(default)]
    response: Option<Option<WireResponse>>,
}

#[derive(Debug, Clone)]
struct BoundedVec<T, const MAX: usize>(Vec<T>);

impl<'de, T, const MAX: usize> Deserialize<'de> for BoundedVec<T, MAX>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BoundedVisitor<T, const MAX: usize>(std::marker::PhantomData<T>);
        impl<'de, T, const MAX: usize> Visitor<'de> for BoundedVisitor<T, MAX>
        where
            T: Deserialize<'de>,
        {
            type Value = BoundedVec<T, MAX>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "an array with at most {MAX} entries")
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::with_capacity(sequence.size_hint().unwrap_or(0).min(MAX));
                while let Some(value) = sequence.next_element()? {
                    if values.len() == MAX {
                        return Err(de::Error::custom("array exceeds the v1 item limit"));
                    }
                    values.push(value);
                }
                Ok(BoundedVec(values))
            }
        }
        deserializer.deserialize_seq(BoundedVisitor::<T, MAX>(std::marker::PhantomData))
    }
}

impl TryFrom<WireRequest> for AiosdRequest {
    type Error = ContractError;

    fn try_from(wire: WireRequest) -> Result<Self, Self::Error> {
        require_version(&wire.request_version)?;
        validate_uuid(&wire.request_id)?;
        validate_uuid(&wire.trace_id)?;
        validate_hash(&wire.decision_hash)?;
        if wire.capability_id != CAPABILITIES_DESCRIBE {
            return Err(ContractError::InvalidField("capability_id"));
        }
        let _ = wire.arguments;
        let evidence = parse_evidence(wire.evidence.0)?;
        require_provenance(&evidence)?;
        let grant = parse_grant(wire.grant)?;
        if grant.grant_id.is_empty() {
            return Err(ContractError::InvalidIdentifier);
        }
        let request = Self {
            request_id: wire.request_id,
            trace_id: wire.trace_id,
            decision_hash: wire.decision_hash,
            evidence,
            grant,
        };
        if request.grant.capability_id != CAPABILITIES_DESCRIBE
            || request.grant.resource_scope != wire.resource_scope.into()
            || request.grant.trace_id != request.trace_id
            || request.grant.decision_hash != request.decision_hash
            || request.grant.evidence != request.evidence
            || request.grant.argument_sha256.as_str()
                != capabilities_describe_argument_sha256().as_str()
        {
            return Err(ContractError::InvalidField("grant/request binding"));
        }
        Ok(request)
    }
}

fn parse_grant(wire: WireGrant) -> Result<GrantClaims, ContractError> {
    require_version(&wire.grant_version)?;
    validate_uuid(&wire.grant_id)?;
    validate_token(&wire.issuer_id, 64)?;
    if wire.client_identity.kind != "unix_uid" {
        return Err(ContractError::InvalidField("client_identity.kind"));
    }
    if wire.capability_id != CAPABILITIES_DESCRIBE {
        return Err(ContractError::InvalidField("capability_id"));
    }
    validate_hash(&wire.argument_sha256)?;
    validate_token(&wire.policy_version, 128)?;
    validate_uuid(&wire.trace_id)?;
    validate_hash(&wire.decision_hash)?;
    validate_authenticator(&wire.authenticator)?;
    let evidence = parse_evidence(wire.evidence.0)?;
    require_provenance(&evidence)?;
    Ok(GrantClaims {
        grant_id: wire.grant_id,
        issuer_id: wire.issuer_id,
        client_identity: UnixPeerIdentity {
            uid: wire.client_identity.uid,
        },
        capability_id: wire.capability_id,
        argument_sha256: ArgumentSha256(wire.argument_sha256),
        resource_scope: wire.resource_scope.into(),
        policy_version: wire.policy_version,
        expires_at_unix_ms: wire.expires_at_unix_ms,
        trace_id: wire.trace_id,
        decision_hash: wire.decision_hash,
        evidence,
        authenticator: AuthenticatorEnvelope {
            format: wire.authenticator.format,
            proof: wire.authenticator.proof,
        },
    })
}

fn validate_authenticator(wire: &WireAuthenticator) -> Result<(), ContractError> {
    if !valid_format_identifier(&wire.format)
        || wire.proof.is_empty()
        || !wire.proof.len().is_multiple_of(2)
        || wire.proof.len() > MAX_AUTHENTICATOR_PROOF_BYTES * 2
        || !wire
            .proof
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ContractError::InvalidAuthenticator);
    }
    Ok(())
}

fn parse_evidence(wire: Vec<WireEvidence>) -> Result<Vec<EvidenceRef>, ContractError> {
    if wire.len() > MAX_EVIDENCE_REFS {
        return Err(ContractError::TooManyEvidenceReferences);
    }
    let mut values = Vec::with_capacity(wire.len());
    for item in wire {
        let value = serde_json::json!({
            "ref": item.reference,
            "kind": item.kind,
            "trust_tier": item.trust_tier,
            "source": item.source,
        });
        values.push(EvidenceRef::from_value(&value).map_err(|_| ContractError::InvalidEvidence)?);
    }
    values.sort_by(|left, right| left.reference.cmp(&right.reference));
    if values
        .windows(2)
        .any(|pair| pair[0].reference == pair[1].reference)
    {
        return Err(ContractError::InvalidEvidence);
    }
    Ok(values)
}

fn require_provenance(evidence: &[EvidenceRef]) -> Result<(), ContractError> {
    if evidence.is_empty() {
        return Err(ContractError::MissingProvenance);
    }
    Ok(())
}

fn validate_receipt(wire: WireReceipt) -> Result<Receipt, ContractError> {
    require_version(&wire.receipt_version)?;
    validate_uuid(&wire.receipt_id)?;
    validate_uuid(&wire.request_id)?;
    validate_uuid(&wire.grant_id)?;
    validate_uuid(&wire.trace_id)?;
    if wire.caller_identity.kind != "unix_uid" {
        return Err(ContractError::InvalidField("caller_identity.kind"));
    }
    if wire.capability_id != CAPABILITIES_DESCRIBE {
        return Err(ContractError::InvalidField("capability_id"));
    }
    validate_hash(&wire.argument_sha256)?;
    validate_hash(&wire.decision_hash)?;
    validate_result_ref(&wire.safe_result_ref)?;
    if wire.started_at_unix_ms > wire.completed_at_unix_ms {
        return Err(ContractError::InvalidField("receipt timestamps"));
    }
    let evidence = parse_evidence(wire.evidence.0)?;
    require_provenance(&evidence)?;
    Ok(Receipt {
        receipt_id: wire.receipt_id,
        request_id: wire.request_id,
        grant_id: wire.grant_id,
        caller_identity: UnixPeerIdentity {
            uid: wire.caller_identity.uid,
        },
        capability_id: wire.capability_id,
        argument_sha256: ArgumentSha256(wire.argument_sha256),
        outcome: parse_receipt_outcome(&wire.outcome)?,
        safe_result_ref: wire.safe_result_ref,
        started_at_unix_ms: wire.started_at_unix_ms,
        completed_at_unix_ms: wire.completed_at_unix_ms,
        trace_id: wire.trace_id,
        decision_hash: wire.decision_hash,
        evidence,
    })
}

fn validate_descriptor(wire: WireDescriptor) -> Result<CapabilityDescriptor, ContractError> {
    require_version(&wire.descriptor_version)?;
    if wire.capability_id != CAPABILITIES_DESCRIBE
        || !valid_bounded_text(&wire.description, 512)
        || wire.mutating
        || wire.argument_contract != "empty_object_v1"
    {
        return Err(ContractError::InvalidField("capability descriptor"));
    }
    Ok(CapabilityDescriptor {
        capability_id: wire.capability_id,
        description: wire.description,
        mutating: false,
        argument_contract: wire.argument_contract,
        resource_scope: wire.resource_scope.into(),
    })
}

fn validate_postcondition(
    wire: WirePostcondition,
) -> Result<PostconditionVerification, ContractError> {
    require_version(&wire.verification_version)?;
    validate_token(&wire.condition_id, 64)?;
    validate_token(&wire.verification_owner, 64)?;
    validate_uuid(&wire.request_id)?;
    validate_uuid(&wire.trace_id)?;
    validate_uuid(&wire.grant_id)?;
    validate_uuid(&wire.receipt_id)?;
    let evidence = parse_evidence(wire.evidence.0)?;
    require_provenance(&evidence)?;
    let verdict = match wire.verdict.as_str() {
        "satisfied" => PostconditionVerdict::Satisfied,
        "failed" => PostconditionVerdict::Failed,
        "unknown" => PostconditionVerdict::Unknown,
        _ => return Err(ContractError::InvalidField("verdict")),
    };
    Ok(PostconditionVerification {
        condition_id: wire.condition_id,
        verification_owner: wire.verification_owner,
        verdict,
        request_id: wire.request_id,
        trace_id: wire.trace_id,
        grant_id: wire.grant_id,
        receipt_id: wire.receipt_id,
        evidence,
    })
}

fn parse_receipt_outcome(value: &str) -> Result<ReceiptOutcome, ContractError> {
    match value {
        "succeeded" => Ok(ReceiptOutcome::Succeeded),
        "failed" => Ok(ReceiptOutcome::Failed),
        _ => Err(ContractError::InvalidField("outcome")),
    }
}

fn require_version(value: &str) -> Result<(), ContractError> {
    if value == CONTRACT_VERSION {
        Ok(())
    } else {
        Err(ContractError::UnsupportedVersion)
    }
}

fn valid_bounded_text(value: &str, max_chars: usize) -> bool {
    !value.is_empty() && value.chars().count() <= max_chars && !value.chars().any(char::is_control)
}

fn valid_format_identifier(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 64
        && bytes[0].is_ascii_lowercase()
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn validate_token(value: &str, max_len: usize) -> Result<(), ContractError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > max_len
        || !bytes[0].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
    {
        return Err(ContractError::InvalidIdentifier);
    }
    Ok(())
}

fn validate_uuid(value: &str) -> Result<(), ContractError> {
    if value.len() != 36
        || !value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                byte == b'-'
            } else {
                byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
            }
        })
    {
        return Err(ContractError::InvalidIdentifier);
    }
    Ok(())
}

fn uuid_bytes(value: &str) -> Result<[u8; 16], ContractError> {
    validate_uuid(value)?;
    let mut out = [0; 16];
    let mut nibble = None;
    let mut at = 0;
    for byte in value.bytes().filter(|byte| *byte != b'-') {
        let value = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => return Err(ContractError::InvalidIdentifier),
        };
        if let Some(high) = nibble.take() {
            out[at] = (high << 4) | value;
            at += 1;
        } else {
            nibble = Some(value);
        }
    }
    Ok(out)
}

fn validate_hash(value: &str) -> Result<(), ContractError> {
    let _ = sha256_bytes(value)?;
    Ok(())
}

fn sha256_bytes(value: &str) -> Result<[u8; 32], ContractError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(ContractError::InvalidHash);
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ContractError::InvalidHash);
    }
    let mut out = [0; 32];
    let (pairs, remainder) = hex.as_bytes().as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(ContractError::InvalidHash);
    }
    for (index, pair) in pairs.iter().enumerate() {
        out[index] = (hex_value(pair[0])? << 4) | hex_value(pair[1])?;
    }
    Ok(out)
}

fn hex_value(byte: u8) -> Result<u8, ContractError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(ContractError::InvalidHash),
    }
}

fn format_sha256(digest: &[u8]) -> String {
    let mut output = String::with_capacity(7 + digest.len() * 2);
    output.push_str("sha256:");
    for byte in digest {
        write!(&mut output, "{byte:02x}").expect("writing to String cannot fail");
    }
    output
}

fn validate_result_ref(value: &str) -> Result<(), ContractError> {
    let Some(digest) = value.strip_prefix("result://sha256/") else {
        return Err(ContractError::InvalidField("safe_result_ref"));
    };
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ContractError::InvalidField("safe_result_ref"));
    }
    Ok(())
}

impl From<WireResourceScope> for ResourceScope {
    fn from(value: WireResourceScope) -> Self {
        match value {
            WireResourceScope::RegisteredCapabilityDescriptorMetadata => {
                Self::RegisteredCapabilityDescriptorMetadata
            }
        }
    }
}

fn parse_bounded_json<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, ContractError> {
    if bytes.len() > MAX_JSON_BYTES {
        return Err(ContractError::InputTooLarge);
    }
    if !arrays_within_limit(bytes) {
        return Err(ContractError::MalformedJson);
    }
    // Deserialize typed structs directly before building the generic JSON
    // value. Serde's typed struct visitors reject repeated members; parsing a
    // Value first would silently collapse duplicate raw JSON keys.
    let parsed: T = serde_json::from_slice(bytes).map_err(|_| ContractError::MalformedJson)?;
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| ContractError::MalformedJson)?;
    if !wire_object_shapes_are_valid(&value, None) {
        return Err(ContractError::MalformedJson);
    }
    Ok(parsed)
}

/// Counts array elements in the bounded source bytes before serde allocates
/// typed vectors. The scan is lexical only; serde remains responsible for
/// JSON syntax validation.
fn arrays_within_limit(bytes: &[u8]) -> bool {
    #[derive(Clone, Copy)]
    struct Container {
        is_array: bool,
        has_value: bool,
        commas: usize,
    }

    let mut stack: Vec<Container> = Vec::new();
    let mut in_string = false;
    let mut escaped = false;
    for byte in bytes {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match *byte {
            b'"' => {
                in_string = true;
                if let Some(parent) = stack.last_mut()
                    && parent.is_array
                {
                    parent.has_value = true;
                }
            }
            b'[' | b'{' => {
                if let Some(parent) = stack.last_mut()
                    && parent.is_array
                {
                    parent.has_value = true;
                }
                stack.push(Container {
                    is_array: *byte == b'[',
                    has_value: false,
                    commas: 0,
                });
            }
            b']' => {
                let Some(container) = stack.pop() else {
                    return true;
                };
                if container.is_array
                    && container.has_value
                    && container.commas + 1 > MAX_EVIDENCE_REFS
                {
                    return false;
                }
            }
            b'}' => {
                if stack.pop().is_none() {
                    return true;
                }
            }
            b',' => {
                if let Some(container) = stack.last_mut()
                    && container.is_array
                {
                    container.commas += 1;
                    if container.commas >= MAX_EVIDENCE_REFS {
                        return false;
                    }
                }
            }
            byte if !byte.is_ascii_whitespace() => {
                if let Some(parent) = stack.last_mut()
                    && parent.is_array
                {
                    parent.has_value = true;
                }
            }
            _ => {}
        }
    }
    true
}

/// Serde's derived struct deserializers also accept positional arrays. The
/// SEC wire contract requires objects, and only the evidence/descriptor
/// fields may contain arrays.
fn wire_object_shapes_are_valid(value: &serde_json::Value, field: Option<&str>) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().all(|(key, value)| {
            (!matches!(key.as_str(), "request" | "response") || !value.is_null())
                && wire_object_shapes_are_valid(value, Some(key))
        }),
        serde_json::Value::Array(items) => {
            matches!(field, Some("evidence" | "descriptors"))
                && items.len() <= MAX_EVIDENCE_REFS
                && items
                    .iter()
                    .all(|item| item.is_object() && wire_object_shapes_are_valid(item, None))
        }
        _ => true,
    }
}

fn push_field(output: &mut Vec<u8>, tag: u8, value: &[u8]) -> Result<(), ContractError> {
    let len = u32::try_from(value.len()).map_err(|_| ContractError::InvalidField("transcript"))?;
    output.push(tag);
    output.extend_from_slice(&len.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn evidence_transcript(evidence: &[EvidenceRef]) -> Result<Vec<u8>, ContractError> {
    if evidence.len() > MAX_EVIDENCE_REFS {
        return Err(ContractError::TooManyEvidenceReferences);
    }
    let count =
        u32::try_from(evidence.len()).map_err(|_| ContractError::TooManyEvidenceReferences)?;
    let mut out = Vec::with_capacity(4 + evidence.len() * 64);
    out.extend_from_slice(&count.to_be_bytes());
    for item in evidence {
        let Some(hex) = item.reference.strip_prefix("evidence://sha256/") else {
            return Err(ContractError::InvalidEvidence);
        };
        let mut digest = [0; 32];
        let (pairs, remainder) = hex.as_bytes().as_chunks::<2>();
        if !remainder.is_empty() {
            return Err(ContractError::InvalidEvidence);
        }
        for (index, pair) in pairs.iter().enumerate() {
            digest[index] = (hex_value(pair[0])? << 4) | hex_value(pair[1])?;
        }
        out.extend_from_slice(&digest);
        let kind = match item.kind {
            EvidenceKind::UserInput => "user_input",
            EvidenceKind::Registry => "registry",
            EvidenceKind::ToolResult => "tool_result",
            EvidenceKind::Memory => "memory",
            EvidenceKind::ExternalDocument => "external_document",
        };
        let tier = match item.trust_tier {
            TrustTier::Untrusted => "untrusted",
            TrustTier::Observed => "observed",
            TrustTier::Verified => "verified",
            TrustTier::Policy => "policy",
        };
        push_len_prefixed(&mut out, kind.as_bytes())?;
        push_len_prefixed(&mut out, tier.as_bytes())?;
        push_len_prefixed(&mut out, item.source.as_bytes())?;
    }
    Ok(out)
}

fn push_len_prefixed(output: &mut Vec<u8>, value: &[u8]) -> Result<(), ContractError> {
    let len = u32::try_from(value.len()).map_err(|_| ContractError::InvalidField("transcript"))?;
    output.extend_from_slice(&len.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use super::{capabilities_describe_argument_sha256, uuid_bytes, validate_uuid};

    #[test]
    fn uuid_encoding_is_fixed_width_and_lowercase() {
        let value = "00112233-4455-6677-8899-aabbccddeeff";
        assert_eq!(
            uuid_bytes(value).expect("valid uuid"),
            [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
                0xee, 0xff,
            ]
        );
        assert!(validate_uuid("00112233-4455-6677-8899-AABBCCDDEEFF").is_err());
    }

    #[test]
    fn empty_argument_digest_is_exact_sha256_of_two_bytes() {
        assert_eq!(
            capabilities_describe_argument_sha256().as_str(),
            "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
        );
    }
}
