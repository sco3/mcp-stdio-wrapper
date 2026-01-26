use flume::Receiver;
use jsonrpc_core::ErrorCode;
use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::streamer_error::mcp_error;
use serde_json::{json, Value};

#[cfg(test)]
#[tokio::test]
/// test id parsing
/// # Errors
/// returns error when test fails
/// # Panics
/// code panics when test fails
async fn test_error() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(Some("debug"), None);

    let (tx, rx) = flume::unbounded();

    let worker: usize = 1;

    // Define test cases as (input_json, error_message, expected_output) tuples
    let test_cases = vec![
        (
            json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}),
            "error1",
            json!({
                "jsonrpc":"2.0",
                "error":{"code":ErrorCode::InternalError,"message":"error1"},
                "id":1
            }),
        ),
        (
            json!({"jsonrpc":"2.0","id":"id_2","method":"tools/list"}),
            "error2",
            json!({
                "jsonrpc":"2.0",
                "error":{"code":ErrorCode::InternalError,"message":"error2"},
                "id":"id_2"
            }),
        ),
        (
            json!(""),
            "error3",
            json!({
                "jsonrpc":"2.0",
                "error":{"code":ErrorCode::InternalError,"message":"error3"},
                "id":null
            }),
        ),
    ];

    // Run tests in a loop
    for (input_json, error_msg, expected) in test_cases {
        let s = input_json.to_string();
        mcp_error(&worker, &s, error_msg, &tx).await;
        verify(&rx, expected).await;
    }
    Ok(())
}

async fn verify(rx: &Receiver<String>, expected: Value) {
    let msg = rx.recv_async().await.expect("receiving error");
    let actual = serde_json::from_str::<Value>(&msg).expect("deserializing error");
    println!("{actual}");
    assert_eq!(actual, expected);
}
