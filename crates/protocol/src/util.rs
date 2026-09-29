//! Shared private parsing helpers used by the envelope validators.

use serde_json::{Map, Number, Value};

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

/// Extracts a required non-negative integer field. JSON Schema integers are
/// mathematical integers, so integral decimal representations such as `1.0`
/// are accepted as well as integer tokens. Negative, fractional, non-finite,
/// and out-of-u64-range numbers fail closed; range policy is applied
/// afterwards by the semantic validators with specific typed errors.
pub(crate) fn uint_field(
    object: &Map<String, Value>,
    field: &'static str,
) -> Result<u64, ProtocolError> {
    let value = object
        .get(field)
        .ok_or(ProtocolError::MissingField { field })?;
    if let Some(integer) = value.as_u64() {
        return Ok(integer);
    }
    let integer = value.as_number().and_then(exact_u64_integer);
    integer.ok_or(ProtocolError::MalformedJson {
        reason: "expected a non-negative integer",
    })
}

/// Converts the exact JSON number representation to `u64` only when it is a
/// mathematical non-negative integer. `serde_json/arbitrary_precision`
/// preserves the source decimal so a fractional value cannot round to an
/// integer before this check.
fn exact_u64_integer(number: &Number) -> Option<u64> {
    if let Some(integer) = number.as_u64() {
        return Some(integer);
    }

    let token = number.to_string();
    let (mantissa, exponent_text) = token
        .split_once(['e', 'E'])
        .map_or((token.as_str(), None), |(mantissa, exponent)| {
            (mantissa, Some(exponent))
        });
    let negative = mantissa.starts_with('-');
    let unsigned_mantissa = mantissa.strip_prefix('-').unwrap_or(mantissa);
    let (whole, fraction) = unsigned_mantissa
        .split_once('.')
        .map_or((unsigned_mantissa, ""), |(whole, fraction)| {
            (whole, fraction)
        });
    let digits = format!("{whole}{fraction}");
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if digits.bytes().all(|byte| byte == b'0') {
        return Some(0);
    }
    if negative {
        return None;
    }

    let exponent = exponent_text.map_or(Some(0), |text| text.parse::<i64>().ok())?;
    let decimal_position = i64::try_from(whole.len()).ok()?.checked_add(exponent)?;
    if decimal_position <= 0 {
        return None;
    }

    let digit_count = i64::try_from(digits.len()).ok()?;
    if decimal_position < digit_count
        && digits[usize::try_from(decimal_position).ok()?..]
            .bytes()
            .any(|byte| byte != b'0')
    {
        return None;
    }

    let integer_digits = &digits[..usize::try_from(decimal_position.min(digit_count)).ok()?];
    let significant = integer_digits.trim_start_matches('0');
    let appended_zeroes = usize::try_from((decimal_position - digit_count).max(0)).ok()?;
    if significant.len().checked_add(appended_zeroes)? > 20 {
        return None;
    }
    let mut integer = significant.to_owned();
    integer.push_str(&"0".repeat(appended_zeroes));
    integer.parse().ok()
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

/// Returns whether a JSON number is mathematically within the closed unit
/// interval without first rounding its decimal representation to `f64`.
pub(crate) fn is_unit_interval_number(value: &Value) -> bool {
    let Some(number) = value.as_number() else {
        return false;
    };
    let token = number.to_string();
    let (mantissa, exponent_text) = token
        .split_once(['e', 'E'])
        .map_or((token.as_str(), None), |(mantissa, exponent)| {
            (mantissa, Some(exponent))
        });
    let negative = mantissa.starts_with('-');
    let unsigned_mantissa = mantissa.strip_prefix('-').unwrap_or(mantissa);
    let (whole, fraction) = unsigned_mantissa
        .split_once('.')
        .map_or((unsigned_mantissa, ""), |(whole, fraction)| {
            (whole, fraction)
        });
    let digits = format!("{whole}{fraction}");
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    if digits.bytes().all(|byte| byte == b'0') {
        return true;
    }
    if negative {
        return false;
    }

    let exponent = match exponent_text {
        None => 0,
        Some(text) => match text.parse::<i64>() {
            Ok(exponent) => exponent,
            Err(_) => return text.starts_with('-'),
        },
    };
    let Some(decimal_position) = i64::try_from(whole.len())
        .ok()
        .and_then(|length| length.checked_add(exponent))
    else {
        return exponent < 0;
    };
    let leading_zeroes = digits.bytes().take_while(|byte| *byte == b'0').count();
    let Ok(leading_zeroes) = i64::try_from(leading_zeroes) else {
        return false;
    };
    let Some(significant_decimal_position) = decimal_position.checked_sub(leading_zeroes) else {
        return exponent < 0;
    };
    if significant_decimal_position <= 0 {
        return true;
    }
    if significant_decimal_position > 1 {
        return false;
    }

    let significant = digits.trim_start_matches('0');
    significant.starts_with('1') && significant[1..].bytes().all(|byte| byte == b'0')
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
    use serde_json::{Map, Value};

    use super::{exact_u64_integer, is_unit_interval_number, is_uuid, uint_field};

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

    #[test]
    fn integer_number_conversion_preserves_exact_decimal_semantics() {
        for (token, expected) in [("1.0", 1), ("8.00e2", 800), ("100e-2", 1)] {
            let value: Value = serde_json::from_str(token).expect("JSON number");
            assert_eq!(
                exact_u64_integer(value.as_number().expect("number")),
                Some(expected),
                "{token} should represent an integer"
            );
        }

        for token in ["800.000000000000001", "1.5", "1e-1", "-1", "1e30"] {
            let value: Value = serde_json::from_str(token).expect("JSON number");
            assert_eq!(
                exact_u64_integer(value.as_number().expect("number")),
                None,
                "{token} should not be accepted as a bounded unsigned integer"
            );
        }

        let object: Map<String, Value> =
            serde_json::from_str(r#"{"budget":800.000000000000001}"#).expect("object");
        assert!(uint_field(&object, "budget").is_err());
    }

    #[test]
    fn unit_interval_check_preserves_exact_decimal_bounds() {
        for token in [
            "0",
            "-0.0",
            "0.999999999999999999999",
            "1",
            "1.0000",
            "10e-1",
            "100e-2",
            "1e-999999999999999999999999999999",
        ] {
            let value: Value = serde_json::from_str(token).expect("JSON number");
            assert!(is_unit_interval_number(&value), "{token} is within [0, 1]");
        }

        for token in [
            "1.00000000000000001",
            "1.0000000000000000000000000001",
            "-0.00000000000000001",
            "-1e-999999999999999999999999999999",
            "1e400",
            "-1e400",
            "10",
        ] {
            let value: Value = serde_json::from_str(token).expect("JSON number");
            assert!(
                !is_unit_interval_number(&value),
                "{token} is outside [0, 1]"
            );
        }
    }
}
