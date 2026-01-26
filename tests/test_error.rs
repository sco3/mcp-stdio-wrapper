use flume::Receiver;
use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::streamer_error::mcp_error;

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
        r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"error1"},"id":"1"}"#,
    )
    .await;

    let json = r#"{"jsonrpc":"2.0","id":"id_2","method":"tools/list"}"#;
    mcp_error(&worker, json, "error2", &tx).await;
    verify(
        &rx,
        r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"error2"},"id":"id_2"}"#,
    )
    .await;

    let json = "";
    mcp_error(&worker, json, "error3", &tx).await;
    verify(
        &rx,
        r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"error3"},"id":"<unknown id>"}"#,
    )
    .await;
    Ok(())
}

async fn verify(rx: &Receiver<String>, expected_err: &str) {
    let msg = rx.recv_async().await.expect("receiving error");
    println!("{msg}");
    assert_eq!(msg, expected_err.to_string());
}
