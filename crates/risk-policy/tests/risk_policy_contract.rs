//! Schema/Rust parity and fixture inventory for POL-01.

use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::Validator;
use serde_json::Value;
use zhoor_risk_policy::{PolicyAssessment, PolicyInput};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("fixture/schema must be readable"))
        .expect("fixture/schema must be valid JSON")
}

fn input_schema() -> Validator {
    let path = repo_root().join("schemas/risk-policy/policy-input-v1.schema.json");
    jsonschema::draft202012::new(&read_json(&path)).expect("input schema must compile")
}

fn assessment_schema() -> Validator {
    let path = repo_root().join("schemas/risk-policy/policy-assessment-v1.schema.json");
    jsonschema::draft202012::new(&read_json(&path)).expect("assessment schema must compile")
}

fn json_files(path: &Path) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(path)
        .expect("fixture directory must exist")
        .map(|entry| entry.expect("fixture entry must be readable").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect();
    files.sort();
    files
}

#[test]
fn valid_fixtures_match_schema_rust_and_expected_assessments() {
    let schema = input_schema();
    let output_schema = assessment_schema();
    let directory = repo_root().join("fixtures/risk-policy/valid");
    let fixtures = json_files(&directory);
    assert!(
        !fixtures.is_empty(),
        "valid fixture coverage must not disappear"
    );

    for path in fixtures {
        let fixture = read_json(&path);
        let input_value = &fixture["input"];
        assert!(
            schema.is_valid(input_value),
            "input schema rejects {}",
            path.display()
        );
        let input: PolicyInput = serde_json::from_value(input_value.clone())
            .unwrap_or_else(|error| panic!("Rust rejects {}: {error}", path.display()));
        let actual = serde_json::to_value(input.evaluate().expect("evaluation succeeds"))
            .expect("assessment serializes");
        assert_eq!(
            actual,
            fixture["expected"],
            "assessment mismatch: {}",
            path.display()
        );
        assert!(
            output_schema.is_valid(&actual),
            "assessment schema rejects {}",
            path.display()
        );
    }
}

#[test]
fn invalid_fixtures_fail_schema_and_rust_closed() {
    let schema = input_schema();
    let directory = repo_root().join("fixtures/risk-policy/invalid");
    let fixtures = json_files(&directory);
    assert!(
        !fixtures.is_empty(),
        "invalid fixture coverage must not disappear"
    );

    for path in fixtures {
        let value = read_json(&path);
        assert!(
            !schema.is_valid(&value),
            "schema unexpectedly accepts {}",
            path.display()
        );
        let rust_rejects = match serde_json::from_value::<PolicyInput>(value) {
            Err(_) => true,
            Ok(input) => input.evaluate().is_err(),
        };
        assert!(rust_rejects, "Rust unexpectedly accepts {}", path.display());
    }
}

#[test]
fn invalid_assessments_fail_schema_and_rust_closed() {
    let schema = assessment_schema();
    let directory = repo_root().join("fixtures/risk-policy/invalid-assessment");
    let fixtures = json_files(&directory);
    assert!(
        !fixtures.is_empty(),
        "invalid assessment coverage must not disappear"
    );

    for path in fixtures {
        let value = read_json(&path);
        assert!(
            !schema.is_valid(&value),
            "schema unexpectedly accepts {}",
            path.display()
        );
        assert!(
            serde_json::from_value::<PolicyAssessment>(value).is_err(),
            "Rust unexpectedly accepts {}",
            path.display()
        );
    }
}
