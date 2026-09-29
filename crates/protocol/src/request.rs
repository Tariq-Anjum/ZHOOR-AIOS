//! Version 1 request envelope: parsing, typed model, and deterministic
//! semantic validation.

use serde::Serialize;
use serde_json::Value;

use crate::error::ProtocolError;
use crate::evidence::EvidenceRef;
use crate::util::{array_field, is_uuid, object_fields, str_field, uint_field};

/// Maximum goal length in characters.
pub const MAX_GOAL_CHARS: usize = 4096;
/// Maximum number of evidence references.
pub const MAX_EVIDENCE_REFS: usize = 64;
/// Maximum number of candidates.
pub const MAX_CANDIDATES: usize = 64;
/// Maximum candidate identifier length in characters.
pub const MAX_CANDIDATE_CHARS: usize = 256;
/// Maximum `max_choices`.
pub const MAX_CHOICES: u64 = 64;
/// Inclusive upper bound for `input_token_budget`.
pub const MAX_INPUT_TOKEN_BUDGET: u64 = 100_000;
/// Inclusive upper bound for `queue_ms`.
pub const MAX_QUEUE_MS: u64 = 60_000;
/// Inclusive upper bound for `execution_ms`.
pub const MAX_EXECUTION_MS: u64 = 600_000;
/// Maximum identifier length accepted before the strict UUID check.
const MAX_ID_CHARS: usize = 64;
/// Divisor for the deterministic serialized-character-to-token estimate.
const CHARS_PER_ESTIMATED_TOKEN: u64 = 4;

const REQUEST_FIELDS: &[&str] = &[
    "protocol_version",
    "request_id",
    "operation",
    "worker_kind",
    "goal",
    "evidence",
    "candidates",
    "constraints",
    "deadline",
    "policy_ref",
    "provenance",
];

const CONSTRAINT_FIELDS: &[&str] = &["max_choices", "input_token_budget"];
const DEADLINE_FIELDS: &[&str] = &["queue_ms", "execution_ms"];
const PROVENANCE_FIELDS: &[&str] = &["client", "session_ref"];

/// Bounded operation kind. Each operation pins a deterministic semantic
/// contract; unknown operations fail closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    /// Select one or more identifiers from the supplied candidates.
    Select,
    /// Produce a bounded numeric score with a defined scale and threshold.
    Score,
    /// Extract schema-bounded fields, each linked to source evidence.
    Extract,
    /// Return `supported`, `unsupported`, or `insufficient` with evidence
    /// references.
    Verify,
}

impl Operation {
    /// Parses a wire operation string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<Operation> {
        match value {
            "select" => Some(Operation::Select),
            "score" => Some(Operation::Score),
            "extract" => Some(Operation::Extract),
            "verify" => Some(Operation::Verify),
            _ => None,
        }
    }
}

/// Bounded worker kind. New workers require a protocol version change; the
/// open set is intentionally not extendable within v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerKind {
    /// Skill selection worker.
    SkillRouter,
    /// Risk assessment worker.
    RiskJudge,
}

impl WorkerKind {
    /// Parses a wire worker kind string, failing closed on unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Option<WorkerKind> {
        match value {
            "skill_router" => Some(WorkerKind::SkillRouter),
            "risk_judge" => Some(WorkerKind::RiskJudge),
            _ => None,
        }
    }
}

/// Supported protocol version. Unknown versions fail closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ProtocolVersion {
    /// Version 1.
    #[serde(rename = "1")]
    V1,
}

/// Bounded input constraints for a request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Constraints {
    /// Maximum number of choices a `select` result may contain.
    pub max_choices: u64,
    /// Declared input token budget for the envelope.
    pub input_token_budget: u64,
}

/// Bounded queue and execution deadline for a request, in milliseconds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Deadline {
    /// Maximum queue wait in milliseconds.
    pub queue_ms: u64,
    /// Maximum execution time in milliseconds.
    pub execution_ms: u64,
}

