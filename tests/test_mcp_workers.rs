use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::config_defaults::*;
use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::mcp_workers::*;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use mockito::Server;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// # Errors
/// may happen on test failure
/// # Panics
/// may happen on test failure
#[tokio::test]
pub async fn test_mcp_workers() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(Some("debug"));
    let mut server = Server::new_async().await;

    let mock_init = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("mcp-session-id", "session-42")
        .with_body(r#"data: {"init":"ok"}"#)
        .create_async()
        .await;

    let config = Config {
        mcp_server_url: server.url(),
        mcp_auth: default_mcp_auth(),
        concurrency: default_concurrency(),
        mcp_wrapper_log_level: default_mcp_wrapper_log_level(),
        mcp_tool_call_timeout: default_mcp_tool_call_timeout(),
    };

    let client = McpStreamClient::try_new(config)?;
    let (tx_in, rx_in) = flume::unbounded();
    let (tx_out, _rx_out) = flume::unbounded();

    spawn_workers(
        //
        2,
        &Arc::new(client),
        &rx_in,
        tx_out,
    );
    tx_in.send_async(String::from(r#"{"id":1}"#)).await?;

    sleep(Duration::from_millis(100)).await;
    mock_init.assert_async().await;

    Ok(())
}
