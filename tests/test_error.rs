use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::streamer_error::mcp_error;

#[cfg(test)]
#[test]
/// test id parsing
/// # Errors
/// errors mean test failure
fn test_error() {
    init_logger(Some("debug"), None);

    let worker: usize = 1;

    let json = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
    mcp_error(&worker, json, "error");

    let json = r#"{"jsonrpc":"2.0","id":"id:2","method":"tools/list"}"#;
    mcp_error(&worker, json, "error");

    let json = "";
    mcp_error(&worker, json, "error");
}
