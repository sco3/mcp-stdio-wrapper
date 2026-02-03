use http::Response as HttpResponse;
use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::config_defaults::*;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use reqwest::header::HeaderValue;

/// # Panics
/// * test fails
/// # Errors
/// * test setup fails
#[tokio::test]
pub async fn test_streamer_bad_chars() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create bytes that are NOT valid UTF-8 (0xFF is a classic example)
    let invalid_bytes = vec![0x61, 0x62, 0xFF, 0xFE];
    let invalid_header_val = HeaderValue::from_bytes(&invalid_bytes).unwrap();
    let response_builder = HttpResponse::builder().status(200);
    let http_res = response_builder
        .header("mcp-session-id", invalid_header_val)
        .body("")
        .unwrap();
    let response: reqwest::Response = http_res.into();

    let config = Config {
        mcp_server_url: "file:///tmp".to_string(),
        mcp_auth: default_mcp_auth(),
        concurrency: default_concurrency(),
        mcp_wrapper_log_level: default_mcp_wrapper_log_level(),
        mcp_wrapper_log_file: None,
        mcp_tool_call_timeout: default_mcp_tool_call_timeout(),
        tls_cert: None,
    };

    let client = McpStreamClient::try_new(config)?;

    let result: Option<String> = client.process_session_id(&response).await;
    assert!(
        result.is_none(),
        "Should return None for invalid UTF-8 headers"
    );

    Ok(())
}
