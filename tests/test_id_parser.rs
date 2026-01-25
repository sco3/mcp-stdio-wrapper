use mcp_stdio_wrapper::json_rpc_header::{find_first_id, parse_id};

use std::time::Instant;

#[cfg(test)]
#[test]
fn test_parse_id_performance() {
    // 1. Setup Short Input
    let short_json = r#"{"jsonrpc": "2.0", "method": "test", "id": 123}"#;

    // 2. Setup Huge Input (Simulating a several MB payload)
    // We generate a large array of data inside the JSON to make it heavy
    let mut large_data = String::from(r#"{"jsonrpc": "2.0", "id": 999, "data": ["#);
    for i in 0..200000 {
        if i > 0 {
            large_data.push(',');
        }
        large_data.push_str(&format!(
            r#"{{"index": {i}, "payload": "some repeated data"}}"#
        ));
    }

    large_data.push_str("]}");

    println!("Large JSON size: {} MB", large_data.len() / 1_024 / 1_024);
    //println!("Large JSON: {}", large_data);

    // --- Benchmark Short ---
    let start_short = Instant::now();
    let id_short = parse_id(short_json).expect("Failed to parse short");
    let duration_short = start_short.elapsed();
    assert_eq!(id_short, 123);

    // --- Benchmark Large ---
    let start_large = Instant::now();
    let id_large = find_first_id(&large_data).expect("Failed to parse large");
    let duration_large = start_large.elapsed();
    assert_eq!(id_large, 999);

    println!("Short JSON parse time: {duration_short:?}");
    println!("Large JSON parse time: {duration_large:?}");

    let found = find_first_id("{}");
    assert!(found.is_none());

    let found = find_first_id("");
    assert!(found.is_none());

    let false_id = r#"{"some_key": "this value contains \"id\": 123", "id": 456}"#;
    let found = find_first_id(false_id).unwrap();
    assert_eq!(found, 456);
}
