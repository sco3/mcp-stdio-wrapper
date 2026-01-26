use jsonrpc_core::Id;
use serde::Deserialize;
use struson::reader::{JsonReader, JsonStreamReader};

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

/// Finds the first "id" value from a JSON string using a streaming parser.
///
/// This function is optimized for performance on large JSON payloads by avoiding
/// full deserialization.
///
/// # Returns
///
/// * `Some(Value)` containing the `id` if found.
/// * `None` if the JSON is invalid, not an object, or does not contain an "id" key.
#[must_use]
pub fn find_first_id(json: &str) -> Option<Id> {
    let mut reader = JsonStreamReader::new(json.as_bytes());

    if reader.begin_object().is_err() {
        return None;
    }

    while let Ok(true) = reader.has_next() {
        let name = reader.next_name().ok()?;
        if name == "id" {
            // Deserialize directly into the jsonrpc_core::Id type
            let id: Id = reader.deserialize_next().ok()?;
            return Some(id);
        }
        reader.skip_value().ok()?;
    }
    None
}
