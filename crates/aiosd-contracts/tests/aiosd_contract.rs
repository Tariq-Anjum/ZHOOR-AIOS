use std::path::PathBuf;

use jsonschema::Validator;
use serde_json::{Value, json};
use zhoor_aiosd_contracts::{
    AiosdRequest, ContractCheckError, ContractError, Frame, GrantVerificationFailure,
    PostconditionRequirement, PostconditionVerdict, PostconditionVerification, Receipt,
    ValidationContext, capabilities_describe_argument_sha256, check_grant_bindings,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str) -> Value {
    serde_json::from_slice(
        &std::fs::read(repo_root().join("fixtures/aiosd").join(name)).expect("fixture exists"),
    )
    .expect("fixture is JSON")
}

fn schema() -> Value {
    serde_json::from_slice(
        &std::fs::read(repo_root().join("schemas/aiosd/aiosd-v1.schema.json"))
            .expect("schema exists"),
    )
    .expect("schema is JSON")
}

fn validator(definition: &str) -> Validator {
    let mut document = schema();
    document["$ref"] = json!(format!("#/$defs/{definition}"));
    jsonschema::draft202012::new(&document).expect("schema compiles")
}

fn valid_request_value() -> Value {
    fixture("request-valid-v1.json")
}

fn valid_request() -> AiosdRequest {
    AiosdRequest::from_json_bytes(
        &serde_json::to_vec(&valid_request_value()).expect("serialize fixture"),
    )
    .expect("valid request")
}

