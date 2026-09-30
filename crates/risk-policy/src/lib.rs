//! Deterministic ZHOOR v1 risk classification and scrutiny recommendations.
//!
//! This crate reports risk facts, evidence requirements, and recommendations
//! for later policy review. It does not authorize, deny, grant, execute, read
//! secrets, inspect the host, or interpret natural-language input.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use zhoor_protocol::TrustTier;

const MAX_INPUT_JSON_BYTES: usize = 64 * 1024;
const MAX_SIGNALS: usize = 7;
const MAX_PROMPT_INJECTION_INDICATORS: usize = 1;
const MAX_EVIDENCE_TRUST_TIERS: usize = 4;

/// The only capability whose low-risk behavior is pinned by this v1 policy.
pub const KNOWN_LOW_RISK_CAPABILITY: &str = "system.info.read";

/// Closed policy version. A new policy requires an explicit version change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyVersion {
    /// Deterministic risk policy version 1.
    #[serde(rename = "1")]
    V1,
}

/// Explicitly supplied, normalized risk facts. This type contains no raw
/// request text, paths, commands, or host state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PolicyInput {
    /// Pinned policy version.
    pub policy_version: PolicyVersion,
    /// Exact capability identifier from the typed request.
    pub capability: String,
    /// Facts already normalized by the caller; POL-01 does not parse them.
    pub signals: Vec<RiskSignal>,
    /// A deterministic upstream ambiguity fact. It can only raise scrutiny.
    pub semantic_ambiguity: bool,
    /// Explicit upstream indicators; this crate does not scan prompt text.
    pub prompt_injection_indicators: Vec<PromptInjectionIndicator>,
    /// Trust tiers of evidence metadata supplied with the request.
    pub evidence_trust_tiers: Vec<String>,
}

/// Closed hard-risk facts. Any occurrence produces high risk and human review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskSignal {
    /// A shell or command interpreter would be used.
    Shell,
    /// A package would be installed or changed.
    PackageInstallation,
    /// A system service would be controlled.
    ServiceControl,
    /// A request would use network egress.
    NetworkEgress,
    /// A requested resource is outside its declared scope.
    ResourceScopeExceeded,
    /// A request would use privilege beyond the caller's ordinary identity.
    Privilege,
    /// A request would mutate state.
    Mutation,
}

/// Narrow prompt-injection indicator vocabulary supported in v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptInjectionIndicator {
    /// Supplied content explicitly asks to bypass safeguards or policy.
    SafeguardBypassInstruction,
}

/// Coarse deterministic risk classification. It is not an execution decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    /// Known low-risk capability with no applicable higher rule.
    Low,
    /// A scrutiny-raising signal applies without a hard high-risk fact.
    Medium,
    /// One or more deterministic hard-risk facts apply.
    High,
    /// The capability is not defined by this policy version.
    Unknown,
}

/// Recommended scrutiny level. No variant grants or denies execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrutinyRecommendation {
    /// Routine policy review for the pinned low-risk capability.
    Routine,
    /// Increased scrutiny for ambiguity or untrusted instructions.
    Increased,
    /// Human review recommendation for hard-risk or unresolved facts.
    HumanReview,
}

/// Non-authoritative evaluation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationState {
    /// The supplied facts were sufficient for this bounded classification.
    Evaluated,
    /// The result is incomplete and must not be treated as a safe answer.
    Abstained,
}

/// Stable reason codes emitted in enum order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonCode {
    /// Capability identifier is not in the fixed v1 known set.
    UnknownCapability,
    /// A shell or command interpreter is requested.
    ShellRequested,
    /// Package installation or change is requested.
    PackageInstallationRequested,
    /// Service control is requested.
    ServiceControlRequested,
    /// Network egress is requested.
    NetworkEgressRequested,
    /// A resource lies outside declared scope.
    ResourceScopeExceeded,
    /// Privilege is requested.
    PrivilegeRequested,
    /// Mutation is requested.
    MutationRequested,
    /// Semantic ambiguity is present; it cannot lower other rules.
    SemanticAmbiguity,
    /// An explicit safeguard-bypass instruction indicator was supplied.
    SafeguardBypassInstruction,
    /// The evidence tier required by the applicable rule is absent.
    MissingRequiredEvidence,
}

