//! PRO-01 contract test suite for the ZHOOR decision protocol v1.
//!
//! This suite enumerates every committed valid and invalid protocol fixture
//! and pins, for each one, the deterministic Rust validation expectation
//! (accept, or reject with one specific typed [`ErrorKind`]) plus
//! deterministic Draft 2020-12 schema parity, round-trip, pairing, coverage,
//! and hygiene guarantees. Cross-envelope semantic checks are tested
//! separately because they require the paired request.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::Validator;
use serde_json::Value;

use zhoor_protocol::{ErrorKind, ProtocolError, Request, Response};

/// Returns the repository root relative to this package.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Returns a fixture directory for `valid` or `invalid`.
fn fixture_dir(kind: &str) -> PathBuf {
    repo_root().join("fixtures").join("protocol").join(kind)
}

/// Returns the canonical schema directory.
fn schema_dir() -> PathBuf {
    repo_root().join("schemas").join("protocol")
}

fn compiled_schema(name: &str) -> Validator {
    let document = read_json(&schema_dir().join(name));
    jsonschema::draft202012::new(&document).unwrap_or_else(|error| {
        panic!("canonical Draft 2020-12 schema {name} must compile: {error}")
    })
}

fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

/// Recursively enumerates every JSON file. Unexpected non-JSON files fail
/// closed instead of silently escaping schema/Rust fixture coverage.
fn collect_json_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to list {}: {error}", directory.display()));
    for entry in entries {
        let entry =
            entry.unwrap_or_else(|error| panic!("directory entry error in {directory:?}: {error}"));
        let path = entry.path();
        let file_type = entry
            .file_type()
            .unwrap_or_else(|error| panic!("failed to inspect {}: {error}", path.display()));
        if file_type.is_dir() {
            collect_json_files(&path, files);
        } else if file_type.is_file() {
            assert_eq!(
                path.extension().and_then(|extension| extension.to_str()),
                Some("json"),
                "unexpected non-JSON contract file {}; include it explicitly in validation",
                path.display()
            );
            files.push(path);
        } else {
            panic!("unsupported fixture entry type: {}", path.display());
        }
    }
}

/// Lists all JSON files under a directory, sorted; panics on an empty set so
/// a lost fixture directory cannot silently reduce coverage.
fn json_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_json_files(directory, &mut files);
    files.sort();
    assert!(
        !files.is_empty(),
        "no JSON files found in {}; fixture coverage must not silently shrink",
        directory.display()
    );
    files
}

fn fixture_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .expect("fixture file name must be UTF-8")
        .to_owned()
}

/// Pinned typed-error expectation for every invalid fixture.
const INVALID_EXPECTATIONS: &[(&str, ErrorKind)] = &[
    (
        "request-unknown-version.json",
        ErrorKind::UnsupportedVersion,
    ),
    (
        "request-unknown-operation.json",
        ErrorKind::UnknownOperation,
    ),
    (
        "request-unknown-worker-kind.json",
        ErrorKind::UnknownWorkerKind,
    ),
    ("request-missing-request-id.json", ErrorKind::MissingField),
    ("request-malformed-request-id.json", ErrorKind::InvalidId),
    ("request-malformed-goal.json", ErrorKind::MalformedJson),
    ("request-unknown-field.json", ErrorKind::UnknownField),
    (
        "request-unknown-trust-tier.json",
        ErrorKind::UnknownTrustTier,
    ),
    (
        "request-malformed-evidence-ref.json",
        ErrorKind::InvalidEvidenceReference,
    ),
    ("request-invalid-budget.json", ErrorKind::InvalidBudget),
    ("request-over-budget.json", ErrorKind::OverBudget),
    ("request-invalid-deadline.json", ErrorKind::InvalidDeadline),
    (
        "request-inconsistent-envelope.json",
        ErrorKind::InconsistentEnvelope,
    ),
    (
        "response-unknown-result-kind.json",
        ErrorKind::UnknownResultKind,
    ),
    (
        "response-invalid-disposition.json",
        ErrorKind::InvalidDisposition,
    ),
    (
        "response-unknown-trust-tier.json",
        ErrorKind::UnknownTrustTier,
    ),
    (
        "response-invalid-confidence.json",
        ErrorKind::InvalidConfidence,
    ),
    ("response-unknown-field.json", ErrorKind::UnknownField),
    ("response-missing-request-id.json", ErrorKind::MissingField),
    ("response-malformed-trace-id.json", ErrorKind::InvalidId),
    (
        "response-malformed-evidence-ref.json",
        ErrorKind::InvalidEvidenceReference,
    ),
    (
        "response-result-and-error.json",
        ErrorKind::InconsistentEnvelope,
    ),
    ("response-unknown-backend.json", ErrorKind::UnknownBackend),
    ("response-invalid-score.json", ErrorKind::InvalidResult),
    ("response-invalid-verdict.json", ErrorKind::InvalidResult),
];