fn transcript(value: &Value) -> Vec<u8> {
    AiosdRequest::from_json_bytes(&serde_json::to_vec(value).expect("serialize request"))
        .expect("valid request")
        .grant()
        .claims_transcript()
        .expect("claims transcript")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn positive_request_receipt_and_postcondition_fixtures_match_schema_and_parser() {
    let request_value = valid_request_value();
    assert!(validator("request").is_valid(&request_value));
    let request = AiosdRequest::from_json_bytes(
        &serde_json::to_vec(&request_value).expect("serialize request"),
    )
    .expect("valid request parses");

    let receipt_value = fixture("receipt-valid-v1.json");
    assert!(validator("receipt").is_valid(&receipt_value));
    let receipt =
        Receipt::from_json_bytes(&serde_json::to_vec(&receipt_value).expect("serialize receipt"))
            .expect("valid receipt parses");
    receipt
        .check_request_bindings(&request)
        .expect("receipt fields bind to request");

    let postcondition_value = fixture("postcondition-unknown-v1.json");
    assert!(validator("postconditionVerification").is_valid(&postcondition_value));
    let requirement_value = fixture("postcondition-required-v1.json");
    assert!(validator("postconditionRequirement").is_valid(&requirement_value));
    let requirement = PostconditionRequirement::from_json_bytes(
        &serde_json::to_vec(&requirement_value).expect("serialize requirement"),
    )
    .expect("valid requirement parses");
    let report = PostconditionVerification::from_json_bytes(
        &serde_json::to_vec(&postcondition_value).expect("serialize postcondition"),
    )
    .expect("unknown is a valid distinct verdict");
    assert_eq!(report.verdict(), PostconditionVerdict::Unknown);
    report
        .check_bindings(&requirement, &request, &receipt)
        .expect("postcondition linkage matches");
}

#[test]
fn request_hostile_fixtures_fail_schema_and_parser() {
    let mut cases = Vec::new();

    let mut value = valid_request_value();
    value["request_version"] = json!("2");
    cases.push(("unsupported version", value, false));

    let mut value = valid_request_value();
    value["unexpected"] = json!(true);
    cases.push(("unknown field", value, false));

    let mut value = valid_request_value();
    value["arguments"]["path"] = json!("../../etc/passwd");
    cases.push(("non-empty arguments", value, false));

    let mut value = valid_request_value();
    value["arguments"] = json!([]);
    cases.push(("arguments encoded as array", value, false));

    for scope in ["../../etc/passwd", "/etc/passwd", "symlink://outside/root"] {
        let mut value = valid_request_value();
        value["resource_scope"] = json!(scope);
        cases.push(("path-shaped scope", value, false));
    }

    let mut value = valid_request_value();
    value["evidence"] = json!([]);
    value["grant"]["evidence"] = json!([]);
    cases.push(("missing provenance", value, false));

    let mut value = valid_request_value();
    value["grant"]["authenticator"] = Value::Null;
    cases.push(("missing authenticator", value, false));

    let mut value = valid_request_value();
    value["grant"]["authenticator"]["proof"] = json!("not-lowercase-hex");
    cases.push(("malformed authenticator", value, false));

    let mut value = valid_request_value();
    value["grant"]["authenticator"]["format"] = json!("not supported yet");
    cases.push(("malformed authenticator format", value, false));

    let mut value = valid_request_value();
    value["grant"]["authenticator"]["proof"] = json!("00".repeat(4097));
    cases.push(("oversized proof representation", value, false));

    let mut value = valid_request_value();
    value["grant"]["capability_id"] = json!("filesystem.write");
    cases.push(("capability substitution", value, false));

    let mut value = valid_request_value();
    value["grant"]["argument_sha256"] =
        json!("sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
    cases.push(("argument tampering", value, false));

    let mut value = valid_request_value();
    value["grant"]["trace_id"] = json!("55555555-5555-4555-8555-555555555555");
    cases.push(("trace substitution", value, true));

    let mut value = valid_request_value();
    value["grant"]["decision_hash"] =
        json!("sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
    cases.push(("decision substitution", value, true));

    let mut value = valid_request_value();
    value["grant"]["evidence"][0]["source"] = json!("other source");
    cases.push(("evidence binding mismatch", value, true));

    let mut value = valid_request_value();
    value["grant"]["resource_scope"] = json!("other scope");
    cases.push(("scope substitution", value, false));

    let mut value = valid_request_value();
    value["grant"]["grant_id"] = json!("not-a-uuid");
    cases.push(("malformed id", value, false));

    let mut value = valid_request_value();
    value["grant"]["argument_sha256"] =
        json!("SHA256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a");
    cases.push(("hash grammar/case", value, false));

    let mut value = valid_request_value();
    let duplicate = value["evidence"][0].clone();
    value["evidence"].as_array_mut().unwrap().push(duplicate);
    value["grant"]["evidence"] = value["evidence"].clone();
    cases.push(("duplicate evidence identifier", value, false));

    let mut value = valid_request_value();
    value["evidence"][0]["source"] = json!("first source");
    let mut duplicate = value["evidence"][0].clone();
    duplicate["source"] = json!("second source");
    value["evidence"].as_array_mut().unwrap().push(duplicate);
    value["grant"]["evidence"] = value["evidence"].clone();
    cases.push(("duplicate reference with altered provenance", value, true));

    let mut value = valid_request_value();
    let item = value["evidence"][0].clone();
    value["evidence"] = json!(vec![item; 65]);
    value["grant"]["evidence"] = value["evidence"].clone();
    cases.push(("too many evidence items", value, false));

    for (label, value, schema_accepts) in cases {
        assert_eq!(
            validator("request").is_valid(&value),
            schema_accepts,
            "unexpected schema result for {label}"
        );
        let bytes = serde_json::to_vec(&value).expect("serialize hostile request");
        assert!(
            AiosdRequest::from_json_bytes(&bytes).is_err(),
            "parser accepted {label}"
        );
    }
}

#[test]
fn bounded_input_and_context_failures_are_explicit() {
    let request = valid_request();
    let mut context = ValidationContext {
        authenticated_peer_uid: 1000,
        trusted_now_unix_ms: 1000,
        grant_already_consumed: false,
    };
    check_grant_bindings(&request, context).expect("matching context");

    context.authenticated_peer_uid = 1001;
    assert_eq!(
        check_grant_bindings(&request, context),
        Err(ContractCheckError::CallerMismatch)
    );
    context.authenticated_peer_uid = 1000;
    context.grant_already_consumed = true;
    assert_eq!(
        check_grant_bindings(&request, context),
        Err(ContractCheckError::ReplayedOrConsumed)
    );
    context.grant_already_consumed = false;
    context.trusted_now_unix_ms = 2000;
    assert_eq!(
        check_grant_bindings(&request, context),
        Err(ContractCheckError::Expired)
    );

    context.trusted_now_unix_ms = 1000;
    let mut far_future = valid_request_value();
    far_future["grant"]["expires_at_unix_ms"] = json!(31_001);
    let far_future =
        AiosdRequest::from_json_bytes(&serde_json::to_vec(&far_future).expect("serialize request"))
            .expect("well-formed future expiry");
    assert_eq!(
        check_grant_bindings(&far_future, context),
        Err(ContractCheckError::ExpiryTooFarInFuture)
    );

    let too_large = vec![b' '; 65_537];
    assert_eq!(
        AiosdRequest::from_json_bytes(&too_large),
        Err(ContractError::InputTooLarge)
    );
    assert_eq!(
        capabilities_describe_argument_sha256().as_str(),
        "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
    );
}

#[test]
fn claims_transcript_binds_claims_but_not_proof_bytes() {
    let mut value = valid_request_value();
    let first =
        AiosdRequest::from_json_bytes(&serde_json::to_vec(&value).expect("serialize request"))
            .expect("valid request");
    let first_transcript = first.grant().claims_transcript().expect("transcript");

    value["grant"]["authenticator"]["proof"] = json!("ff");
    let changed_proof =
        AiosdRequest::from_json_bytes(&serde_json::to_vec(&value).expect("serialize request"))
            .expect("opaque proof is structurally valid");
    assert_eq!(
        first_transcript,
        changed_proof
            .grant()
            .claims_transcript()
            .expect("transcript")
    );

    value["grant"]["issuer_id"] = json!("different-issuer");
    let changed_claim =
        AiosdRequest::from_json_bytes(&serde_json::to_vec(&value).expect("serialize request"))
            .expect("issuer claim is structurally valid");
    assert_ne!(
        first_transcript,
        changed_claim
            .grant()
            .claims_transcript()
            .expect("transcript")
    );
}

#[test]
fn claims_transcript_matches_the_frozen_golden_vector() {
    let expected = include_str!("../../../fixtures/aiosd/grant-claims-transcript-v1.hex").trim();
    assert_eq!(
        hex(&valid_request()
            .grant()
            .claims_transcript()
            .expect("transcript")),
        expected
    );
}

#[test]
fn every_variable_frozen_claim_changes_the_transcript() {
    let base = valid_request_value();
    let original = transcript(&base);
    let mut variants = Vec::new();

    let mut value = base.clone();
    value["grant"]["grant_id"] = json!("55555555-5555-4555-8555-555555555555");
    variants.push(("grant_id", value));

    let mut value = base.clone();
    value["grant"]["issuer_id"] = json!("other-issuer");
    variants.push(("issuer_id", value));

    let mut value = base.clone();
    value["grant"]["client_identity"]["uid"] = json!(1001);
    variants.push(("client UID", value));

    let mut value = base.clone();
    value["grant"]["policy_version"] = json!("v2");
    variants.push(("policy_version", value));

    let mut value = base.clone();
    value["grant"]["expires_at_unix_ms"] = json!(2001);
    variants.push(("expiry", value));

    let mut value = base.clone();
    value["trace_id"] = json!("55555555-5555-4555-8555-555555555555");
    value["grant"]["trace_id"] = value["trace_id"].clone();
    variants.push(("trace_id", value));

    let mut value = base.clone();
    value["decision_hash"] =
        json!("sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
    value["grant"]["decision_hash"] = value["decision_hash"].clone();
    variants.push(("decision_hash", value));

    let mut value = base.clone();
    value["evidence"][0]["source"] = json!("different provenance");
    value["grant"]["evidence"] = value["evidence"].clone();
    variants.push(("evidence binding", value));

    let mut value = base.clone();
    value["grant"]["authenticator"]["format"] = json!("opaque-v2");
    variants.push(("authenticator format", value));

    for (claim, value) in variants {
        assert_ne!(original, transcript(&value), "transcript omitted {claim}");
    }

    for (claim, path, replacement) in [
        ("grant version", "grant_version", "2"),
        (
            "capability substitution",
            "capability_id",
            "filesystem.write",
        ),
        (
            "argument substitution",
            "argument_sha256",
            "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        ),
        (
            "resource-scope substitution",
            "resource_scope",
            "filesystem.write",
        ),
    ] {
        let mut value = base.clone();
        value["grant"][path] = json!(replacement);
        assert!(
            AiosdRequest::from_json_bytes(&serde_json::to_vec(&value).expect("serialize")).is_err(),
            "accepted substituted {claim}"
        );
    }
}

#[test]
fn evidence_order_does_not_change_the_claims_transcript() {
    let mut first = valid_request_value();
    let mut second_evidence = first["evidence"][0].clone();
    second_evidence["ref"] = json!(format!("evidence://sha256/{}", "d".repeat(64)));
    second_evidence["source"] = json!("second evidence source");
    first["evidence"]
        .as_array_mut()
        .unwrap()
        .push(second_evidence);
    first["grant"]["evidence"] = first["evidence"].clone();

    let mut reversed = first.clone();
    reversed["evidence"].as_array_mut().unwrap().reverse();
    reversed["grant"]["evidence"] = reversed["evidence"].clone();
    assert_eq!(transcript(&first), transcript(&reversed));
}

#[test]
fn raw_duplicate_json_members_reject_before_generic_value_collapse() {
    let bytes = serde_json::to_string(&valid_request_value()).expect("serialize request");
    let duplicated = bytes.replace(
        "\"issuer_id\":\"local-policy\"",
        "\"issuer_id\":\"local-policy\",\"issuer_id\":\"other-issuer\"",
    );
    assert_ne!(bytes, duplicated, "fixture needle must match");
    assert!(AiosdRequest::from_json_bytes(duplicated.as_bytes()).is_err());

    let duplicated_expiry = bytes.replace(
        "\"expires_at_unix_ms\":2000",
        "\"expires_at_unix_ms\":2000,\"expires_at_unix_ms\":4000",
    );
    assert_ne!(bytes, duplicated_expiry, "expiry needle must match");
    assert!(AiosdRequest::from_json_bytes(duplicated_expiry.as_bytes()).is_err());
}

#[test]
fn opaque_proof_format_and_verifier_failures_have_no_success_claim() {
    let mut value = valid_request_value();
    value["grant"]["authenticator"]["format"] = json!("future-proof-v9");
    let request =
        AiosdRequest::from_json_bytes(&serde_json::to_vec(&value).expect("serialize request"))
            .expect("SEC checks syntax without deciding supported formats");
    assert_eq!(request.grant().authenticator().format(), "future-proof-v9");

    let schema_value = schema();
    let failures = [
        "missing_authenticator",
        "malformed_authenticator",
        "unsupported_authenticator_format",
        "invalid_authenticator",
        "untrusted_issuer",
        "unsupported_version",
        "caller_mismatch",
        "capability_mismatch",
        "argument_binding_mismatch",
        "resource_scope_mismatch",
        "policy_version_mismatch",
        "expired",
        "replayed_or_consumed",
        "trace_mismatch",
        "decision_hash_mismatch",
        "evidence_binding_mismatch",
    ];
    let failure_schema = &schema_value["$defs"]["grantVerificationFailure"];
    for failure in failures {
        assert!(
            failure_schema["enum"]
                .as_array()
                .unwrap()
                .contains(&json!(failure)),
            "schema omitted verifier failure {failure}"
        );
    }
    assert_eq!(
        serde_json::to_value(GrantVerificationFailure::UnsupportedAuthenticatorFormat)
            .expect("serialize failure"),
        json!("unsupported_authenticator_format")
    );
    assert!(serde_json::from_value::<GrantVerificationFailure>(json!("verified")).is_err());
}

#[test]
fn receipt_and_postcondition_linkage_and_verdicts_stay_distinct() {
    let request = valid_request();
    let receipt_value = fixture("receipt-valid-v1.json");
    let receipt =
        Receipt::from_json_bytes(&serde_json::to_vec(&receipt_value).expect("serialize receipt"))
            .expect("valid receipt");
    let mut wrong_receipt = receipt_value;
    wrong_receipt["grant_id"] = json!("55555555-5555-4555-8555-555555555555");
    let wrong_receipt =
        Receipt::from_json_bytes(&serde_json::to_vec(&wrong_receipt).expect("serialize receipt"))
            .expect("well-formed mismatched receipt");
    assert_eq!(
        wrong_receipt.check_request_bindings(&request),
        Err(ContractError::ReceiptMismatch)
    );

    let postcondition = fixture("postcondition-unknown-v1.json");
    let requirement = PostconditionRequirement::from_json_bytes(
        &serde_json::to_vec(&fixture("postcondition-required-v1.json"))
            .expect("serialize requirement"),
    )
    .expect("valid requirement");
    let report = PostconditionVerification::from_json_bytes(
        &serde_json::to_vec(&postcondition).expect("serialize report"),
    )
    .expect("unknown verdict parses");
    assert_eq!(report.verdict(), PostconditionVerdict::Unknown);
    let mut mismatched = postcondition;
    mismatched["receipt_id"] = json!("55555555-5555-4555-8555-555555555555");
    let mismatched = PostconditionVerification::from_json_bytes(
        &serde_json::to_vec(&mismatched).expect("serialize report"),
    )
    .expect("well-formed mismatched report");
    assert_eq!(
        mismatched.check_bindings(&requirement, &request, &receipt),
        Err(ContractError::PostconditionBindingMismatch)
    );

    let mut failed = fixture("postcondition-unknown-v1.json");
    failed["verdict"] = json!("failed");
    assert_eq!(
        PostconditionVerification::from_json_bytes(
            &serde_json::to_vec(&failed).expect("serialize failed report")
        )
        .expect("failed is a distinct valid verdict")
        .verdict(),
        PostconditionVerdict::Failed
    );
    assert_eq!(
        PostconditionVerdict::Unknown,
        PostconditionVerification::from_json_bytes(
            &serde_json::to_vec(&fixture("postcondition-unknown-v1.json"))
                .expect("serialize unknown report")
        )
        .expect("unknown report")
        .verdict()
    );
}

#[test]
fn framing_requires_exact_complete_bounded_message_and_matching_correlation() {
    let request_value = valid_request_value();
    let mut frame = json!({
        "frame_version": "1",
        "message_kind": "request",
        "correlation_id": request_value["request_id"],
        "request": request_value
    });
    assert!(validator("frame").is_valid(&frame));
    let payload = serde_json::to_vec(&frame).expect("serialize frame");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(matches!(
        Frame::from_complete_bytes(&bytes),
        Ok(Frame::Request { .. })
    ));

    frame["response"] = Value::Null;
    assert!(!validator("frame").is_valid(&frame));
    let payload = serde_json::to_vec(&frame).expect("serialize explicit null body");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(Frame::from_complete_bytes(&bytes).is_err());
    frame.as_object_mut().unwrap().remove("response");

    bytes.push(0);
    assert_eq!(
        Frame::from_complete_bytes(&bytes),
        Err(ContractError::IncompleteOrTrailingFrame)
    );

    frame["correlation_id"] = json!("55555555-5555-4555-8555-555555555555");
    let payload = serde_json::to_vec(&frame).expect("serialize mismatched frame");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(validator("frame").is_valid(&frame));
    assert_eq!(
        Frame::from_complete_bytes(&bytes),
        Err(ContractError::CorrelationMismatch)
    );

    frame["frame_version"] = json!("2");
    frame["correlation_id"] = frame["request"]["request_id"].clone();
    let payload = serde_json::to_vec(&frame).expect("serialize unknown version frame");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(!validator("frame").is_valid(&frame));
    assert_eq!(
        Frame::from_complete_bytes(&bytes),
        Err(ContractError::UnsupportedVersion)
    );

    frame["frame_version"] = json!("1");
    frame["message_kind"] = json!("unknown");
    let payload = serde_json::to_vec(&frame).expect("serialize unknown kind frame");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(!validator("frame").is_valid(&frame));
    assert!(Frame::from_complete_bytes(&bytes).is_err());

    let oversized = (65_537_u32).to_be_bytes().to_vec();
    assert_eq!(
        Frame::from_complete_bytes(&oversized),
        Err(ContractError::FrameTooLarge)
    );
}

#[test]
fn response_frame_descriptor_count_matches_closed_outcome() {
    let receipt = fixture("receipt-valid-v1.json");
    let descriptor = json!({
        "descriptor_version": "1",
        "capability_id": "capabilities.describe",
        "description": "Describe registered capability metadata.",
        "mutating": false,
        "argument_contract": "empty_object_v1",
        "resource_scope": "registered_capability_descriptor_metadata"
    });
    let mut response_frame = json!({
        "frame_version": "1",
        "message_kind": "response",
        "correlation_id": receipt["request_id"],
        "response": {
            "outcome": "succeeded",
            "descriptors": [descriptor],
            "receipt": receipt
        }
    });
    assert!(validator("frame").is_valid(&response_frame));
    let payload = serde_json::to_vec(&response_frame).expect("serialize response frame");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(matches!(
        Frame::from_complete_bytes(&bytes),
        Ok(Frame::Response { .. })
    ));

    response_frame["response"]["descriptors"][0]["description"] = json!("line\nbreak");
    assert!(!validator("frame").is_valid(&response_frame));
    let payload =
        serde_json::to_vec(&response_frame).expect("serialize control-character descriptor");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(Frame::from_complete_bytes(&bytes).is_err());

    response_frame["response"]["descriptors"][0]["description"] =
        json!("Describe registered capability metadata.");
    response_frame["response"]["descriptors"] = json!([]);
    assert!(!validator("frame").is_valid(&response_frame));
    let payload = serde_json::to_vec(&response_frame).expect("serialize missing descriptor");
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(&payload);
    assert!(Frame::from_complete_bytes(&bytes).is_err());
}
