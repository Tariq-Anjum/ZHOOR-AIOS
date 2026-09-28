# NON-PRODUCTION ARC-02 SPIKE
"""Stdlib-only JSONL request validator and deterministic candidate selector."""
from dataclasses import dataclass
import json
import sys

TOP_KEYS = {"schema_version", "request_id", "operation", "goal", "candidates"}
SCORE_MIN = -(2**63)
SCORE_MAX = 2**63 - 1

@dataclass(frozen=True)
class Candidate:
    id: str
    score: int

def response(status, request_id=None, selected_id=None, score=None, error=None):
    return {"status": status, "request_id": request_id, "selected_id": selected_id, "score": score, "error": error}

def process(line):
    try:
        data = json.loads(line)
    except (json.JSONDecodeError, UnicodeDecodeError):
        return response("error", error="invalid_json")
    if not isinstance(data, dict) or set(data) != TOP_KEYS:
        return response("error", error="invalid_request")
    if type(data["schema_version"]) is not int or data["schema_version"] != 1:
        return response("error", error="invalid_request")
    if not isinstance(data["request_id"], str) or not data["request_id"]:
        return response("error", error="invalid_request")
    if data["operation"] != "select" or not isinstance(data["operation"], str):
        return response("error", error="invalid_request")
    if not isinstance(data["goal"], str) or not data["goal"]:
        return response("error", error="invalid_request")
    if not isinstance(data["candidates"], list):
        return response("error", error="invalid_request")
    candidates = []
    ids = set()
    for item in data["candidates"]:
        if not isinstance(item, dict) or set(item) != {"id", "score"}:
            return response("error", error="invalid_request")
        if (
            not isinstance(item["id"], str)
            or not item["id"]
            or type(item["score"]) is not int
            or not SCORE_MIN <= item["score"] <= SCORE_MAX
            or item["id"] in ids
        ):
            return response("error", error="invalid_request")
        ids.add(item["id"])
        candidates.append(Candidate(item["id"], item["score"]))
    if not candidates:
        return response("abstained", request_id=data["request_id"])
    selected = min(candidates, key=lambda c: (-c.score, c.id))
    return response("resolved", data["request_id"], selected.id, selected.score)

def main():
    for line in sys.stdin:
        print(json.dumps(process(line), separators=(",", ":")), flush=True)

if __name__ == "__main__":
    main()
