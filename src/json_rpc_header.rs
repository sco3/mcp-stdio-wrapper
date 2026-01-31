use jsonrpc_core::Id;
use serde::Deserialize;
use aws_smithy_json::deserialize::Token;

#[derive(Deserialize, Debug)]
struct JsonRpcHeader {
    id: serde_json::Value,
}
/// parses string for jsonrpc id
/// # Errors
///
/// Returns an error if the string is not valid JSON, or if the `id` field is missing or not a `u64`.
pub fn parse_id(json_str: &str) -> Result<serde_json::Value, serde_json::Error> {
    let header: JsonRpcHeader = serde_json::from_str(json_str)?;
    Ok(header.id)
}

/// Finds the first "id" value from a JSON string using aws_smithy_json streaming parser.
///
/// This function is optimized for performance on large JSON payloads by avoiding
/// full deserialization.
///
/// # Returns
///
/// * `Some(Id)` containing the `id` if found.
/// * `None` if the JSON is invalid, not an object, or does not contain an "id" key.
#[must_use]
pub fn find_first_id(json: &str) -> Option<Id> {
    let mut tokens = aws_smithy_json::deserialize::json_token_iter(json.as_bytes()).peekable();
    
    // Expect start of object
    match tokens.next()? {
        Ok(Token::StartObject { .. }) => {},
        _ => return None,
    }
    
    // Iterate through object keys
    while let Some(token) = tokens.next() {
        match token {
            Ok(Token::ObjectKey { key, .. }) => {
                // Convert EscapedStr to string for comparison
                if let Ok(key_str) = key.to_unescaped() {
                    if key_str.as_ref() == "id" {
                        // Next token should be the value
                        if let Some(Ok(value_token)) = tokens.next() {
                            return match value_token {
                                Token::ValueNumber { value, .. } => {
                                    // Use to_f64_lossy to convert Number to f64
                                    let f: f64 = value.to_f64_lossy();
                                    if f >= 0.0 && f.fract() == 0.0 && f <= u64::MAX as f64 {
                                        Some(Id::Num(f as u64))
                                    } else {
                                        None
                                    }
                                },
                                Token::ValueString { value, .. } => {
                                    if let Ok(s) = value.to_unescaped() {
                                        Some(Id::Str(s.as_ref().to_string()))
                                    } else {
                                        None
                                    }
                                },
                                Token::ValueNull { .. } => Some(Id::Null),
                                _ => None,
                            };
                        }
                        return None;
                    }
                }
                // Skip the value for this key
                tokens.next();
            },
            Ok(Token::EndObject { .. }) => break,
            Err(_) => return None,
            _ => {},
        }
    }
    
    None
}
