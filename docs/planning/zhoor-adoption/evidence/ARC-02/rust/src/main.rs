// NON-PRODUCTION ARC-02 SPIKE
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema_version: u64,
    request_id: String,
    operation: String,
    goal: String,
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Candidate {
    id: String,
    score: i64,
}

#[derive(Debug, Serialize)]
struct Response {
    status: &'static str,
    request_id: Option<String>,
    selected_id: Option<String>,
    score: Option<i64>,
    error: Option<&'static str>,
}

fn rejected(code: &'static str) -> Response {
    Response { status: "error", request_id: None, selected_id: None, score: None, error: Some(code) }
}

fn process(line: &str) -> Response {
    let value: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(_) => return rejected("invalid_json"),
    };
    let req: Request = match serde_json::from_value(value) {
        Ok(v) => v,
        Err(_) => return rejected("invalid_request"),
    };
    if req.schema_version != 1 || req.request_id.is_empty() || req.operation != "select" || req.goal.is_empty() {
        return rejected("invalid_request");
    }
    let mut ids = std::collections::HashSet::new();
    if req.candidates.iter().any(|c| c.id.is_empty() || !ids.insert(c.id.as_str())) {
        return rejected("invalid_request");
    }
    if req.candidates.is_empty() {
        return Response { status: "abstained", request_id: Some(req.request_id), selected_id: None, score: None, error: None };
    }
    let selected = req.candidates.iter().max_by(|a, b| a.score.cmp(&b.score).then_with(|| b.id.cmp(&a.id))).unwrap();
    Response { status: "resolved", request_id: Some(req.request_id), selected_id: Some(selected.id.clone()), score: Some(selected.score), error: None }
}

fn main() {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                let out = serde_json::to_string(&process(&line)).expect("response serializes");
                println!("{out}");
                io::stdout().flush().expect("stdout flush");
            }
            Err(_) => break,
        }
    }
}