/// Evidence requirement codes emitted in stable enum order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRequirement {
    /// At least observed-tier evidence must be supplied.
    ObservedEvidence,
    /// At least verified-tier evidence must be supplied.
    VerifiedEvidence,
    /// A human must review the request and supplied evidence.
    HumanReview,
}

/// Complete deterministic output for one policy evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PolicyAssessment {
    /// Policy version used.
    pub policy_version: PolicyVersion,
    /// Classification; never an authorization disposition.
    pub risk_level: RiskLevel,
    /// Recommended level of scrutiny.
    pub scrutiny: ScrutinyRecommendation,
    /// Whether the classification is complete for this bounded contract.
    pub state: EvaluationState,
    /// Whether escalation to a human is recommended.
    pub advisory_escalation: bool,
    /// Stable, duplicate-free reason codes.
    pub reasons: Vec<ReasonCode>,
    /// Stable, duplicate-free evidence requirements.
    pub evidence_requirements: Vec<EvidenceRequirement>,
}

/// Deterministic policy input or evaluation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    /// A required bounded identifier is malformed.
    InvalidCapability,
    /// A duplicate signal or evidence tier was supplied.
    DuplicateInputFact,
    /// An evidence trust tier is not defined by the accepted protocol.
    UnknownEvidenceTier,
    /// A JSON input or a directly constructed input exceeds v1 bounds.
    InputLimitExceeded,
}

