use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::Validator;
use serde_json::{Value, json};
use zhoor_protocol::{RegistryContractError, RegistryVersion};
use zhoor_registry::{RegistryError, RegistryLoadContext, RegistrySnapshot};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("fixture or schema must be readable"))
        .expect("fixture or schema must be valid JSON")
}

fn schema() -> Value {
    read_json(&repo_root().join("schemas/registry/registry-snapshot-v1.schema.json"))
}

fn compiled_schema() -> Validator {
    jsonschema::draft202012::new(&schema()).expect("registry schema must compile")
}

fn valid_fixture() -> Value {
    read_json(&repo_root().join("fixtures/registry/valid/registry-v1.json"))
}

fn context(requirements: &[&str], expected: Option<&str>) -> RegistryLoadContext {
    RegistryLoadContext {
        available_requirements: requirements
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        expected_registry_version: expected.map(|value| RegistryVersion::parse(value).unwrap()),
    }
}

fn assert_schema_and_rust_agree(document: &Value, ctx: &RegistryLoadContext, expected: bool) {
    let schema_accepts = compiled_schema().is_valid(document);
    let rust_accepts = RegistrySnapshot::from_json_str(&document.to_string(), ctx).is_ok();
    assert_eq!(schema_accepts, expected, "schema result for {document}");
    assert_eq!(rust_accepts, expected, "Rust result for {document}");
}

#[test]
fn valid_snapshot_loads_after_context_and_version_checks() {
    let document = valid_fixture();
    assert_schema_and_rust_agree(
        &document,
        &context(&["linux"], Some("skills-2026.09")),
        true,
    );

    let loaded = RegistrySnapshot::from_json_str(
        &document.to_string(),
        &context(&["linux"], Some("skills-2026.09")),
    )
    .unwrap();
    assert_eq!(
        loaded.metadata().registry_version.as_str(),
        "skills-2026.09"
    );
    assert_eq!(
        loaded
            .descriptors()
            .iter()
            .map(|item| item.id())
            .collect::<Vec<_>>(),
        ["alpha-tool", "zeta-tool"]
    );
    assert_eq!(
        loaded.find_by_id("zeta-tool").unwrap().capabilities().len(),
        2
    );
    assert!(loaded.find_by_id("Zeta-tool").is_none());
    assert_eq!(
        loaded
            .find_by_capability("inspect_service")
            .iter()
            .map(|item| item.id())
            .collect::<Vec<_>>(),
        ["alpha-tool", "zeta-tool"]
    );
    assert_eq!(
        loaded.find_by_capability("read_journal")[0].id(),
        "zeta-tool"
    );
}

#[test]
fn canonical_activation_and_risk_values_are_accepted() {
    let ctx = context(&["linux"], None);
    for (activation, risk_level) in [
        ("always", "low"),
        ("contextual", "medium"),
        ("explicit", "high"),
        ("manual", "low"),
        ("dependency", "medium"),
    ] {
        let mut document = valid_fixture();
        document["skills"][0]["activation"] = json!(activation);
        document["skills"][0]["risk_level"] = json!(risk_level);
        assert_schema_and_rust_agree(&document, &ctx, true);
    }
}

#[test]
fn protocol_metadata_schema_definitions_match_the_protocol_contract() {
    let registry_schema = schema();
    let protocol_schema =
        read_json(&repo_root().join("schemas/protocol/registry-snapshot-v1.schema.json"));
    for definition in [
        "descriptorFormatVersion",
        "registryVersion",
        "artifactSha256",
    ] {
        assert_eq!(
            registry_schema["$defs"][definition], protocol_schema["$defs"][definition],
            "registry schema must retain the protocol-owned {definition} contract"
        );
    }
}

#[test]
fn root_metadata_is_rejected_before_invalid_descriptors_are_processed() {
    let mut document = valid_fixture();
    document["descriptor_format_version"] = json!("2");
    document["skills"] = json!([{"id": false}]);
    assert!(matches!(
        RegistrySnapshot::from_json_str(&document.to_string(), &RegistryLoadContext::default()),
        Err(RegistryError::Protocol(
            RegistryContractError::UnsupportedDescriptorFormatVersion(_)
        ))
    ));
}

#[test]
fn expected_registry_version_mismatch_is_explicit() {
    let document = valid_fixture();
    assert!(matches!(
        RegistrySnapshot::from_json_str(
            &document.to_string(),
            &context(&["linux"], Some("skills-2026.10"))
        ),
        Err(RegistryError::RegistryVersionMismatch { expected, found })
            if expected == "skills-2026.10" && found == "skills-2026.09"
    ));
}

