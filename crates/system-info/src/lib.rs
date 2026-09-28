use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const MAX_REQUEST_BYTES: usize = 4096;
const SCHEMA_VERSION: u16 = 1;
const CAPABILITY_ID: &str = "system.info.read";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema_version: u16,
    request_id: String,
    intent: TypedIntent,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TypedIntent {
    capability_id: String,
    arguments: Value,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyArguments {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SystemInfo {
    pub os: String,
    pub architecture: String,
    pub kernel_release: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionStatus {
    Completed,
    Rejected,
    Denied,
    Failed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    RequestTooLarge,
    InvalidRequest,
    InvalidRequestId,
    UnsupportedSchema,
    UnknownCapability,
    PolicyDenied,
    BackendUnavailable,
    ExecutionFailed,
    VerificationFailed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Response {
    pub schema_version: u16,
    pub request_id: Option<String>,
    pub capability_id: Option<String>,
    pub status: CompletionStatus,
    pub system_info: Option<SystemInfo>,
    pub error: Option<ErrorCode>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerStage {
    InputValidation,
    RegistryLookup,
    Policy,
    Execution,
    Verification,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerOutcome {
    Completed,
    Rejected,
    Denied,
    Failed,
}

/// One bounded audit event. It intentionally omits system values and raw input.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LedgerEvent {
    pub schema_version: u16,
    pub request_id: Option<String>,
    pub capability_id: Option<String>,
    pub stage: LedgerStage,
    pub outcome: LedgerOutcome,
    pub reason: Option<ErrorCode>,
}

/// The CLI returns the event to its caller; durable append storage is a later layer.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProcessOutput {
    pub response: Response,
    pub ledger_event: LedgerEvent,
}

#[derive(Clone, Copy)]
struct CapabilityDescriptor {
    id: &'static str,
    read_only: bool,
}

const CAPABILITIES: [CapabilityDescriptor; 1] = [CapabilityDescriptor {
    id: CAPABILITY_ID,
    read_only: true,
}];

trait InfoProvider: Send + Sync {
    fn read_system_info(&self) -> Result<SystemInfo, ()>;
}

struct LinuxInfoProvider;

impl InfoProvider for LinuxInfoProvider {
    fn read_system_info(&self) -> Result<SystemInfo, ()> {
        let kernel_release = fs::read_to_string("/proc/sys/kernel/osrelease").map_err(|_| ())?;
        Ok(SystemInfo {
            os: std::env::consts::OS.to_owned(),
            architecture: std::env::consts::ARCH.to_owned(),
            kernel_release: kernel_release.trim().to_owned(),
        })
    }
}

struct Policy {
    enabled: bool,
}

pub struct Engine {
    policy: Policy,
    effective_uid: Option<u32>,
    provider: Option<Box<dyn InfoProvider>>,
}

impl Engine {
    /// Create the fixed read-only capability engine for the current Linux user.
    pub fn system_info() -> Self {
        Self {
            policy: Policy { enabled: true },
            effective_uid: effective_uid(),
            provider: Some(Box::new(LinuxInfoProvider)),
        }
    }

    /// Validate and process one bounded JSON request, returning its compact ledger event.
    pub fn process(&self, input: &[u8]) -> ProcessOutput {
        if input.len() > MAX_REQUEST_BYTES {
            return self.finish(
                None,
                None,
                LedgerStage::InputValidation,
                Some(ErrorCode::RequestTooLarge),
                None,
            );
        }

        let request: Request = match serde_json::from_slice(input) {
            Ok(request) => request,
            Err(_) => {
                return self.finish(
                    None,
                    None,
                    LedgerStage::InputValidation,
                    Some(ErrorCode::InvalidRequest),
                    None,
                );
            }
        };

        let request_id = if valid_request_id(&request.request_id) {
            request.request_id
        } else {
            return self.finish(
                None,
                None,
                LedgerStage::InputValidation,
                Some(ErrorCode::InvalidRequestId),
                None,
            );
        };
        let capability_id = request.intent.capability_id;

        if request.schema_version != SCHEMA_VERSION {
            return self.finish(
                Some(request_id),
                Some(capability_id),
                LedgerStage::InputValidation,
                Some(ErrorCode::UnsupportedSchema),
                None,
            );
        }

        let Some(capability) = CAPABILITIES.iter().find(|entry| entry.id == capability_id) else {
            return self.finish(
                Some(request_id),
                Some(capability_id),
                LedgerStage::RegistryLookup,
                Some(ErrorCode::UnknownCapability),
                None,
            );
        };

        if serde_json::from_value::<EmptyArguments>(request.intent.arguments).is_err() {
            return self.finish(
                Some(request_id),
                Some(capability_id),
                LedgerStage::InputValidation,
                Some(ErrorCode::InvalidRequest),
                None,
            );
        }

        if !self.policy.enabled
            || !capability.read_only
            || self.effective_uid.is_none()
            || self.effective_uid == Some(0)
        {
            return self.finish(
                Some(request_id),
                Some(capability_id),
                LedgerStage::Policy,
                Some(ErrorCode::PolicyDenied),
                None,
            );
        }

        let Some(provider) = self.provider.as_deref() else {
            return self.finish(
                Some(request_id),
                Some(capability_id),
                LedgerStage::Execution,
                Some(ErrorCode::BackendUnavailable),
                None,
            );
        };

        let info = match provider.read_system_info() {
            Ok(info) => info,
            Err(()) => {
                return self.finish(
                    Some(request_id),
                    Some(capability_id),
                    LedgerStage::Execution,
                    Some(ErrorCode::ExecutionFailed),
                    None,
                );
            }
        };

        if !valid_system_info(&info) {
            return self.finish(
                Some(request_id),
                Some(capability_id),
                LedgerStage::Verification,
                Some(ErrorCode::VerificationFailed),
                None,
            );
        }

        self.finish(
            Some(request_id),
            Some(capability_id),
            LedgerStage::Verification,
            None,
            Some(info),
        )
    }

    fn finish(
        &self,
        request_id: Option<String>,
        capability_id: Option<String>,
        stage: LedgerStage,
        error: Option<ErrorCode>,
        system_info: Option<SystemInfo>,
    ) -> ProcessOutput {
        let (status, outcome) = match (stage, error) {
            (_, None) => (CompletionStatus::Completed, LedgerOutcome::Completed),
            (LedgerStage::Policy, Some(_)) => (CompletionStatus::Denied, LedgerOutcome::Denied),
            (LedgerStage::InputValidation | LedgerStage::RegistryLookup, Some(_)) => {
                (CompletionStatus::Rejected, LedgerOutcome::Rejected)
            }
            (_, Some(_)) => (CompletionStatus::Failed, LedgerOutcome::Failed),
        };
        ProcessOutput {
            response: Response {
                schema_version: SCHEMA_VERSION,
                request_id: request_id.clone(),
                capability_id: capability_id.clone(),
                status,
                system_info,
                error,
            },
            ledger_event: LedgerEvent {
                schema_version: SCHEMA_VERSION,
                request_id,
                capability_id,
                stage,
                outcome,
                reason: error,
            },
        }
    }
}

fn valid_request_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

fn valid_system_info(info: &SystemInfo) -> bool {
    valid_field(&info.os, 64)
        && valid_field(&info.architecture, 64)
        && valid_field(&info.kernel_release, 128)
}

fn valid_field(value: &str, max_len: usize) -> bool {
    !value.is_empty() && value.len() <= max_len && !value.chars().any(char::is_control)
}

fn effective_uid() -> Option<u32> {
    fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|line| {
            let values = line.strip_prefix("Uid:")?.split_whitespace();
            values.clone().nth(1)?.parse().ok()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    const VALID: &[u8] = br#"{"schema_version":1,"request_id":"test-1","intent":{"capability_id":"system.info.read","arguments":{}}}"#;

    fn info() -> SystemInfo {
        SystemInfo {
            os: "linux".into(),
            architecture: "x86_64".into(),
            kernel_release: "6.1.0-test".into(),
        }
    }

    fn engine(provider: Option<Box<dyn InfoProvider>>, enabled: bool) -> Engine {
        Engine {
            policy: Policy { enabled },
            effective_uid: Some(1000),
            provider,
        }
    }

    struct StaticProvider(Result<SystemInfo, ()>);

    impl InfoProvider for StaticProvider {
        fn read_system_info(&self) -> Result<SystemInfo, ()> {
            self.0.clone()
        }
    }

    struct CountingProvider(Arc<AtomicUsize>);

    impl InfoProvider for CountingProvider {
        fn read_system_info(&self) -> Result<SystemInfo, ()> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(info())
        }
    }

    #[test]
    fn valid_request_runs_every_stage_and_emits_compact_ledger_event() {
        let engine = engine(Some(Box::new(StaticProvider(Ok(info())))), true);
        let output = engine.process(VALID);

        assert_eq!(output.response.status, CompletionStatus::Completed);
        assert_eq!(output.response.system_info, Some(info()));
        assert_eq!(output.ledger_event.stage, LedgerStage::Verification);
        assert_eq!(output.ledger_event.outcome, LedgerOutcome::Completed);
        assert_eq!(output.ledger_event.request_id.as_deref(), Some("test-1"));
        assert_eq!(
            output.ledger_event.capability_id.as_deref(),
            Some(CAPABILITY_ID)
        );
        assert!(output.ledger_event.reason.is_none());
    }

    #[test]
    fn malformed_input_is_rejected_before_registry_or_execution() {
        let calls = Arc::new(AtomicUsize::new(0));
        let engine = engine(Some(Box::new(CountingProvider(calls.clone()))), true);
        let output = engine.process(b"{not-json");

        assert_eq!(output.response.error, Some(ErrorCode::InvalidRequest));
        assert_eq!(output.ledger_event.stage, LedgerStage::InputValidation);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn oversized_request_is_rejected_before_parsing_or_execution() {
        let calls = Arc::new(AtomicUsize::new(0));
        let engine = engine(Some(Box::new(CountingProvider(calls.clone()))), true);
        let output = engine.process(&vec![b'x'; MAX_REQUEST_BYTES + 1]);

        assert_eq!(output.response.error, Some(ErrorCode::RequestTooLarge));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unknown_capability_is_rejected_without_execution() {
        let calls = Arc::new(AtomicUsize::new(0));
        let engine = engine(Some(Box::new(CountingProvider(calls.clone()))), true);
        let input = br#"{"schema_version":1,"request_id":"test-2","intent":{"capability_id":"shell.run","arguments":{}}}"#;
        let output = engine.process(input);

        assert_eq!(output.response.error, Some(ErrorCode::UnknownCapability));
        assert_eq!(output.ledger_event.stage, LedgerStage::RegistryLookup);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn policy_denial_never_reaches_execution() {
        let calls = Arc::new(AtomicUsize::new(0));
        let engine = engine(Some(Box::new(CountingProvider(calls.clone()))), false);
        let output = engine.process(VALID);

        assert_eq!(output.response.status, CompletionStatus::Denied);
        assert_eq!(output.response.error, Some(ErrorCode::PolicyDenied));
        assert_eq!(output.ledger_event.stage, LedgerStage::Policy);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn root_identity_is_denied_before_execution() {
        let calls = Arc::new(AtomicUsize::new(0));
        let engine = Engine {
            policy: Policy { enabled: true },
            effective_uid: Some(0),
            provider: Some(Box::new(CountingProvider(calls.clone()))),
        };
        let output = engine.process(VALID);

        assert_eq!(output.response.error, Some(ErrorCode::PolicyDenied));
        assert_eq!(output.ledger_event.stage, LedgerStage::Policy);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unavailable_backend_fails_closed() {
        let output = engine(None, true).process(VALID);

        assert_eq!(output.response.error, Some(ErrorCode::BackendUnavailable));
        assert_eq!(output.ledger_event.stage, LedgerStage::Execution);
    }

    #[test]
    fn execution_failure_is_recorded_without_provider_details() {
        let engine = engine(Some(Box::new(StaticProvider(Err(())))), true);
        let output = engine.process(VALID);

        assert_eq!(output.response.error, Some(ErrorCode::ExecutionFailed));
        assert!(output.response.system_info.is_none());
        assert_eq!(output.ledger_event.stage, LedgerStage::Execution);
    }

    #[test]
    fn malformed_provider_result_fails_verification() {
        let malformed = SystemInfo {
            os: "linux".into(),
            architecture: "x86_64".into(),
            kernel_release: "".into(),
        };
        let engine = engine(Some(Box::new(StaticProvider(Ok(malformed)))), true);
        let output = engine.process(VALID);

        assert_eq!(output.response.error, Some(ErrorCode::VerificationFailed));
        assert!(output.response.system_info.is_none());
        assert_eq!(output.ledger_event.stage, LedgerStage::Verification);
    }

    #[test]
    fn invalid_arguments_are_rejected() {
        let calls = Arc::new(AtomicUsize::new(0));
        let engine = engine(Some(Box::new(CountingProvider(calls.clone()))), true);
        let input = br#"{"schema_version":1,"request_id":"test-3","intent":{"capability_id":"system.info.read","arguments":{"path":"/etc/passwd"}}}"#;
        let output = engine.process(input);

        assert_eq!(output.response.error, Some(ErrorCode::InvalidRequest));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unsupported_schema_and_invalid_request_id_are_rejected() {
        let engine = engine(Some(Box::new(StaticProvider(Ok(info())))), true);
        let version = engine.process(br#"{"schema_version":2,"request_id":"v2","intent":{"capability_id":"system.info.read","arguments":{}}}"#);
        let id = engine.process(br#"{"schema_version":1,"request_id":"bad id","intent":{"capability_id":"system.info.read","arguments":{}}}"#);

        assert_eq!(version.response.error, Some(ErrorCode::UnsupportedSchema));
        assert_eq!(id.response.error, Some(ErrorCode::InvalidRequestId));
    }
}
