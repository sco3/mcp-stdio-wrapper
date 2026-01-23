use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::config_defaults::*;
use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::mcp_workers::*;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use mockito::Server;
use std::sync::Arc;

/// Tests that `spawn_workers` correctly processes a message by sending it to a mock server
/// and forwarding the response.
/// # Errors
/// Returns an error if channel operations fail or if the test times out.
/// # Panics
/// Panics if an assertion fails.
#[tokio::test]
pub async fn test_mcp_workers() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(Some("debug"));
    let mut server = Server::new_async().await;

    let expected = "ok";

    let mock_init = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("mcp-session-id", "session-42")
        .with_body(format!("data: {expected}"))
        .create_async()
        .await;

    let config = Config::from_cli(["test".to_string(), "--url".to_string(), server.url()]);

    let client = McpStreamClient::try_new(config)?;
    let (tx_in, rx_in) = flume::unbounded();
    let (tx_out, rx_out) = flume::unbounded();

    spawn_workers(default_concurrency(), &Arc::new(client), &rx_in, tx_out);
    tx_in.send_async(String::from("init")).await?;

    let out = rx_out.recv_async().await?;

    assert_eq!(expected, out);
    mock_init.assert_async().await;

    Ok(())
}