/// Pairing between every response fixture (valid and invalid) and the valid
/// request fixture it must be validated against for the cross-envelope
/// rules.
const RESPONSE_PAIRS: &[(&str, &str)] = &[
    ("response-choice.json", "request-select-minimal.json"),
    ("response-score.json", "request-score.json"),
    ("response-extract.json", "request-extract.json"),
    ("response-verdict.json", "request-verify.json"),
    ("response-verdict-insufficient.json", "request-verify.json"),
    ("response-abstained.json", "request-score.json"),
    ("response-escalated.json", "request-score.json"),
    ("response-denied.json", "request-score.json"),
    ("response-unknown-result-kind.json", "request-score.json"),
    ("response-invalid-disposition.json", "request-score.json"),
    ("response-unknown-trust-tier.json", "request-score.json"),
    ("response-invalid-confidence.json", "request-score.json"),
    ("response-unknown-field.json", "request-score.json"),
    ("response-missing-request-id.json", "request-score.json"),
    ("response-malformed-trace-id.json", "request-score.json"),
    ("response-malformed-evidence-ref.json", "request-score.json"),
    ("response-result-and-error.json", "request-score.json"),
    ("response-unknown-backend.json", "request-score.json"),
    ("response-invalid-score.json", "request-score.json"),
    ("response-invalid-verdict.json", "request-verify.json"),
];

/// Field-name substrings that must never appear in accepted contracts or
/// valid fixtures. The generic token `token` is intentionally excluded
/// because `input_token_budget` is a bounded resource budget, not a secret;
/// specific credential token names are forbidden instead.
const FORBIDDEN_KEY_SUBSTRINGS: &[&str] = &[
    "secret",
    "password",
    "credential",
    "api_key",
    "apikey",
    "access_token",
    "refresh_token",
    "private_key",
    "authorization",
    "authoriz",
    "authoris",
    "approve",
    "grant",
    "execute",
    "reasoning",
    "chain_of_thought",
    "transcript",
    "scratchpad",
    "hidden",
];

/// Parsed envelope with its cross-envelope validation already applied for
/// responses.
#[derive(Debug)]
enum Envelope {
    Request(Request),
    Response(Response),
}

fn paired_request(response_name: &str) -> Option<Request> {
    RESPONSE_PAIRS
        .iter()
        .find(|(response, _)| *response == response_name)
        .map(|(_, request_name)| {
            let path = fixture_dir("valid").join(request_name);
            Request::from_value(&read_json(&path)).unwrap_or_else(|error| {
                panic!("paired request fixture {request_name} must be valid: {error}")
            })
        })
}

fn parse_envelope(name: &str, document: &Value) -> Result<Envelope, ProtocolError> {
    if name.starts_with("request-") {
        Ok(Envelope::Request(Request::from_value(document)?))
    } else if name.starts_with("response-") {
        let response = Response::from_value(document)?;
        if let Some(request) = paired_request(name) {
            response.validate_against(&request)?;
        }
        Ok(Envelope::Response(response))
    } else {
        panic!("fixture {name} does not start with request- or response-");
    }
}

