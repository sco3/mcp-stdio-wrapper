use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::config_defaults::*;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use mockito::Server;

const INIT: &str = r#"{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"prompts":{},"resources":{},"tools":{}},"serverInfo":{"name":"rmcp","version":"0.13.0"},"instructions":"This server provides counter tools and prompts. Tools: increment, decrement, get_value, say_hello, echo, sum. Prompts: example_prompt (takes a message), counter_analysis (analyzes counter state with a goal)."}}"#;
const INIT_OUT: &str = r#"data:
id: 0
retry: 3000

data: {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"prompts":{},"resources":{},"tools":{}},"serverInfo":{"name":"rmcp","version":"0.13.0"},"instructions":"This server provides counter tools and prompts. Tools: increment, decrement, get_value, say_hello, echo, sum. Prompts: example_prompt (takes a message), counter_analysis (analyzes counter state with a goal)."}}
"#;
const NOTIFY: &str = r#"{"jsonrpc":"2.0","method": "notifications/initialized"}"#;

/// Tests the streamer post failure case.
///
/// # Errors
///
/// Returns an error if the mock server setup fails.
///
/// # Panics
///
/// Panics if the mock server does not receive the expected request.
#[tokio::test]
pub async fn test_streamer_post() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new_async().await;
    let url = server.url();

    let mock_init = server
        .mock("POST", "/mcp/")
        .with_status(200)
        .with_header("mcp-session-id", "9cb62a01-2523-4380-964e-2e3efd1d135a")
        .with_body(INIT_OUT)
        .create_async()
        .await;

    let mock_notify = server
        .mock("POST", "/mcp/")
        .with_status(202)
        .with_body("")
        .create_async()
        .await;

    let mut mcp_auth = default_mcp_auth();
    if mcp_auth.is_empty() {
        mcp_auth = "token".to_string();
    }
    let config = Config {
        mcp_server_url: format!("{url}/mcp/"),
        mcp_auth,
        concurrency: default_concurrency(),
        mcp_wrapper_log_level: default_mcp_wrapper_log_level(),
        mcp_wrapper_log_file: None,
        mcp_tool_call_timeout: default_mcp_tool_call_timeout(),
        mcp_content_type: "application/json".to_string(),
    };

    let cli = McpStreamClient::try_new(config)?;
    println!("auth: {}", cli.is_auth());
    assert!(cli.is_auth());

    let out = cli.stream_post(INIT).await;
    mock_init.assert_async().await;
    println!("{out:?}");

    let out = cli.stream_post(NOTIFY).await;
    mock_notify.assert_async().await;

    println!("{out:?}");
    Ok(())
}
