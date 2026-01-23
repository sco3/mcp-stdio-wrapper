use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct JsonRpcHeader {
    id: u64,
}

pub fn parse_id(json_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let header: JsonRpcHeader = serde_json::from_str(json_str)?;
    Ok(header.id)
}
