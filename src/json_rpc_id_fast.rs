use actson::feeder::SliceJsonFeeder;
use actson::{JsonEvent, JsonParser};
use jsonrpc_core::Id;


#[must_use] 
pub fn parse_id_fast(json: &str)->Option<Id> {
    parse_field_fast(json,"id")
    
}
#[must_use] 
pub fn parse_field_fast(json: &str, field_name: &str) -> Option<Id> {
    let feeder = SliceJsonFeeder::new(json.as_bytes());
    let mut parser = JsonParser::new(feeder);
    let mut depth = 0;

    while let Ok(Some(event)) = parser.next_event() {
        match event {
            JsonEvent::StartObject | JsonEvent::StartArray => {
                depth += 1;
                // If we are already deep, skip this whole container
                if depth > 1 {
                    skip_container(&mut parser);
                    depth -= 1; // Correct the depth after skipping
                }
            }
            JsonEvent::EndObject | JsonEvent::EndArray => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            JsonEvent::FieldName => {
                if depth == 1
                    && parser
                        .current_str()
                        .is_ok_and(|name| name == field_name)
                {
                    // Get the very next event (the value)
                    if let Ok(Some(val_event)) = parser.next_event() {
                        let s = parser.current_str().ok()?;
                        return to_id(&val_event, s);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Skips a container entirely.
/// When this starts, we just consumed StartObject/StartArray.
fn skip_container(parser: &mut JsonParser<SliceJsonFeeder>) {
    let mut skip_depth = 1;
    while skip_depth > 0 {
        if let Ok(Some(sub_event)) = parser.next_event() {
            match sub_event {
                JsonEvent::StartObject | JsonEvent::StartArray => skip_depth += 1,
                JsonEvent::EndObject | JsonEvent::EndArray => skip_depth -= 1,
                _ => {}
            }
        } else {
            break;
        }
    }
}

pub fn to_id(event: &JsonEvent, value_str: &str) -> Option<Id> {
    match event {
        JsonEvent::ValueInt => value_str.parse::<u64>().ok().map(Id::Num),
        JsonEvent::ValueString => Some(Id::Str(value_str.to_string())),
        JsonEvent::ValueNull => Some(Id::Null),
        _ => None,
    }
}
