use serde::Deserialize;
use serde_json::Value;

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
/// Finds the first "id" value using a fast SAX-style byte search.
pub fn find_first_id(json: &str) -> Option<Value> {
    let bytes = json.as_bytes();
    let key = b"\"id\"";
    let key_len = key.len();

    let mut pos = 0;

    while let Some(hit) = bytes[pos..].windows(key_len).position(|w| w == key) {
        let absolute_hit = pos + hit;
        let next_pos = absolute_hit + key_len;

        if let Some(stripped) = json[next_pos..].trim_start().strip_prefix(':') {
            let value_part = stripped.trim_start();

            let mut de = serde_json::Deserializer::from_str(value_part);
            if let Ok(id_val) = Value::deserialize(&mut de) {
                return Some(id_val);
            }
        }

        pos = next_pos;
    }
    None
}
