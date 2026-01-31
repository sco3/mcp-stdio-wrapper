use actson::feeder::SliceJsonFeeder;
use actson::{JsonEvent, JsonParser};
use jsonrpc_core::Id;
use serde::Deserialize;

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

/// Finds the first "id" value from a JSON string using `actson` streaming parser.
///
/// This function is optimized for performance on large JSON payloads by using
/// event-based streaming parsing that avoids full deserialization.
///
/// # Returns
///
/// * `Some(Id)` containing the `id` if found.
/// * `None` if the JSON is invalid, not an object, or does not contain an "id" key.
#[must_use]
pub fn find_first_id_actson(json: &str) -> Option<Id> {
    let feeder = SliceJsonFeeder::new(json.as_bytes());
    let mut parser = JsonParser::new(feeder);

    let mut depth = 0;
    let mut is_next_val_id = false;

    while let Some(event) = parser.next_event().ok()? {
        if is_next_val_id {
            match event {
                JsonEvent::ValueInt => {
                    if let Ok(num_str) = parser.current_str() {
                        if let Ok(num) = num_str.parse::<u64>() {
                            return Some(Id::Num(num));
                        }
                    }
                    return None;
                }
                JsonEvent::ValueString => {
                    if let Ok(s) = parser.current_str() {
                        return Some(Id::Str(s.to_string()));
                    }
                    return None;
                }
                JsonEvent::ValueNull => {
                    return Some(Id::Null);
                }
                // Any other value type for "id" is invalid for JSON-RPC.
                _ => return None,
            }
        }

        match event {
            JsonEvent::StartObject => {
                depth += 1;
            }
            JsonEvent::EndObject => {
                depth -= 1;
                if depth == 0 {
                    // Finished the top-level object. If we haven't returned yet,
                    // there was no "id" or it was invalid.
                    return None;
                }
            }
            JsonEvent::FieldName => {
                if depth == 1 {
                    if let Ok(field_name) = parser.current_str() {
                        if field_name == "id" {
                            is_next_val_id = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    None
}