#[test]
fn schema_and_rust_reject_root_descriptor_and_protocol_metadata_errors() {
    let ctx = context(&["linux"], None);
    for (field, value) in [
        ("descriptor_format_version", Value::Null),
        ("descriptor_format_version", json!(2)),
        ("descriptor_format_version", json!("2")),
        ("registry_version", Value::Null),
        ("registry_version", json!("bad value")),
        ("registry_version", json!("x".repeat(129))),
    ] {
        let mut document = valid_fixture();
        document[field] = value;
        assert_schema_and_rust_agree(&document, &ctx, false);
    }

    for field in ["descriptor_format_version", "registry_version"] {
        let mut document = valid_fixture();
        document.as_object_mut().unwrap().remove(field);
        assert_schema_and_rust_agree(&document, &ctx, false);
    }
}

#[test]
fn schema_and_rust_reject_duplicate_ids_invalid_metadata_and_unknown_fields() {
    let ctx = context(&["linux"], None);
    let mut cases = Vec::new();

    let mut duplicate_ids = valid_fixture();
    duplicate_ids["skills"][1]["id"] = json!("zeta-tool");
    // JSON Schema has no standard constraint for uniqueness of one selected
    // property across array objects; this cross-entry rule belongs to Rust.
    assert!(compiled_schema().is_valid(&duplicate_ids));
    assert!(matches!(
        RegistrySnapshot::from_json_str(&duplicate_ids.to_string(), &ctx),
        Err(RegistryError::DuplicateId(id)) if id == "zeta-tool"
    ));

    let mut invalid_activation = valid_fixture();
    invalid_activation["skills"][0]["activation"] = json!("sometimes");
    cases.push(invalid_activation);

    let mut invalid_risk = valid_fixture();
    invalid_risk["skills"][0]["risk_level"] = json!("critical");
    cases.push(invalid_risk);

    let mut malformed_id = valid_fixture();
    malformed_id["skills"][0]["id"] = json!("Upper Case");
    cases.push(malformed_id);

    let mut empty_description = valid_fixture();
    empty_description["skills"][0]["description"] = json!("");
    cases.push(empty_description);

    let mut empty_capabilities = valid_fixture();
    empty_capabilities["skills"][0]["capabilities"] = json!([]);
    cases.push(empty_capabilities);

    let mut duplicate_capability = valid_fixture();
    duplicate_capability["skills"][0]["capabilities"] = json!(["read_journal", "read_journal"]);
    cases.push(duplicate_capability);

    let mut duplicate_requirement = valid_fixture();
    duplicate_requirement["skills"][0]["requires"] = json!(["linux", "linux"]);
    cases.push(duplicate_requirement);

    let mut unknown_descriptor = valid_fixture();
    unknown_descriptor["skills"][0]["body_path"] = json!("skills/zeta-tool/SKILL.md");
    cases.push(unknown_descriptor);

    let mut unknown_root = valid_fixture();
    unknown_root["schema_version"] = json!(1);
    cases.push(unknown_root);

    let mut malformed_metadata = valid_fixture();
    malformed_metadata["skills"][0]["capabilities"] = json!("inspect_service");
    cases.push(malformed_metadata);

    let mut missing_descriptor_field = valid_fixture();
    missing_descriptor_field["skills"][0]
        .as_object_mut()
        .unwrap()
        .remove("artifact_sha256");
    cases.push(missing_descriptor_field);

    for document in cases {
        assert_schema_and_rust_agree(&document, &ctx, false);
    }
}

#[test]
fn schema_and_rust_reject_bad_hashes_and_unsatisfied_requirements() {
    let ctx = context(&[], None);
    let base = valid_fixture();
    for hash in [
        "sha1:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    ] {
        let mut document = base.clone();
        document["skills"][0]["artifact_sha256"] = json!(hash);
        assert_schema_and_rust_agree(&document, &ctx, false);
    }

    assert!(matches!(
        RegistrySnapshot::from_json_str(&base.to_string(), &ctx),
        Err(RegistryError::UnsatisfiedRequirement { descriptor_id, requirement })
            if descriptor_id == "zeta-tool" && requirement == "linux"
    ));
    // Schema validates shape; satisfiability is intentionally contextual Rust validation.
    assert!(compiled_schema().is_valid(&base));
}

#[test]
fn registry_and_descriptor_objects_are_closed() {
    let mut document = valid_fixture();
    document["extra"] = json!(true);
    assert!(!compiled_schema().is_valid(&document));

    let mut document = valid_fixture();
    document["skills"][0]["extra"] = json!(true);
    assert!(!compiled_schema().is_valid(&document));
    assert!(
        RegistrySnapshot::from_json_str(&document.to_string(), &RegistryLoadContext::default())
            .is_err()
    );
}