/// Bounded provenance for a request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Provenance {
    /// Client label.
    pub client: String,
    /// Session reference label.
    pub session_ref: String,
}

/// Version 1 request envelope. A model may propose, classify, extract, or
/// score through this envelope; nothing in it authorizes execution.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Request {
    /// Protocol version.
    pub protocol_version: ProtocolVersion,
    /// Canonical lowercase request identifier.
    pub request_id: String,
    /// Operation kind.
    pub operation: Operation,
    /// Worker kind.
    pub worker_kind: WorkerKind,
    /// Bounded goal text.
    pub goal: String,
    /// Evidence references supplied with the request.
    pub evidence: Vec<EvidenceRef>,
    /// Candidate identifiers relevant to the request.
    pub candidates: Vec<String>,
    /// Input constraints.
    pub constraints: Constraints,
    /// Queue and execution deadlines.
    pub deadline: Deadline,
    /// Policy version reference.
    pub policy_ref: String,
    /// Provenance labels.
    pub provenance: Provenance,
}

impl Request {
    /// Parses and validates a request envelope from JSON text, failing
    /// closed with typed errors.
    pub fn from_json_str(text: &str) -> Result<Request, ProtocolError> {
        let value: Value =
            serde_json::from_str(text).map_err(|_| ProtocolError::MalformedJson {
                reason: "invalid JSON text",
            })?;
        Request::from_value(&value)
    }

    /// Parses and validates a request envelope from a JSON value, failing
    /// closed with typed errors.
    pub fn from_value(value: &Value) -> Result<Request, ProtocolError> {
        let object = object_fields(value, REQUEST_FIELDS, REQUEST_FIELDS)?;

        let version = str_field(object, "protocol_version", 1, 8)?;
        if version != "1" {
            return Err(ProtocolError::UnsupportedVersion { found: version });
        }

        let request_id = str_field(object, "request_id", 1, MAX_ID_CHARS)?;
        if !is_uuid(&request_id) {
            return Err(ProtocolError::InvalidId {
                field: "request_id",
                found: request_id,
            });
        }

        let operation_raw = str_field(object, "operation", 1, 32)?;
        let operation =
            Operation::parse(&operation_raw).ok_or_else(|| ProtocolError::UnknownOperation {
                found: operation_raw.clone(),
            })?;

        let worker_raw = str_field(object, "worker_kind", 1, 32)?;
        let worker_kind =
            WorkerKind::parse(&worker_raw).ok_or_else(|| ProtocolError::UnknownWorkerKind {
                found: worker_raw.clone(),
            })?;

        let goal = str_field(object, "goal", 1, MAX_GOAL_CHARS)?;

        let evidence_items = array_field(object, "evidence")?;
        if evidence_items.len() > MAX_EVIDENCE_REFS {
            return Err(ProtocolError::MalformedJson {
                reason: "too many evidence references",
            });
        }
        let mut evidence = Vec::with_capacity(evidence_items.len());
        for item in evidence_items {
            evidence.push(EvidenceRef::from_value(item)?);
        }

        let candidate_items = array_field(object, "candidates")?;
        if candidate_items.len() > MAX_CANDIDATES {
            return Err(ProtocolError::MalformedJson {
                reason: "too many candidates",
            });
        }
        let mut candidates = Vec::with_capacity(candidate_items.len());
        for item in candidate_items {
            let text = item.as_str().ok_or(ProtocolError::MalformedJson {
                reason: "candidate must be a JSON string",
            })?;
            let len = text.chars().count();
            if !(1..=MAX_CANDIDATE_CHARS).contains(&len) {
                return Err(ProtocolError::MalformedJson {
                    reason: "candidate length out of range",
                });
            }
            candidates.push(text.to_owned());
        }

        let constraints_value = object
            .get("constraints")
            .ok_or(ProtocolError::MissingField {
                field: "constraints",
            })?;
        let constraints_object =
            object_fields(constraints_value, CONSTRAINT_FIELDS, CONSTRAINT_FIELDS)?;
        let constraints = Constraints {
            max_choices: uint_field(constraints_object, "max_choices")?,
            input_token_budget: uint_field(constraints_object, "input_token_budget")?,
        };

        let deadline_value = object
            .get("deadline")
            .ok_or(ProtocolError::MissingField { field: "deadline" })?;
        let deadline_object = object_fields(deadline_value, DEADLINE_FIELDS, DEADLINE_FIELDS)?;
        let deadline = Deadline {
            queue_ms: uint_field(deadline_object, "queue_ms")?,
            execution_ms: uint_field(deadline_object, "execution_ms")?,
        };

        let policy_ref = str_field(object, "policy_ref", 1, 256)?;

        let provenance_value = object
            .get("provenance")
            .ok_or(ProtocolError::MissingField {
                field: "provenance",
            })?;
        let provenance_object =
            object_fields(provenance_value, PROVENANCE_FIELDS, PROVENANCE_FIELDS)?;
        let provenance = Provenance {
            client: str_field(provenance_object, "client", 1, 256)?,
            session_ref: str_field(provenance_object, "session_ref", 1, 256)?,
        };

        let request = Request {
            protocol_version: ProtocolVersion::V1,
            request_id,
            operation,
            worker_kind,
            goal,
            evidence,
            candidates,
            constraints,
            deadline,
            policy_ref,
            provenance,
        };
        request.validate_semantics()?;
        Ok(request)
    }

