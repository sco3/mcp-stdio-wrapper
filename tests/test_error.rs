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

    let json = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#;
    mcp_error(&worker, json, "error1", &tx).await;
    verify(
        &rx,
        &json!({
            "jsonrpc":"2.0","error":{"code":ErrorCode::InternalError,"message":"error1"},
            "id":1
        }),
    )
    .await;

    let json = r#"{"jsonrpc":"2.0","id":"id_2","method":"tools/list"}"#;
    mcp_error(&worker, json, "error2", &tx).await;
    verify(
        &rx,
        &json!({
            "jsonrpc":"2.0","error":{"code":ErrorCode::InternalError,"message":"error2"},
            "id":"id_2"
        }),
    )
    .await;

    let json = "";
    mcp_error(&worker, json, "error3", &tx).await;
    verify(
        &rx,
        &json!({
            "jsonrpc":"2.0","error":{"code":ErrorCode::InternalError,"message":"error3"},
            "id":null
        }),
    )
    .await;
    Ok(())
}

async fn verify(rx: &Receiver<String>, expected: &Value) {
    let msg = rx.recv_async().await.expect("receiving error");
    let actual = serde_json::from_str::<Value>(&msg).expect("deserializing error");
    println!("{actual}");
    assert_eq!(actual, *expected);
}
