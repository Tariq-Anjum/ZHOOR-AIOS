//! Deterministic, index-first loading and lookup for compact skill descriptors.
//!
//! The loader validates protocol-owned snapshot metadata before it deserializes
//! any descriptors. It stores `artifact_sha256` as an identity value and never
//! opens or verifies a `SKILL.md` body.

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use serde::Deserialize;
use serde_json::Value;
use zhoor_protocol::{
    ArtifactSha256, DescriptorFormatVersion, RegistryContractError, RegistrySnapshotMetadata,
    RegistryVersion,
};

/// Exact activation values present in the canonical compact skills index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activation {
    Always,
    Contextual,
    Explicit,
    Manual,
    Dependency,
}

impl Activation {
    fn parse(value: &str) -> Result<Self, RegistryError> {
        match value {
            "always" => Ok(Self::Always),
            "contextual" => Ok(Self::Contextual),
            "explicit" => Ok(Self::Explicit),
            "manual" => Ok(Self::Manual),
            "dependency" => Ok(Self::Dependency),
            _ => Err(RegistryError::InvalidActivation(value.to_owned())),
        }
    }
}

/// Exact risk labels present in the canonical compact skills index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

impl RiskLevel {
    fn parse(value: &str) -> Result<Self, RegistryError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err(RegistryError::InvalidRiskLevel(value.to_owned())),
        }
    }
}

/// One validated compact descriptor. It contains metadata only, never body data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Descriptor {
    id: String,
    description: String,
    capabilities: Vec<String>,
    requires: Vec<String>,
    activation: Activation,
    risk_level: RiskLevel,
    artifact_sha256: ArtifactSha256,
}

impl Descriptor {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    /// Requirements are exact, case-sensitive context tokens.
    #[must_use]
    pub fn requires(&self) -> &[String] {
        &self.requires
    }

    #[must_use]
    pub const fn activation(&self) -> Activation {
        self.activation
    }

    #[must_use]
    pub const fn risk_level(&self) -> RiskLevel {
        self.risk_level
    }

    #[must_use]
    pub fn artifact_sha256(&self) -> &ArtifactSha256 {
        &self.artifact_sha256
    }
}

/// Caller-supplied facts used while loading the snapshot.
///
/// Every descriptor requirement must exactly match one token in
/// `available_requirements`. The registry performs no host probing or package
/// installation. The optional expected version is checked before descriptors.
#[derive(Debug, Clone, Default)]
pub struct RegistryLoadContext {
    pub available_requirements: BTreeSet<String>,
    pub expected_registry_version: Option<RegistryVersion>,
}

/// A validated snapshot, with descriptors stored in canonical ID order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrySnapshot {
    metadata: RegistrySnapshotMetadata,
    descriptors: Vec<Descriptor>,
}

impl RegistrySnapshot {
    /// Parses and validates one complete JSON snapshot.
    ///
    /// Protocol-owned root metadata and the optional expected version are
    /// checked before descriptor objects are deserialized or validated.
    pub fn from_json_str(
        input: &str,
        context: &RegistryLoadContext,
    ) -> Result<Self, RegistryError> {
        let root: Value = serde_json::from_str(input).map_err(RegistryError::MalformedDocument)?;
        let metadata = RegistrySnapshotMetadata::from_root_value(&root)
            .map_err(RegistryError::from_protocol)?;
        metadata
            .check_expected(context.expected_registry_version.as_ref())
            .map_err(RegistryError::from_protocol)?;

        let wire: RegistryWire =
            serde_json::from_value(root).map_err(RegistryError::MalformedDocument)?;
        let _validated_protocol_fields = (wire.descriptor_format_version, wire.registry_version);

        let mut ids = BTreeSet::new();
        let mut descriptors = Vec::with_capacity(wire.skills.len());
        for item in wire.skills {
            validate_token("id", &item.id)?;
            if !ids.insert(item.id.clone()) {
                return Err(RegistryError::DuplicateId(item.id));
            }
            if item.description.is_empty() {
                return Err(RegistryError::InvalidDescription { id: item.id });
            }
            if item.capabilities.is_empty() {
                return Err(RegistryError::NoCapabilities { id: item.id });
            }
            validate_unique_tokens("capabilities", &item.id, &item.capabilities)?;
            validate_unique_tokens("requires", &item.id, &item.requires)?;
            for requirement in &item.requires {
                if !context.available_requirements.contains(requirement) {
                    return Err(RegistryError::UnsatisfiedRequirement {
                        descriptor_id: item.id.clone(),
                        requirement: requirement.clone(),
                    });
                }
            }
            let activation = Activation::parse(&item.activation)?;
            let risk_level = RiskLevel::parse(&item.risk_level)?;
            descriptors.push(Descriptor {
                id: item.id,
                description: item.description,
                capabilities: item.capabilities,
                requires: item.requires,
                activation,
                risk_level,
                artifact_sha256: item.artifact_sha256,
            });
        }

        descriptors.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(Self {
            metadata,
            descriptors,
        })
    }

