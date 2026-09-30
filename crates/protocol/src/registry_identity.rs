//! Shared registry snapshot identity primitives.
//!
//! This module validates only protocol-owned metadata. The registry package
//! owns the complete root schema, descriptor entries, ordering, and lookup.
//! A caller should parse this metadata before processing any entries.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

/// The only descriptor field format supported by protocol v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DescriptorFormatVersion {
    /// Descriptor field contract version 1.
    #[serde(rename = "1")]
    V1,
}

impl DescriptorFormatVersion {
    /// Parses the exact supported wire value.
    pub fn parse(value: &str) -> Result<Self, RegistryContractError> {
        match value {
            "1" => Ok(Self::V1),
            other => Err(RegistryContractError::UnsupportedDescriptorFormatVersion(
                other.to_owned(),
            )),
        }
    }

    /// Returns the canonical wire value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

impl<'de> Deserialize<'de> for DescriptorFormatVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

/// An opaque registry snapshot label using the existing protocol version-label
/// grammar: 1–128 ASCII characters, starting with an ASCII letter or digit,
/// followed only by ASCII letters, digits, `.`, `_`, `+`, or `-`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct RegistryVersion(String);

impl RegistryVersion {
    /// Validates and constructs a registry snapshot label.
    pub fn parse(value: impl Into<String>) -> Result<Self, RegistryContractError> {
        let value = value.into();
        if valid_registry_version(&value) {
            Ok(Self(value))
        } else {
            Err(RegistryContractError::MalformedField {
                field: "registry_version",
            })
        }
    }

    /// Returns the canonical wire value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for RegistryVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// A lowercase textual SHA-256 content identifier for exact raw `SKILL.md`
/// bytes. It has no authentication or trust semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ArtifactSha256(String);

impl ArtifactSha256 {
    /// Validates the exact `sha256:<64 lowercase hexadecimal characters>`
    /// representation.
    pub fn parse(value: impl Into<String>) -> Result<Self, RegistryContractError> {
        let value = value.into();
        let Some(digest) = value.strip_prefix("sha256:") else {
            return Err(RegistryContractError::MalformedArtifactHash);
        };
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(RegistryContractError::MalformedArtifactHash);
        }
        Ok(Self(value))
    }

    /// Computes the identifier for exact raw `SKILL.md` bytes.
    #[must_use]
    pub fn for_skill_md_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        let mut value = String::with_capacity("sha256:".len() + 64);
        value.push_str("sha256:");
        for byte in digest {
            use fmt::Write as _;
            write!(&mut value, "{byte:02x}").expect("writing to String cannot fail");
        }
        Self(value)
    }

    /// Compares this identifier with the exact raw bytes of a `SKILL.md` file.
    #[must_use]
    pub fn matches_skill_md_bytes(&self, bytes: &[u8]) -> bool {
        *self == Self::for_skill_md_bytes(bytes)
    }

    /// Returns the canonical wire value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ArtifactSha256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// The two protocol-owned fields carried once at a registry snapshot root.
/// The registry implementation must validate these before reading entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RegistrySnapshotMetadata {
    /// Descriptor field contract used by every entry in this snapshot.
    pub descriptor_format_version: DescriptorFormatVersion,
    /// Snapshot identity label, scoped to the same configured canonical
    /// registry source and not globally unique.
    pub registry_version: RegistryVersion,
}

impl RegistrySnapshotMetadata {
    /// Extracts protocol-owned fields from a registry root before the registry
    /// package processes entries. Registry-owned root keys are intentionally
    /// left to the complete registry schema and validator.
    pub fn from_root_value(root: &serde_json::Value) -> Result<Self, RegistryContractError> {
        let object = root
            .as_object()
            .ok_or(RegistryContractError::MalformedField {
                field: "registry root",
            })?;
        let descriptor_format_version = object
            .get("descriptor_format_version")
            .ok_or(RegistryContractError::MissingField {
                field: "descriptor_format_version",
            })?
            .as_str()
            .ok_or(RegistryContractError::MalformedField {
                field: "descriptor_format_version",
            })?;
        let registry_version = object
            .get("registry_version")
            .ok_or(RegistryContractError::MissingField {
                field: "registry_version",
            })?
            .as_str()
            .ok_or(RegistryContractError::MalformedField {
                field: "registry_version",
            })?;
        Ok(Self {
            descriptor_format_version: DescriptorFormatVersion::parse(descriptor_format_version)?,
            registry_version: RegistryVersion::parse(registry_version.to_owned())?,
        })
    }

