//! Contract tests for sanitized trace serialization and replay.

use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::Validator;
use serde_json::Value;
use zhoor_protocol::{Request, Response};
use zhoor_trace::{DecisionTrace, TraceContext, TraceError};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON fixture"))
        .unwrap_or_else(|error| panic!("valid JSON fixture {}: {error}", path.display()))
}

fn schema() -> Validator {
    let document = read_json(&repo_root().join("schemas/trace/trace-v1.schema.json"));
    jsonschema::draft202012::new(&document).expect("trace schema compiles")
}

fn context() -> TraceContext {
    TraceContext {
        worker_version: "risk-judge-v1".to_owned(),
        model_version: Some("model-v2".to_owned()),
        registry_version: Some("registry-v3".to_owned()),
        recorded_at_unix_ms: 1_759_000_000_000,
        receipt_ref: Some(format!("receipt://sha256/{}", "d".repeat(64))),
    }
}

#[test]
fn committed_fixtures_match_schema_and_rust_contract() {
    let validator = schema();
    for (kind, expected_valid) in [("valid", true), ("invalid", false)] {
        let directory = repo_root().join("fixtures/trace").join(kind);
        for entry in fs::read_dir(directory).expect("fixture directory") {
            let path = entry.expect("fixture entry").path();
            let instance = read_json(&path);
            assert_eq!(
                validator.is_valid(&instance),
                expected_valid,
                "schema expectation for {}",
                path.display()
            );
            assert_eq!(
                DecisionTrace::from_json_str(&fs::read_to_string(&path).expect("fixture text"))
                    .is_ok(),
                expected_valid,
                "Rust expectation for {}",
                path.display()
            );
        }
    }
}

#[test]
fn creation_serialization_sanitization_and_replay_round_trip() {
    let root = repo_root();
    let request_json = fs::read_to_string(root.join("fixtures/trace/replay/request-score.json"))
        .expect("replay request");
    let response_json =
        fs::read_to_string(root.join("fixtures/protocol/valid/response-score.json"))
            .expect("response fixture");
    let request = Request::from_json_str(&request_json).expect("valid request");
    let response = Response::from_json_str(&response_json).expect("valid response");
    let trace = DecisionTrace::create(&request, &response, context()).expect("create trace");
    let serialized = trace.to_json_string().expect("serialize trace");
    let instance: Value = serde_json::from_str(&serialized).expect("trace JSON");

    assert!(schema().is_valid(&instance));
    let reordered =
        serde_json::to_string(&serde_json::from_str::<Value>(&request_json).expect("request JSON"))
            .expect("normalized JSON");
    let reordered_request = Request::from_json_str(&reordered).expect("reordered request");
    assert_eq!(
        trace.input_hash,
        DecisionTrace::create(&reordered_request, &response, context())
            .expect("same normalized request")
            .input_hash
    );
    assert!(!serialized.contains("Assess the operational risk"));
    assert!(!serialized.contains("tool:disk-usage"));
    assert!(!serialized.contains("registry:skills"));
    assert!(!serialized.contains("score within the guard band"));
    assert!(!serialized.contains("\"score\":0.42"));
    assert!(!serialized.contains("calibration-risk-judge-v1"));
    assert!(serialized.contains(&trace.input_hash));
    assert!(serialized.contains(&trace.response_hash));
    assert_eq!(trace.latency.queue_ms, 4);
    assert_eq!(trace.latency.load_ms, 1);
    assert_eq!(trace.latency.inference_ms, 22);
    assert_eq!(trace.evidence_refs.len(), 1);
    assert!(trace.receipt_ref.is_some());

    let parsed = DecisionTrace::from_json_str(&serialized).expect("parse trace");
    let replayed = parsed
        .reconstruct_replay_input(&request_json)
        .expect("reconstruct normalized request");
    assert_eq!(replayed.request_id, request.request_id);
    assert_eq!(replayed.operation, request.operation);
    assert_eq!(
        parsed.input_hash,
        DecisionTrace::create(&request, &response, context())
            .expect("repeat trace")
            .input_hash
    );

    let altered = request_json.replace("operational risk", "changed risk");
    assert_eq!(
        parsed.reconstruct_replay_input(&altered),
        Err(TraceError::ReplayMismatch)
    );
}

#[test]
fn creation_rejects_mismatched_response_linkage() {
    let root = repo_root();
    let request_json = fs::read_to_string(root.join("fixtures/trace/replay/request-score.json"))
        .expect("replay request");
    let response_json =
        fs::read_to_string(root.join("fixtures/protocol/valid/response-score.json"))
            .expect("response fixture");
    let request = Request::from_json_str(&request_json).expect("valid request");
    let altered_response = response_json.replace(
        "22222222-2222-4222-8222-222222222222",
        "33333333-3333-4333-8333-333333333333",
    );
    let response = Response::from_json_str(&altered_response).expect("valid response");
    assert_eq!(
        DecisionTrace::create(&request, &response, context()),
        Err(TraceError::InvalidProtocolEnvelope)
    );
}

