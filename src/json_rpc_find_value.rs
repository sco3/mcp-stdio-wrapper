

/// Extracts the value of a specific field from JSON bytes as a slice reference.
/// 
/// This function attempts to return a slice of the original input bytes when possible.
/// For string and integer values, it returns a slice from the input.
/// For boolean and null values, it returns static byte slices.
/// 
/// # Arguments
/// * `json_bytes` - The JSON data as bytes
/// * `target_field` - The name of the field to find (e.g., "id", "method")
/// 
/// # Returns
/// * `Some(&[u8])` - A slice reference to the field value bytes
/// * `None` - If the field is not found or parsing fails
pub fn find_json_value<'a>(json_bytes: &'a [u8], target_field: &str) -> Option<&'a [u8]> {
    // Find the field name position first
    let field_pattern = format!("\"{}\"", target_field);
    let field_bytes = field_pattern.as_bytes();
    
    let mut search_pos = 0;
    while let Some(pos) = find_bytes(&json_bytes[search_pos..], field_bytes) {
        let abs_pos = search_pos + pos;
        
        // Check if this is a field name (followed by ':')
        let after_field = abs_pos + field_bytes.len();
        if let Some(colon_pos) = find_next_non_whitespace(&json_bytes[after_field..]) {
            if json_bytes[after_field + colon_pos] == b':' {
                // Found the field, now extract the value
                let value_start = after_field + colon_pos + 1;
                if let Some(value_slice) = extract_value_slice(&json_bytes[value_start..]) {
                    return Some(value_slice);
                }
            }
        }
        
        search_pos = abs_pos + 1;
    }
    
    None
}

/// Finds a byte pattern in a slice
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

/// Finds the next non-whitespace character position
fn find_next_non_whitespace(bytes: &[u8]) -> Option<usize> {
    bytes.iter().position(|&b| !matches!(b, b' ' | b'\t' | b'\n' | b'\r'))
}

/// Extracts a JSON value slice from the beginning of the input
fn extract_value_slice(bytes: &[u8]) -> Option<&[u8]> {
    let start = find_next_non_whitespace(bytes)?;
    let bytes = &bytes[start..];
    
    if bytes.is_empty() {
        return None;
    }
    
    match bytes[0] {
        b'"' => {
            // String value - find closing quote
            let mut i = 1;
            while i < bytes.len() {
                if bytes[i] == b'"' && bytes[i - 1] != b'\\' {
                    return Some(&bytes[1..i]); // Return without quotes
                }
                i += 1;
            }
            None
        }
        b't' if bytes.starts_with(b"true") => Some(b"true"),
        b'f' if bytes.starts_with(b"false") => Some(b"false"),
        b'n' if bytes.starts_with(b"null") => Some(b"null"),
        b'-' | b'0'..=b'9' => {
            // Number value
            let mut i = 0;
            if bytes[i] == b'-' {
                i += 1;
            }
            while i < bytes.len() && matches!(bytes[i], b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-') {
                i += 1;
            }
            if i > 0 {
                Some(&bytes[..i])
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_json_value_bytes_string() {
        let json = br#"{"name": "test", "id": 123}"#;
        let result = find_json_value(json, "name");
        assert_eq!(result, Some(b"test".as_ref()));
    }

    #[test]
    fn test_get_json_value_bytes_number() {
        let json = br#"{"name": "test", "id": 123}"#;
        let result = find_json_value(json, "id");
        assert_eq!(result, Some(b"123".as_ref()));
    }

    #[test]
    fn test_get_json_value_bytes_not_found() {
        let json = br#"{"name": "test", "id": 123}"#;
        let result = find_json_value(json, "missing");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_json_value_bytes_null() {
        let json = br#"{"name": null, "id": 123}"#;
        let result = find_json_value(json, "name");
        assert_eq!(result, Some(b"null".as_ref()));
    }

    #[test]
    fn test_get_json_value_bytes_boolean() {
        let json = br#"{"active": true, "id": 123}"#;
        let result = find_json_value(json, "active");
        assert_eq!(result, Some(b"true".as_ref()));
    }

    #[test]
    fn test_get_json_value_bytes_negative_number() {
        let json = br#"{"value": -42, "id": 123}"#;
        let result = find_json_value(json, "value");
        assert_eq!(result, Some(b"-42".as_ref()));
    }

    #[test]
    fn test_get_json_value_bytes_float() {
        let json = br#"{"value": 3.14, "id": 123}"#;
        let result = find_json_value(json, "value");
        assert_eq!(result, Some(b"3.14".as_ref()));
    }
}