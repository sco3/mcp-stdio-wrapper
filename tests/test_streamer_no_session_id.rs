use http::Response as HttpResponse;
use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::config_defaults::*;
use mcp_stdio_wrapper::streamer::McpStreamClient;

/// # Panics
/// * test fails
/// # Errors
/// * test setup fails
#[tokio::test]
pub async fn test_streamer_no_session_id() -> Result<(), Box<dyn std::error::Error>> {
    let response_builder = HttpResponse::builder().status(200);

    let http_res = response_builder.body("")?;

    let response: reqwest::Response = http_res.into();

    let config = Config {
        mcp_server_url: "file:///tmp".to_string(),
        mcp_auth: default_mcp_auth(),
        concurrency: default_concurrency(),
        mcp_wrapper_log_level: default_mcp_wrapper_log_level(),
        mcp_wrapper_log_file: None,
        mcp_tool_call_timeout: default_mcp_tool_call_timeout(),
        mcp_content_type: "application/json".to_string(),
    };

    let client = McpStreamClient::try_new(config)?;

    let result: Option<String> = client.process_session_id(&response).await;
    assert!(
        result.is_none(),
        "Should return None when session id not found"
    );

    Ok(())
}