    #[must_use]
    pub const fn metadata(&self) -> &RegistrySnapshotMetadata {
        &self.metadata
    }

    #[must_use]
    pub fn descriptors(&self) -> &[Descriptor] {
        &self.descriptors
    }

    /// Returns the exact, case-sensitive ID match, if present.
    #[must_use]
    pub fn find_by_id(&self, id: &str) -> Option<&Descriptor> {
        self.descriptors
            .binary_search_by(|descriptor| descriptor.id.as_str().cmp(id))
            .ok()
            .map(|index| &self.descriptors[index])
    }

    /// Returns exact, case-sensitive capability matches in ascending ID order.
    #[must_use]
    pub fn find_by_capability(&self, capability: &str) -> Vec<&Descriptor> {
        self.descriptors
            .iter()
            .filter(|descriptor| {
                descriptor
                    .capabilities
                    .iter()
                    .any(|item| item == capability)
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryWire {
    descriptor_format_version: DescriptorFormatVersion,
    registry_version: RegistryVersion,
    skills: Vec<DescriptorWire>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DescriptorWire {
    id: String,
    description: String,
    capabilities: Vec<String>,
    requires: Vec<String>,
    activation: String,
    risk_level: String,
    artifact_sha256: ArtifactSha256,
}

/// Deterministic registry parsing and validation errors.
#[derive(Debug)]
pub enum RegistryError {
    Protocol(RegistryContractError),
    RegistryVersionMismatch {
        expected: String,
        found: String,
    },
    MalformedDocument(serde_json::Error),
    InvalidToken {
        field: &'static str,
        value: String,
    },
    InvalidDescription {
        id: String,
    },
    NoCapabilities {
        id: String,
    },
    DuplicateId(String),
    DuplicateValue {
        descriptor_id: String,
        field: &'static str,
        value: String,
    },
    InvalidActivation(String),
    InvalidRiskLevel(String),
    UnsatisfiedRequirement {
        descriptor_id: String,
        requirement: String,
    },
}

impl RegistryError {
    fn from_protocol(error: RegistryContractError) -> Self {
        match error {
            RegistryContractError::RegistryVersionMismatch { expected, found } => {
                Self::RegistryVersionMismatch { expected, found }
            }
            other => Self::Protocol(other),
        }
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protocol(error) => write!(formatter, "registry protocol metadata: {error}"),
            Self::RegistryVersionMismatch { expected, found } => write!(
                formatter,
                "registry snapshot version mismatch: expected {expected:?}, found {found:?}"
            ),
            Self::MalformedDocument(error) => write!(formatter, "malformed registry JSON: {error}"),
            Self::InvalidToken { field, value } => {
                write!(formatter, "invalid {field} token: {value:?}")
            }
            Self::InvalidDescription { id } => {
                write!(formatter, "descriptor {id:?} has an empty description")
            }
            Self::NoCapabilities { id } => {
                write!(formatter, "descriptor {id:?} has no capabilities")
            }
            Self::DuplicateId(id) => write!(formatter, "duplicate descriptor ID: {id:?}"),
            Self::DuplicateValue {
                descriptor_id,
                field,
                value,
            } => write!(
                formatter,
                "descriptor {descriptor_id:?} repeats {field} value {value:?}"
            ),
            Self::InvalidActivation(value) => {
                write!(formatter, "invalid activation value: {value:?}")
            }
            Self::InvalidRiskLevel(value) => {
                write!(formatter, "invalid risk_level value: {value:?}")
            }
            Self::UnsatisfiedRequirement {
                descriptor_id,
                requirement,
            } => write!(
                formatter,
                "descriptor {descriptor_id:?} requires unavailable token {requirement:?}"
            ),
        }
    }
}

impl StdError for RegistryError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Protocol(error) => Some(error),
            Self::MalformedDocument(error) => Some(error),
            _ => None,
        }
    }
}

fn validate_token(field: &'static str, value: &str) -> Result<(), RegistryError> {
    let mut bytes = value.bytes();
    let valid_first = bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit());
    let valid_rest = bytes.all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
    });
    if valid_first && valid_rest {
        Ok(())
    } else {
        Err(RegistryError::InvalidToken {
            field,
            value: value.to_owned(),
        })
    }
}

fn validate_unique_tokens(
    field: &'static str,
    descriptor_id: &str,
    values: &[String],
) -> Result<(), RegistryError> {
    let mut seen = BTreeSet::new();
    for value in values {
        validate_token(field, value)?;
        if !seen.insert(value) {
            return Err(RegistryError::DuplicateValue {
                descriptor_id: descriptor_id.to_owned(),
                field,
                value: value.clone(),
            });
        }
    }
    Ok(())
}