#[test]
fn denied_trace_keeps_only_typed_error_code() {
    let root = repo_root();
    let request_json = fs::read_to_string(root.join("fixtures/trace/replay/request-score.json"))
        .expect("replay request");
    let response_json =
        fs::read_to_string(root.join("fixtures/protocol/valid/response-denied.json"))
            .expect("denied response fixture");
    let request = Request::from_json_str(&request_json).expect("valid request");
    let response = Response::from_json_str(&response_json).expect("valid denied response");
    let mut context = context();
    context.model_version = None;
    context.registry_version = None;
    let trace = DecisionTrace::create(&request, &response, context).expect("denied trace");
    let serialized = trace.to_json_string().expect("serialize denied trace");

    assert_eq!(
        trace.error_code,
        Some(zhoor_trace::TraceErrorCode::DeniedByPolicy)
    );
    assert_eq!(trace.result_kind, None);
    assert!(!serialized.contains("no eligible evidence tier"));
    assert!(schema().is_valid(&serde_json::from_str(&serialized).expect("trace JSON")));
}

#[test]
fn unsafe_labels_and_receipt_references_are_rejected() {
    let root = repo_root();
    let request_json = fs::read_to_string(root.join("fixtures/trace/replay/request-score.json"))
        .expect("replay request");
    let response_json =
        fs::read_to_string(root.join("fixtures/protocol/valid/response-score.json"))
            .expect("response fixture");
    let request = Request::from_json_str(&request_json).expect("valid request");
    let response = Response::from_json_str(&response_json).expect("valid response");

    let mut unsafe_context = context();
    unsafe_context.worker_version = "worker version with spaces".to_owned();
    assert_eq!(
        DecisionTrace::create(&request, &response, unsafe_context),
        Err(TraceError::InvalidContext)
    );

    let mut unsafe_receipt = context();
    unsafe_receipt.receipt_ref = Some("secret-looking-untyped-receipt".to_owned());
    assert_eq!(
        DecisionTrace::create(&request, &response, unsafe_receipt),
        Err(TraceError::InvalidContext)
    );
}

#[test]
fn confidence_decimal_bounds_match_schema_without_float_rounding() {
    let root = repo_root();
    let fixture =
        fs::read_to_string(root.join("fixtures/trace/valid/trace-v1.json")).expect("trace fixture");
    let schema_document = read_json(&root.join("schemas/trace/trace-v1.schema.json"));
    assert_eq!(
        schema_document["$defs"]["confidence"]["properties"]["raw"]["maximum"],
        1
    );
    assert_eq!(
        schema_document["$defs"]["confidence"]["properties"]["calibrated"]["maximum"],
        1
    );
    let outside_upper_bound = fixture.replace("\"raw\": 0.66", "\"raw\": 1.00000000000000001");
    assert_eq!(
        DecisionTrace::from_json_str(&outside_upper_bound),
        Err(TraceError::InvalidTrace)
    );

    let outside_lower_bound = fixture.replace("\"raw\": 0.66", "\"raw\": -0.00000000000000001");
    assert_eq!(
        DecisionTrace::from_json_str(&outside_lower_bound),
        Err(TraceError::InvalidTrace)
    );

    let calibrated_outside_upper_bound = fixture.replace(
        "\"calibrated\": 0.61",
        "\"calibrated\": 1.00000000000000001",
    );
    assert_eq!(
        DecisionTrace::from_json_str(&calibrated_outside_upper_bound),
        Err(TraceError::InvalidTrace)
    );
}

#[test]
fn timestamp_schema_bound_matches_rust_u64() {
    let root = repo_root();
    let fixture =
        fs::read_to_string(root.join("fixtures/trace/valid/trace-v1.json")).expect("trace fixture");
    let at_maximum = fixture.replace(
        "\"recorded_at_unix_ms\": 1759000000000",
        "\"recorded_at_unix_ms\": 18446744073709551615",
    );
    let instance = read_json_value(&at_maximum);
    assert!(schema().is_valid(&instance));
    assert!(DecisionTrace::from_json_str(&at_maximum).is_ok());

    let above_maximum = fixture.replace(
        "\"recorded_at_unix_ms\": 1759000000000",
        "\"recorded_at_unix_ms\": 18446744073709551616",
    );
    let instance = read_json_value(&above_maximum);
    assert!(!schema().is_valid(&instance));
    assert_eq!(
        DecisionTrace::from_json_str(&above_maximum),
        Err(TraceError::InvalidTrace)
    );
}

fn read_json_value(text: &str) -> Value {
    serde_json::from_str(text).expect("valid JSON instance")
}
