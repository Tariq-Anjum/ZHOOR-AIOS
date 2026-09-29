//! Shared private parsing helpers used by the envelope validators.

use serde_json::{Map, Value};

use crate::error::ProtocolError;

/// Reports whether a string is a lowercase canonical UUID of the form
/// `8-4-4-4-12` hexadecimal groups separated by hyphens. This mirrors the
/// `uuid` pattern in the canonical schemas, which accepts lowercase hex only.
pub(crate) fn is_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }
    bytes.iter().enumerate().all(|(index, byte)| {
        if matches!(index, 8 | 13 | 18 | 23) {
            *byte == b'-'
        } else {
            matches!(byte, b'0'..=b'9' | b'a'..=b'f')
        }
    })
}

/// Checks that a JSON value is an object whose keys are exactly the known
/// field names and that contains every required field name. Returns the
/// object on success. Unknown fields fail closed with
/// [`ProtocolError::UnknownField`]; absent required fields fail closed with
/// [`ProtocolError::MissingField`].
pub(crate) fn object_fields<'a>(
    value: &'a Value,
    known: &[&str],
    required: &[&'static str],
) -> Result<&'a Map<String, Value>, ProtocolError> {
    let object = value.as_object().ok_or(ProtocolError::MalformedJson {
        reason: "expected a JSON object",
    })?;
    object_fields_map(object, known, required)
}

/// Checks a JSON object map against a closed field set and required fields.
pub(crate) fn object_fields_map<'a>(
    object: &'a Map<String, Value>,
    known: &[&str],
    required: &[&'static str],
) -> Result<&'a Map<String, Value>, ProtocolError> {
    for key in object.keys() {
        if !known.contains(&key.as_str()) {
            return Err(ProtocolError::UnknownField { field: key.clone() });
        }
    }
    for field in required {
        if !object.contains_key(*field) {
            return Err(ProtocolError::MissingField { field });
        }
    }
    Ok(object)
}

/// Extracts a required string field with length bounds measured in
/// characters.
pub(crate) fn str_field(
    object: &Map<String, Value>,
    field: &'static str,
    min_len: usize,
    max_len: usize,
) -> Result<String, ProtocolError> {
    let value = object
        .get(field)
        .ok_or(ProtocolError::MissingField { field })?;
    let text = value.as_str().ok_or(ProtocolError::MalformedJson {
        reason: "expected a JSON string",
    })?;
    let len = text.chars().count();
    if len < min_len || len > max_len {
        return Err(ProtocolError::MalformedJson {
            reason: "string length out of range",
        });
    }
    Ok(text.to_owned())
}

/// Extracts a required non-negative integer field. Negative, fractional, or
/// out-of-u64-range numbers fail closed as malformed JSON; range policy is
/// applied afterwards by the semantic validators with specific typed errors.
pub(crate) fn uint_field(
    object: &Map<String, Value>,
    field: &'static str,
) -> Result<u64, ProtocolError> {
    let value = object
        .get(field)
        .ok_or(ProtocolError::MissingField { field })?;
    value.as_u64().ok_or(ProtocolError::MalformedJson {
        reason: "expected a non-negative integer",
    })
}

/// Extracts a required finite JSON number field.
pub(crate) fn f64_field(
    object: &Map<String, Value>,
    field: &'static str,
) -> Result<f64, ProtocolError> {
    let value = object
        .get(field)
        .ok_or(ProtocolError::MissingField { field })?;
    let number = value.as_f64().ok_or(ProtocolError::MalformedJson {
        reason: "expected a JSON number",
    })?;
    if !number.is_finite() {
        return Err(ProtocolError::MalformedJson {
            reason: "expected a finite JSON number",
        });
    }
    Ok(number)
}

/// Extracts a required JSON array field.
pub(crate) fn array_field<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
) -> Result<&'a Vec<Value>, ProtocolError> {
    let value = object
        .get(field)
        .ok_or(ProtocolError::MissingField { field })?;
    value.as_array().ok_or(ProtocolError::MalformedJson {
        reason: "expected a JSON array",
    })
}

/// Extracts a required field that may be `null` or a bounded string.
pub(crate) fn optional_str_field(
    object: &Map<String, Value>,
    field: &'static str,
    max_len: usize,
) -> Result<Option<String>, ProtocolError> {
    let value = object
        .get(field)
        .ok_or(ProtocolError::MissingField { field })?;
    match value {
        Value::Null => Ok(None),
        Value::String(text) => {
            let len = text.chars().count();
            if len < 1 || len > max_len {
                return Err(ProtocolError::MalformedJson {
                    reason: "string length out of range",
                });
            }
            Ok(Some(text.clone()))
        }
        _ => Err(ProtocolError::MalformedJson {
            reason: "expected a JSON string or null",
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::is_uuid;

    #[test]
    fn canonical_lowercase_uuid_is_accepted() {
        assert!(is_uuid("11111111-1111-4111-8111-111111111111"));
    }

    #[test]
    fn malformed_uuids_are_rejected() {
        assert!(!is_uuid("not-a-uuid"));
        assert!(!is_uuid("11111111-1111-4111-8111-111111111111-1"));
        assert!(!is_uuid("AAAAAAAA-0001-4000-8000-000000000001"));
        assert!(!is_uuid(""));
    }
}
