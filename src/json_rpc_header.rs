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
    
    let mut in_object = false;
    let mut found_id_field = false;
    
    while let Some(event) = parser.next_event().ok()? {
        match event {
            JsonEvent::StartObject => {
                in_object = true;
            }
            JsonEvent::FieldName => {
                if in_object {
                    if let Ok(field_name) = parser.current_str() {
                        if field_name == "id" {
                            found_id_field = true;
                        }
                    }
                }
            }
            JsonEvent::ValueInt => {
                if found_id_field {
                    if let Ok(num_str) = parser.current_str() {
                        if let Ok(num) = num_str.parse::<u64>() {
                            return Some(Id::Num(num));
                        }
                    }
                    return None;
                }
            }
            JsonEvent::ValueString => {
                if found_id_field {
                    if let Ok(s) = parser.current_str() {
                        return Some(Id::Str(s.to_string()));
                    }
                    return None;
                }
            }
            JsonEvent::ValueNull => {
                if found_id_field {
                    return Some(Id::Null);
                }
            }
            JsonEvent::EndObject => {
                if in_object && !found_id_field {
                    return None;
                }
            }

            _ => {}
        }
    }
    
    None
}
