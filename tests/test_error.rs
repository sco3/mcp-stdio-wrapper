use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::streamer_error::mcp_error;

#[cfg(test)]
#[tokio::test]
/// test id parsing
/// # Errors
/// errors mean test failure
async fn test_error() {
    init_logger(Some("debug"), None);

    let (tx, rx) = flume::unbounded();

    let worker: usize = 1;

    let json = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#;
    mcp_error(&worker, json, "error", &tx).await;

    let json = r#"{"jsonrpc":"2.0","id":"id:2","method":"tools/list"}"#;
    mcp_error(&worker, json, "error", &tx).await;

    let json = "";
    mcp_error(&worker, json, "error", &tx).await;
}
