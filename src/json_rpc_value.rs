use actson::feeder::SliceJsonFeeder;
use actson::{JsonEvent, JsonParser};
use jsonrpc_core::Id;

pub fn get_value(parser: &mut JsonParser<SliceJsonFeeder>, event: JsonEvent) -> Option<Id> {
    match event {
        JsonEvent::ValueInt => parser
            .current_str()
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Id::Num),
        JsonEvent::ValueString => parser.current_str().ok().map(|s| Id::Str(s.to_string())),
        JsonEvent::ValueNull => Some(Id::Null),
        // Any other value type for "id" is invalid for JSON-RPC.
        _ => None,
    }
}