    /// Deterministic semantic validation beyond JSON shape: budget and
    /// deadline bounds, duplicate detection, operation-specific consistency,
    /// and the input token budget check. Confidence never appears here and
    /// nothing here authorizes execution.
    pub fn validate_semantics(&self) -> Result<(), ProtocolError> {
        if self.constraints.input_token_budget == 0 {
            return Err(ProtocolError::InvalidBudget {
                reason: "input_token_budget must be positive",
            });
        }
        if self.constraints.input_token_budget > MAX_INPUT_TOKEN_BUDGET {
            return Err(ProtocolError::OverBudget {
                reason: format!(
                    "requested input_token_budget ({}) exceeds the v1 maximum ({MAX_INPUT_TOKEN_BUDGET})",
                    self.constraints.input_token_budget
                ),
            });
        }
        if self.constraints.max_choices > MAX_CHOICES {
            return Err(ProtocolError::InvalidBudget {
                reason: "max_choices out of bounds",
            });
        }
        if self.deadline.execution_ms == 0 || self.deadline.execution_ms > MAX_EXECUTION_MS {
            return Err(ProtocolError::InvalidDeadline {
                reason: "execution_ms out of bounds",
            });
        }
        if self.deadline.queue_ms > MAX_QUEUE_MS {
            return Err(ProtocolError::InvalidDeadline {
                reason: "queue_ms out of bounds",
            });
        }

        let mut seen_refs = Vec::new();
        for item in &self.evidence {
            if seen_refs.contains(&item.reference.as_str()) {
                return Err(ProtocolError::InvalidEvidenceReference {
                    reason: "duplicate evidence reference",
                });
            }
            seen_refs.push(item.reference.as_str());
        }

        let mut seen_candidates = Vec::new();
        for candidate in &self.candidates {
            if seen_candidates.contains(&candidate.as_str()) {
                return Err(ProtocolError::InconsistentEnvelope {
                    reason: "duplicate candidate identifier".to_owned(),
                });
            }
            seen_candidates.push(candidate.as_str());
        }

        match self.operation {
            Operation::Select => {
                if self.candidates.is_empty() {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "select requires at least one candidate".to_owned(),
                    });
                }
                if self.constraints.max_choices == 0 {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "select requires max_choices of at least one".to_owned(),
                    });
                }
            }
            Operation::Extract | Operation::Verify => {
                if self.evidence.is_empty() {
                    return Err(ProtocolError::InconsistentEnvelope {
                        reason: "extract and verify require at least one evidence reference"
                            .to_owned(),
                    });
                }
            }
            Operation::Score => {}
        }

        let estimated = self.estimated_input_tokens();
        if estimated > self.constraints.input_token_budget {
            return Err(ProtocolError::OverBudget {
                reason: format!(
                    "estimated input tokens ({estimated}) exceed the declared budget ({})",
                    self.constraints.input_token_budget
                ),
            });
        }
        Ok(())
    }

    /// Deterministic estimate of the serialized request envelope's input
    /// load: count all canonical JSON characters except the declared budget
    /// field itself, divide by four, and round up. Serialization failure
    /// fails closed as an unbounded estimate. The runtime enforces the
    /// worker's input budget before a backend sees data.
    #[must_use]
    pub fn estimated_input_tokens(&self) -> u64 {
        let Ok(mut wire_value) = serde_json::to_value(self) else {
            return u64::MAX;
        };
        if let Some(constraints) = wire_value
            .get_mut("constraints")
            .and_then(Value::as_object_mut)
        {
            constraints.remove("input_token_budget");
        }
        let Ok(serialized) = serde_json::to_string(&wire_value) else {
            return u64::MAX;
        };
        (serialized.chars().count() as u64).div_ceil(CHARS_PER_ESTIMATED_TOKEN)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Request;

    const HASH: &str = "e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0";

    fn boundary_request_json(input_token_budget: u64) -> String {
        json!({
            "protocol_version": "1",
            "request_id": "55555555-5555-4555-8555-555555555555",
            "operation": "score",
            "worker_kind": "risk_judge",
            "goal": "Boundary budget probe for protocol v1.",
            "evidence": [
                {
                    "ref": format!("evidence://sha256/{HASH}"),
                    "kind": "user_input",
                    "trust_tier": "untrusted",
                    "source": "user prompt"
                }
            ],
            "candidates": [],
            "constraints": { "max_choices": 1, "input_token_budget": input_token_budget },
            "deadline": { "queue_ms": 25, "execution_ms": 100 },
            "policy_ref": "risk-policy-v1",
            "provenance": { "client": "zhoor-cli", "session_ref": "session-0006" }
        })
        .to_string()
    }

    #[test]
    fn boundary_budget_exactly_meets_estimate() {
        let sizing_request =
            Request::from_json_str(&boundary_request_json(1_000)).expect("sizing request");
        let exact_budget = sizing_request.estimated_input_tokens();
        let request = Request::from_json_str(&boundary_request_json(exact_budget))
            .expect("request at exact estimated budget");
        assert_eq!(request.estimated_input_tokens(), exact_budget);
        assert_eq!(request.constraints.input_token_budget, exact_budget);
    }

    #[test]
    fn select_choice_limit_may_exceed_available_candidate_count() {
        let mut value: serde_json::Value =
            serde_json::from_str(&boundary_request_json(1_000)).expect("json");
        value["operation"] = json!("select");
        value["candidates"] = json!(["cand-alpha"]);
        value["constraints"]["max_choices"] = json!(2);
        let request = Request::from_value(&value).expect("choice limit is an upper bound");
        assert_eq!(request.constraints.max_choices, 2);
        assert_eq!(request.candidates.len(), 1);
    }

    #[test]
    fn derived_input_over_budget_fails_closed() {
        let mut value: serde_json::Value =
            serde_json::from_str(&boundary_request_json(1_000)).expect("json");
        value["constraints"]["input_token_budget"] = json!(1);
        let error =
            Request::from_value(&value).expect_err("derived input exceeding budget must fail");
        assert!(matches!(
            error,
            crate::error::ProtocolError::OverBudget { .. }
        ));
    }

    #[test]
    fn unknown_version_fails_closed() {
        let mut value: serde_json::Value =
            serde_json::from_str(&boundary_request_json(1_000)).expect("json");
        value["protocol_version"] = json!("2");
        let error = Request::from_value(&value).expect_err("unknown version must fail");
        assert!(matches!(
            error,
            crate::error::ProtocolError::UnsupportedVersion { .. }
        ));
    }
}
