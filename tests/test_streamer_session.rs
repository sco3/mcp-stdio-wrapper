use http::Response as HttpResponse;
use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::config_defaults::*;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use reqwest::header::HeaderValue;

#[tokio::test]
pub async fn test_streamer_bad_chars() {
    // 1. Create bytes that are NOT valid UTF-8 (0xFF is a classic example)
    let invalid_bytes = vec![0x61, 0x62, 0xFF, 0xFE];
    let invalid_header_val = HeaderValue::from_bytes(&invalid_bytes).unwrap();

    // 2. Build a standard http::Response
    let response_builder = HttpResponse::builder().status(200);

    // Use your SID constant (e.g., "mcp-session-id")
    let http_res = response_builder
        .header("mcp-session-id", invalid_header_val)
        .body("")
        .unwrap();

    // 3. Convert to reqwest::Response
    let response: reqwest::Response = http_res.into();

    // 4. Execute the test
    let config = Config {
        mcp_server_url: "file:///tmp".to_string(),
        mcp_auth: default_mcp_auth(),
        concurrency: default_concurrency(),
        mcp_wrapper_log_level: default_mcp_wrapper_log_level(),
        mcp_tool_call_timeout: default_mcp_tool_call_timeout(),
    };

    let client = McpStreamClient::try_new(config);

    if let Ok(client) = client {
        let result: Option<String> = client.process_session_id(&response).await;
        assert!(
            result.is_none(),
            "Should return None for invalid UTF-8 headers"
        );
    } else {
        assert!(false);
    }
}