    /// Checks the caller's optional expected snapshot label. A mismatch is an
    /// explicit error and never causes a current snapshot to be substituted.
    pub fn check_expected(
        &self,
        expected: Option<&RegistryVersion>,
    ) -> Result<(), RegistryContractError> {
        if let Some(expected) = expected
            && *expected != self.registry_version
        {
            return Err(RegistryContractError::RegistryVersionMismatch {
                expected: expected.as_str().to_owned(),
                found: self.registry_version.as_str().to_owned(),
            });
        }
        Ok(())
    }
}

/// Errors for protocol-owned registry metadata and content identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryContractError {
    /// A registry root omitted one required protocol-owned field.
    MissingField { field: &'static str },
    /// A field has the wrong JSON shape or text grammar.
    MalformedField { field: &'static str },
    /// A hash is not the exact lowercase SHA-256 wire representation.
    MalformedArtifactHash,
    /// A descriptor format version is not the sole supported v1 value.
    UnsupportedDescriptorFormatVersion(String),
    /// The loaded snapshot differs from a caller's expected registry label.
    RegistryVersionMismatch { expected: String, found: String },
}

impl fmt::Display for RegistryContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { field } => write!(formatter, "missing required field: {field}"),
            Self::MalformedField { field } => write!(formatter, "malformed field: {field}"),
            Self::MalformedArtifactHash => write!(
                formatter,
                "artifact_sha256 must be sha256 followed by 64 lowercase hexadecimal characters"
            ),
            Self::UnsupportedDescriptorFormatVersion(found) => {
                write!(
                    formatter,
                    "unsupported descriptor format version: {found:?}"
                )
            }
            Self::RegistryVersionMismatch { expected, found } => write!(
                formatter,
                "registry snapshot version mismatch: expected {expected:?}, found {found:?}"
            ),
        }
    }
}

impl std::error::Error for RegistryContractError {}

fn valid_registry_version(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 128
        && bytes[0].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
}

#[cfg(test)]
mod tests {
    use super::{
        ArtifactSha256, DescriptorFormatVersion, RegistryContractError, RegistrySnapshotMetadata,
        RegistryVersion,
    };

    #[test]
    fn accepts_supported_descriptor_and_registry_versions() {
        assert_eq!(
            DescriptorFormatVersion::parse("1").unwrap(),
            DescriptorFormatVersion::V1
        );
        assert_eq!(
            RegistryVersion::parse("skills-2026.09").unwrap().as_str(),
            "skills-2026.09"
        );
    }

    #[test]
    fn rejects_unsupported_or_malformed_versions() {
        assert!(matches!(
            DescriptorFormatVersion::parse("2"),
            Err(RegistryContractError::UnsupportedDescriptorFormatVersion(_))
        ));
        assert!(matches!(
            RegistryVersion::parse("contains space"),
            Err(RegistryContractError::MalformedField {
                field: "registry_version"
            })
        ));
    }

    #[test]
    fn validates_hash_grammar_and_raw_skill_bytes() {
        let correct = ArtifactSha256::parse(
            "sha256:2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        )
        .unwrap();
        assert!(correct.matches_skill_md_bytes(b"hello"));
        assert!(!correct.matches_skill_md_bytes(b"Hello"));
        assert_eq!(
            ArtifactSha256::for_skill_md_bytes(b"hello"),
            correct,
            "SHA-256 covers the exact bytes without normalization"
        );
        for malformed in [
            "sha512:2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            "sha256:2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b982",
            "sha256:2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824",
        ] {
            assert!(matches!(
                ArtifactSha256::parse(malformed),
                Err(RegistryContractError::MalformedArtifactHash)
            ));
        }
    }

    #[test]
    fn expected_registry_version_mismatch_fails_explicitly() {
        let metadata = RegistrySnapshotMetadata {
            descriptor_format_version: DescriptorFormatVersion::V1,
            registry_version: RegistryVersion::parse("snapshot-a").unwrap(),
        };
        let expected = RegistryVersion::parse("snapshot-b").unwrap();
        assert!(matches!(
            metadata.check_expected(Some(&expected)),
            Err(RegistryContractError::RegistryVersionMismatch { .. })
        ));
        assert!(
            metadata
                .check_expected(Some(&metadata.registry_version))
                .is_ok()
        );
        assert!(metadata.check_expected(None).is_ok());
    }
}