fn assert_round_trip(envelope: &Envelope, name: &str) {
    match envelope {
        Envelope::Request(request) => {
            let serialized = serde_json::to_string(request)
                .unwrap_or_else(|error| panic!("serialization failed for {name}: {error}"));
            let reparsed = Request::from_json_str(&serialized)
                .unwrap_or_else(|error| panic!("reparse failed for {name}: {error}"));
            assert_eq!(&reparsed, request, "round-trip value mismatch for {name}");
            let again = serde_json::to_string(&reparsed)
                .unwrap_or_else(|error| panic!("reserialization failed for {name}: {error}"));
            assert_eq!(
                again, serialized,
                "serialization is not a fixed point for {name}"
            );
        }
        Envelope::Response(response) => {
            let serialized = serde_json::to_string(response)
                .unwrap_or_else(|error| panic!("serialization failed for {name}: {error}"));
            let reparsed = Response::from_json_str(&serialized)
                .unwrap_or_else(|error| panic!("reparse failed for {name}: {error}"));
            assert_eq!(&reparsed, response, "round-trip value mismatch for {name}");
            let again = serde_json::to_string(&reparsed)
                .unwrap_or_else(|error| panic!("reserialization failed for {name}: {error}"));
            assert_eq!(
                again, serialized,
                "serialization is not a fixed point for {name}"
            );
        }
    }
}

fn expected_error(name: &str) -> ErrorKind {
    INVALID_EXPECTATIONS
        .iter()
        .find(|(fixture, _)| *fixture == name)
        .map(|(_, kind)| *kind)
        .unwrap_or_else(|| panic!("no expected typed error pinned for invalid fixture {name}"))
}

fn error_kind_samples() -> Vec<(ErrorKind, ProtocolError)> {
    vec![
        (
            ErrorKind::UnsupportedVersion,
            ProtocolError::UnsupportedVersion {
                found: "2".to_owned(),
            },
        ),
        (
            ErrorKind::UnknownOperation,
            ProtocolError::UnknownOperation {
                found: "rank".to_owned(),
            },
        ),
        (
            ErrorKind::UnknownWorkerKind,
            ProtocolError::UnknownWorkerKind {
                found: "extractor".to_owned(),
            },
        ),
        (
            ErrorKind::UnknownResultKind,
            ProtocolError::UnknownResultKind {
                found: "ranking".to_owned(),
            },
        ),
        (
            ErrorKind::InvalidDisposition,
            ProtocolError::InvalidDisposition {
                found: "abandon".to_owned(),
            },
        ),
        (
            ErrorKind::UnknownTrustTier,
            ProtocolError::UnknownTrustTier {
                found: "trusted".to_owned(),
            },
        ),
        (
            ErrorKind::UnknownBackend,
            ProtocolError::UnknownBackend {
                found: "cloud".to_owned(),
            },
        ),
        (
            ErrorKind::InvalidConfidence,
            ProtocolError::InvalidConfidence {
                field: "raw",
                found: "1.5".to_owned(),
            },
        ),
        (
            ErrorKind::InvalidBudget,
            ProtocolError::InvalidBudget { reason: "sample" },
        ),
        (
            ErrorKind::InvalidDeadline,
            ProtocolError::InvalidDeadline { reason: "sample" },
        ),
        (
            ErrorKind::InvalidId,
            ProtocolError::InvalidId {
                field: "request_id",
                found: "bad".to_owned(),
            },
        ),
        (
            ErrorKind::MissingField,
            ProtocolError::MissingField {
                field: "request_id",
            },
        ),
        (
            ErrorKind::UnknownField,
            ProtocolError::UnknownField {
                field: "extra".to_owned(),
            },
        ),
        (
            ErrorKind::InvalidEvidenceReference,
            ProtocolError::InvalidEvidenceReference { reason: "sample" },
        ),
        (
            ErrorKind::OverBudget,
            ProtocolError::OverBudget {
                reason: "sample".to_owned(),
            },
        ),
        (
            ErrorKind::InconsistentEnvelope,
            ProtocolError::InconsistentEnvelope {
                reason: "sample".to_owned(),
            },
        ),
        (
            ErrorKind::InvalidResult,
            ProtocolError::InvalidResult { reason: "sample" },
        ),
        (
            ErrorKind::MalformedJson,
            ProtocolError::MalformedJson { reason: "sample" },
        ),
    ]
}

fn scan_keys(value: &Value, path: &str, problems: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let lowered = key.to_lowercase();
                for forbidden in FORBIDDEN_KEY_SUBSTRINGS {
                    if lowered.contains(forbidden) {
                        problems.push(format!("{path}.{key}"));
                    }
                }
                scan_keys(child, &format!("{path}.{key}"), problems);
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                scan_keys(item, &format!("{path}[{index}]"), problems);
            }
        }
        _ => {}
    }
}

