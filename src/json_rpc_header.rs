use serde::Deserialize;
use serde_json::Value;
use struson::reader::{JsonStreamReader, JsonReader};

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

#[must_use] 
pub fn find_first_id(json: &str) -> Option<Value> {
    let mut reader = JsonStreamReader::new(json.as_bytes());

    // Start reading the top-level object
    if reader.begin_object().is_err() {
        return None;
    }

    // Loop through the keys of the object
    while let Ok(true) = reader.has_next() {
        let name = reader.next_name().ok()?;

        if name == "id" {
            // We found the key!
            let id: Value = reader.deserialize_next().ok()?;
            return Some(id);
        }
        reader.skip_value().ok()?;
    }

    None
}
