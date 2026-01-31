use actson::feeder::SliceJsonFeeder;
use actson::{JsonEvent, JsonParser};
use jsonrpc_core::Id;

pub fn process_event(
    parser: &mut JsonParser<SliceJsonFeeder>,
    depth: &mut i32,
    is_next_val_id: &mut bool,
    event: JsonEvent,
) -> Option<Option<Id>> {
    match event {
        JsonEvent::StartObject => {
            *depth += 1;
        }
        JsonEvent::EndObject => {
            *depth -= 1;
            if *depth == 0 {
                // Finished the top-level object. If we haven't returned yet,
                // there was no "id" or it was invalid.
                return Some(None);
            }
        }
        JsonEvent::FieldName => {
            if *depth == 1 {
                if let Ok(field_name) = parser.current_str() {
                    if field_name == "id" {
                        *is_next_val_id = true;
                    }
                }
            }
        }
        _ => {}
    }
    None
}