#[test]
fn valid_fixtures_pass_rust_validation_and_round_trip() {
    for path in json_files(&fixture_dir("valid")) {
        let name = fixture_name(&path);
        assert!(
            name.starts_with("request-") || name.starts_with("response-"),
            "valid fixture {name} is not classified as a request or response"
        );
        let document = read_json(&path);
        let envelope = parse_envelope(&name, &document)
            .unwrap_or_else(|error| panic!("rust must accept valid fixture {name}: {error}"));
        assert_round_trip(&envelope, &name);
    }
}

#[test]
fn invalid_fixtures_fail_closed_with_typed_errors() {
    for path in json_files(&fixture_dir("invalid")) {
        let name = fixture_name(&path);
        let expected = expected_error(&name);
        let document = read_json(&path);
        let error = match parse_envelope(&name, &document) {
            Ok(_) => panic!("rust must reject invalid fixture {name}"),
            Err(error) => error,
        };
        assert_eq!(
            error.kind(),
            expected,
            "wrong typed error for {name}: {error}"
        );
    }
}

#[test]
fn every_fixture_matches_schema_and_rust_expectations() {
    let request_schema = compiled_schema("request-v1.schema.json");
    let response_schema = compiled_schema("response-v1.schema.json");

    for category in ["valid", "invalid"] {
        let expected_accept = category == "valid";
        for path in json_files(&fixture_dir(category)) {
            let name = fixture_name(&path);
            let document = read_json(&path);
            let schema = if name.starts_with("request-") {
                &request_schema
            } else if name.starts_with("response-") {
                &response_schema
            } else {
                panic!("fixture {name} is not classified as a request or response");
            };
            let schema_accepts = schema.is_valid(&document);
            let rust_accepts = parse_envelope(&name, &document).is_ok();
            assert_eq!(
                schema_accepts, expected_accept,
                "schema expectation mismatch for {category}/{name}"
            );
            assert_eq!(
                rust_accepts, expected_accept,
                "Rust expectation mismatch for {category}/{name}"
            );
            assert_eq!(
                schema_accepts, rust_accepts,
                "schema/Rust disagreement for {category}/{name}"
            );
        }
    }
}

#[test]
fn invalid_expectations_match_fixture_inventory() {
    let inventory: BTreeSet<String> = json_files(&fixture_dir("invalid"))
        .iter()
        .map(|path| fixture_name(path))
        .collect();
    let pinned: BTreeSet<String> = INVALID_EXPECTATIONS
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect();
    assert_eq!(
        inventory, pinned,
        "invalid fixture inventory and pinned typed-error expectations must match exactly"
    );
}

#[test]
fn every_error_kind_is_pinned_by_a_fixture() {
    let samples = error_kind_samples();
    let mut seen = Vec::new();
    for (kind, error) in &samples {
        assert_eq!(error.kind(), *kind, "kind mapping drifted for {kind:?}");
        assert!(
            !seen.contains(kind),
            "duplicate ErrorKind in samples: {kind:?}"
        );
        seen.push(*kind);
    }
    for (kind, _) in &samples {
        let pinned = INVALID_EXPECTATIONS
            .iter()
            .any(|(_, expected)| expected == kind);
        assert!(
            pinned,
            "ErrorKind {kind:?} is not pinned by any invalid fixture"
        );
    }
}

#[test]
fn response_fixtures_have_complete_pairing() {
    let mut response_fixtures = BTreeSet::new();
    for kind in ["valid", "invalid"] {
        for path in json_files(&fixture_dir(kind)) {
            let name = fixture_name(&path);
            if name.starts_with("response-") {
                response_fixtures.insert(name);
            }
        }
    }
    let mut paired = BTreeSet::new();
    for (response, request) in RESPONSE_PAIRS {
        assert!(
            paired.insert((*response).to_owned()),
            "duplicate pairing entry for {response}"
        );
        assert!(
            request.starts_with("request-"),
            "pair target {request} is not a request fixture"
        );
        let request_path = fixture_dir("valid").join(request);
        assert!(
            request_path.is_file(),
            "paired request fixture {request} is missing"
        );
    }
    assert_eq!(
        response_fixtures, paired,
        "every response fixture must have exactly one pairing"
    );
}

