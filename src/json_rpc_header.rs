use crate::json_rpc_event::process_event;
use crate::json_rpc_value::get_value;
use actson::feeder::SliceJsonFeeder;
use actson::JsonParser;
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
            return get_value(&mut parser, event);
        }
        if let Some(value) = process_event(
            &mut parser, //
            &mut depth,
            &mut is_next_val_id,
            event,
        ) {
            return value;
        }
    }

    None
}