impl PolicyInput {
    /// Parse bounded closed JSON input. Unknown fields, versions, and enum values fail.
    pub fn from_json_str(text: &str) -> Result<Self, serde_json::Error> {
        if text.len() > MAX_INPUT_JSON_BYTES {
            return Err(<serde_json::Error as serde::de::Error>::custom(
                "policy input exceeds the 65536-byte v1 limit",
            ));
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireInput {
            policy_version: PolicyVersion,
            capability: String,
            signals: Vec<RiskSignal>,
            semantic_ambiguity: bool,
            prompt_injection_indicators: Vec<PromptInjectionIndicator>,
            evidence_trust_tiers: Vec<String>,
        }

        let wire: WireInput = serde_json::from_str(text)?;
        if wire.signals.len() > MAX_SIGNALS
            || wire.prompt_injection_indicators.len() > MAX_PROMPT_INJECTION_INDICATORS
            || wire.evidence_trust_tiers.len() > MAX_EVIDENCE_TRUST_TIERS
        {
            return Err(<serde_json::Error as serde::de::Error>::custom(
                "policy input exceeds a v1 array limit",
            ));
        }
        Ok(Self {
            policy_version: wire.policy_version,
            capability: wire.capability,
            signals: wire.signals,
            semantic_ambiguity: wire.semantic_ambiguity,
            prompt_injection_indicators: wire.prompt_injection_indicators,
            evidence_trust_tiers: wire.evidence_trust_tiers,
        })
    }
}

impl PolicyAssessment {
    /// Parse a bounded closed assessment; duplicate codes and unknown fields fail.
    pub fn from_json_str(text: &str) -> Result<Self, serde_json::Error> {
        if text.len() > MAX_INPUT_JSON_BYTES {
            return Err(<serde_json::Error as serde::de::Error>::custom(
                "policy assessment exceeds the 65536-byte v1 limit",
            ));
        }

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireAssessment {
            policy_version: PolicyVersion,
            risk_level: RiskLevel,
            scrutiny: ScrutinyRecommendation,
            state: EvaluationState,
            advisory_escalation: bool,
            reasons: Vec<ReasonCode>,
            evidence_requirements: Vec<EvidenceRequirement>,
        }

        let wire: WireAssessment = serde_json::from_str(text)?;
        if wire.reasons.len() > 11 || wire.evidence_requirements.len() > 3 {
            return Err(<serde_json::Error as serde::de::Error>::custom(
                "policy assessment exceeds a v1 array limit",
            ));
        }
        if wire.reasons.iter().copied().collect::<BTreeSet<_>>().len() != wire.reasons.len() {
            return Err(<serde_json::Error as serde::de::Error>::custom(
                "duplicate reason code",
            ));
        }
        if wire
            .evidence_requirements
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != wire.evidence_requirements.len()
        {
            return Err(<serde_json::Error as serde::de::Error>::custom(
                "duplicate evidence requirement",
            ));
        }

        Ok(Self {
            policy_version: wire.policy_version,
            risk_level: wire.risk_level,
            scrutiny: wire.scrutiny,
            state: wire.state,
            advisory_escalation: wire.advisory_escalation,
            reasons: wire.reasons,
            evidence_requirements: wire.evidence_requirements,
        })
    }
}

impl PolicyInput {
    /// Evaluate the normalized facts using the pinned v1 policy.
    pub fn evaluate(&self) -> Result<PolicyAssessment, PolicyError> {
        if self.signals.len() > MAX_SIGNALS
            || self.prompt_injection_indicators.len() > MAX_PROMPT_INJECTION_INDICATORS
            || self.evidence_trust_tiers.len() > MAX_EVIDENCE_TRUST_TIERS
        {
            return Err(PolicyError::InputLimitExceeded);
        }
        let capability_len = self.capability.chars().count();
        if !(1..=256).contains(&capability_len)
            || self.capability.trim_matches(|character: char| {
                character.is_whitespace() || character == '\u{FEFF}'
            }) != self.capability
            || self.capability.chars().any(char::is_control)
        {
            return Err(PolicyError::InvalidCapability);
        }

        let signals: BTreeSet<_> = self.signals.iter().copied().collect();
        let injection: BTreeSet<_> = self.prompt_injection_indicators.iter().copied().collect();
        let mut evidence_names = BTreeSet::new();
        let duplicate_evidence = self
            .evidence_trust_tiers
            .iter()
            .any(|tier| !evidence_names.insert(tier.as_str()));
        if signals.len() != self.signals.len()
            || injection.len() != self.prompt_injection_indicators.len()
            || duplicate_evidence
        {
            return Err(PolicyError::DuplicateInputFact);
        }
        let evidence = self
            .evidence_trust_tiers
            .iter()
            .map(|tier| TrustTier::parse(tier).ok_or(PolicyError::UnknownEvidenceTier))
            .collect::<Result<Vec<_>, _>>()?;

        let known = self.capability == KNOWN_LOW_RISK_CAPABILITY;
        let hard_risk = !signals.is_empty();
        let ambiguous = self.semantic_ambiguity || !injection.is_empty();

        let mut reasons = BTreeSet::new();
        if !known {
            reasons.insert(ReasonCode::UnknownCapability);
        }
        for signal in &signals {
            reasons.insert(match signal {
                RiskSignal::Shell => ReasonCode::ShellRequested,
                RiskSignal::PackageInstallation => ReasonCode::PackageInstallationRequested,
                RiskSignal::ServiceControl => ReasonCode::ServiceControlRequested,
                RiskSignal::NetworkEgress => ReasonCode::NetworkEgressRequested,
                RiskSignal::ResourceScopeExceeded => ReasonCode::ResourceScopeExceeded,
                RiskSignal::Privilege => ReasonCode::PrivilegeRequested,
                RiskSignal::Mutation => ReasonCode::MutationRequested,
            });
        }
        if self.semantic_ambiguity {
            reasons.insert(ReasonCode::SemanticAmbiguity);
        }
        if injection.contains(&PromptInjectionIndicator::SafeguardBypassInstruction) {
            reasons.insert(ReasonCode::SafeguardBypassInstruction);
        }

        let mut requirements = BTreeSet::new();
        let minimum_tier = if hard_risk || !known {
            requirements.insert(EvidenceRequirement::HumanReview);
            TrustTier::Verified
        } else {
            TrustTier::Observed
        };
        requirements.insert(match minimum_tier {
            TrustTier::Observed => EvidenceRequirement::ObservedEvidence,
            _ => EvidenceRequirement::VerifiedEvidence,
        });

        let has_required_evidence = evidence.contains(&minimum_tier)
            || (minimum_tier == TrustTier::Observed && evidence.contains(&TrustTier::Verified));
        if !has_required_evidence {
            reasons.insert(ReasonCode::MissingRequiredEvidence);
        }

        let risk_level = if hard_risk {
            RiskLevel::High
        } else if !known {
            RiskLevel::Unknown
        } else if ambiguous {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        let mut scrutiny = match risk_level {
            RiskLevel::Low => ScrutinyRecommendation::Routine,
            RiskLevel::Medium => ScrutinyRecommendation::Increased,
            RiskLevel::High | RiskLevel::Unknown => ScrutinyRecommendation::HumanReview,
        };
        if ambiguous && scrutiny < ScrutinyRecommendation::Increased {
            scrutiny = ScrutinyRecommendation::Increased;
        }
        if hard_risk || !known {
            scrutiny = ScrutinyRecommendation::HumanReview;
        }
        let missing_evidence = !has_required_evidence;
        if missing_evidence {
            requirements.insert(EvidenceRequirement::HumanReview);
            if scrutiny < ScrutinyRecommendation::Increased {
                scrutiny = ScrutinyRecommendation::Increased;
            }
        }

        let advisory_escalation = !known || hard_risk || ambiguous || missing_evidence;
        let state = if !known || missing_evidence {
            EvaluationState::Abstained
        } else {
            EvaluationState::Evaluated
        };

        Ok(PolicyAssessment {
            policy_version: self.policy_version,
            risk_level,
            scrutiny,
            state,
            advisory_escalation,
            reasons: reasons.into_iter().collect(),
            evidence_requirements: requirements.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> PolicyInput {
        PolicyInput {
            policy_version: PolicyVersion::V1,
            capability: KNOWN_LOW_RISK_CAPABILITY.to_owned(),
            signals: Vec::new(),
            semantic_ambiguity: false,
            prompt_injection_indicators: Vec::new(),
            evidence_trust_tiers: vec!["observed".to_owned()],
        }
    }

    #[test]
    fn known_read_only_capability_is_routine_with_observed_evidence() {
        let result = input().evaluate().expect("valid input");
        assert_eq!(result.risk_level, RiskLevel::Low);
        assert_eq!(result.scrutiny, ScrutinyRecommendation::Routine);
        assert_eq!(result.state, EvaluationState::Evaluated);
        assert!(!result.advisory_escalation);
    }

    #[test]
    fn unknown_capability_abstains_and_escalates() {
        let mut value = input();
        value.capability = "vendor.unknown".to_owned();
        let result = value.evaluate().expect("bounded input");
        assert_eq!(result.risk_level, RiskLevel::Unknown);
        assert_eq!(result.state, EvaluationState::Abstained);
        assert!(result.advisory_escalation);
        assert_eq!(result.scrutiny, ScrutinyRecommendation::HumanReview);
    }

    #[test]
    fn unknown_capability_does_not_hide_a_hard_high_risk_fact() {
        let mut value = input();
        value.capability = "vendor.unknown".to_owned();
        value.signals.push(RiskSignal::Mutation);
        value.evidence_trust_tiers = vec!["verified".to_owned()];
        let result = value.evaluate().expect("bounded input");
        assert_eq!(result.risk_level, RiskLevel::High);
        assert_eq!(result.state, EvaluationState::Abstained);
        assert!(result.advisory_escalation);
        assert!(result.reasons.contains(&ReasonCode::UnknownCapability));
        assert!(result.reasons.contains(&ReasonCode::MutationRequested));
    }

    #[test]
    fn every_hard_signal_is_high_and_requires_human_review() {
        for signal in [
            RiskSignal::Shell,
            RiskSignal::PackageInstallation,
            RiskSignal::ServiceControl,
            RiskSignal::NetworkEgress,
            RiskSignal::ResourceScopeExceeded,
            RiskSignal::Privilege,
            RiskSignal::Mutation,
        ] {
            let mut value = input();
            value.signals.push(signal);
            value.evidence_trust_tiers = vec!["verified".to_owned()];
            let result = value.evaluate().expect("bounded input");
            assert_eq!(result.risk_level, RiskLevel::High);
            assert_eq!(result.scrutiny, ScrutinyRecommendation::HumanReview);
            assert!(result.advisory_escalation);
            assert!(!result.reasons.is_empty());
        }
    }

    #[test]
    fn semantic_ambiguity_cannot_lower_hard_rule_risk() {
        let mut hard = input();
        hard.signals.push(RiskSignal::Mutation);
        hard.evidence_trust_tiers = vec!["verified".to_owned()];
        let before = hard.evaluate().expect("hard rule");
        hard.semantic_ambiguity = true;
        let after = hard.evaluate().expect("ambiguous hard rule");
        assert_eq!(after.risk_level, before.risk_level);
        assert!(after.scrutiny >= before.scrutiny);
    }

    #[test]
    fn missing_evidence_abstains_without_lowering_risk() {
        let mut value = input();
        value.signals.push(RiskSignal::Privilege);
        let result = value.evaluate().expect("bounded input");
        assert_eq!(result.risk_level, RiskLevel::High);
        assert_eq!(result.state, EvaluationState::Abstained);
        assert!(result.advisory_escalation);
        assert!(
            result
                .reasons
                .contains(&ReasonCode::MissingRequiredEvidence)
        );
    }

    #[test]
    fn safeguard_bypass_indicator_is_advisory_and_ordered() {
        let mut value = input();
        value.prompt_injection_indicators =
            vec![PromptInjectionIndicator::SafeguardBypassInstruction];
        let result = value.evaluate().expect("bounded input");
        assert_eq!(result.risk_level, RiskLevel::Medium);
        assert_eq!(result.scrutiny, ScrutinyRecommendation::Increased);
        assert!(result.advisory_escalation);
        assert_eq!(result.state, EvaluationState::Evaluated);
    }

    #[test]
    fn multiple_reasons_and_requirements_have_stable_order_and_no_duplicates() {
        let mut value = input();
        value.signals = vec![RiskSignal::Mutation, RiskSignal::Shell];
        value.semantic_ambiguity = true;
        value.prompt_injection_indicators =
            vec![PromptInjectionIndicator::SafeguardBypassInstruction];
        value.evidence_trust_tiers = vec!["verified".to_owned()];
        let result = value.evaluate().expect("bounded input");
        let mut reasons = result.reasons.clone();
        reasons.sort_unstable();
        reasons.dedup();
        assert_eq!(result.reasons, reasons);
        let mut requirements = result.evidence_requirements.clone();
        requirements.sort_unstable();
        requirements.dedup();
        assert_eq!(result.evidence_requirements, requirements);
        assert_eq!(result.risk_level, RiskLevel::High);
    }

    #[test]
    fn duplicated_facts_and_malformed_capabilities_fail_closed() {
        let mut value = input();
        value.signals = vec![RiskSignal::Shell, RiskSignal::Shell];
        assert_eq!(value.evaluate(), Err(PolicyError::DuplicateInputFact));
        value = input();
        value.capability = " system.info.read".to_owned();
        assert_eq!(value.evaluate(), Err(PolicyError::InvalidCapability));
    }

    #[test]
    fn unknown_policy_version_and_fields_are_rejected() {
        assert!(PolicyInput::from_json_str(
            r#"{"policy_version":"2","capability":"system.info.read","signals":[],"semantic_ambiguity":false,"prompt_injection_indicators":[],"evidence_trust_tiers":["observed"]}"#
        ).is_err());
        assert!(PolicyInput::from_json_str(
            r#"{"policy_version":"1","capability":"system.info.read","signals":[],"semantic_ambiguity":false,"prompt_injection_indicators":[],"evidence_trust_tiers":["observed"],"allow_mutation":true}"#
        ).is_err());
    }

    #[test]
    fn oversized_json_and_direct_inputs_fail_closed() {
        let oversized_json = format!("{}{}", " ".repeat(MAX_INPUT_JSON_BYTES), "{}");
        assert!(PolicyInput::from_json_str(&oversized_json).is_err());
        assert!(PolicyAssessment::from_json_str(&oversized_json).is_err());

        let mut oversized = input();
        oversized.evidence_trust_tiers = vec![
            "untrusted".to_owned(),
            "observed".to_owned(),
            "verified".to_owned(),
            "policy".to_owned(),
            "untrusted".to_owned(),
        ];
        assert_eq!(oversized.evaluate(), Err(PolicyError::InputLimitExceeded));
    }

    #[test]
    fn unicode_bom_boundary_whitespace_is_rejected() {
        let mut value = input();
        value.capability = "\u{FEFF}system.info.read".to_owned();
        assert_eq!(value.evaluate(), Err(PolicyError::InvalidCapability));
    }
}