#[test]
fn cross_envelope_semantics_fail_closed() {
    let score_request =
        Request::from_value(&read_json(&fixture_dir("valid").join("request-score.json")))
            .expect("score request fixture");

    let mut foreign_reference = read_json(&fixture_dir("valid").join("response-score.json"));
    foreign_reference["evidence_refs"][0]["ref"] =
        Value::String(format!("evidence://sha256/{}", "d4".repeat(32)));
    let response = Response::from_value(&foreign_reference).expect("well-shaped response");
    assert_eq!(
        response
            .validate_against(&score_request)
            .expect_err("foreign evidence must fail")
            .kind(),
        ErrorKind::InvalidEvidenceReference
    );

    let selection_request = Request::from_value(&read_json(
        &fixture_dir("valid").join("request-select-minimal.json"),
    ))
    .expect("selection request fixture");
    let mut foreign_choice = read_json(&fixture_dir("valid").join("response-choice.json"));
    foreign_choice["result"]["value"]["choices"][0] = Value::String("cand-gamma".to_owned());
    let response = Response::from_value(&foreign_choice).expect("well-shaped response");
    assert_eq!(
        response
            .validate_against(&selection_request)
            .expect_err("foreign choice must fail")
            .kind(),
        ErrorKind::InconsistentEnvelope
    );

    let mut unsupported_verdict = read_json(&fixture_dir("valid").join("response-verdict.json"));
    unsupported_verdict["evidence_refs"] = Value::Array(Vec::new());
    let response = Response::from_value(&unsupported_verdict).expect("well-shaped response");
    let verify_request = Request::from_value(&read_json(
        &fixture_dir("valid").join("request-verify.json"),
    ))
    .expect("verify request fixture");
    assert_eq!(
        response
            .validate_against(&verify_request)
            .expect_err("supported verdict without evidence must fail")
            .kind(),
        ErrorKind::InconsistentEnvelope
    );

    let mut late_response = read_json(&fixture_dir("valid").join("response-score.json"));
    late_response["latency"]["queue_ms"] = Value::from(26);
    let response = Response::from_value(&late_response).expect("well-shaped response");
    assert_eq!(
        response
            .validate_against(&score_request)
            .expect_err("queue deadline overrun must fail")
            .kind(),
        ErrorKind::InvalidDeadline
    );
}

#[test]
fn canonical_schemas_pin_draft_2020_12() {
    let files = json_files(&schema_dir());
    let names: BTreeSet<String> = files.iter().map(|path| fixture_name(path)).collect();
    assert_eq!(
        names,
        BTreeSet::from([
            "request-v1.schema.json".to_owned(),
            "response-v1.schema.json".to_owned()
        ]),
        "canonical protocol schema file set changed; update this pin deliberately"
    );
    for name in &names {
        let document = read_json(&schema_dir().join(name));
        assert_eq!(
            document["$schema"], "https://json-schema.org/draft/2020-12/schema",
            "{name} must pin JSON Schema Draft 2020-12"
        );
        assert!(
            document["$id"].is_string(),
            "{name} must declare a canonical $id"
        );
        assert!(
            jsonschema::draft202012::meta::is_valid(&document),
            "{name} must satisfy the Draft 2020-12 meta-schema"
        );
        let _ = compiled_schema(name);
    }
    let request = read_json(&schema_dir().join("request-v1.schema.json"));
    let response = read_json(&schema_dir().join("response-v1.schema.json"));
    assert_eq!(
        request["$defs"]["evidenceReference"], response["$defs"]["evidenceReference"],
        "duplicated self-contained evidence-reference definitions must stay identical"
    );
}

#[test]
fn accepted_contracts_and_fixtures_carry_no_forbidden_fields() {
    let mut problems = Vec::new();
    for path in json_files(&schema_dir()) {
        scan_keys(&read_json(&path), &fixture_name(&path), &mut problems);
    }
    for path in json_files(&fixture_dir("valid")) {
        scan_keys(&read_json(&path), &fixture_name(&path), &mut problems);
    }
    assert!(
        problems.is_empty(),
        "forbidden secret/hidden-reasoning/unrestricted-transcript fields present: {problems:?}"
    );
}
