use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::streamer::McpStreamClient;

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
#[should_panic(expected = "Failed to open cert file")]
pub async fn test_streamer_cert() {
    let config = Config::from_cli([
        "test",
        "--url",
        "https://localhost:3000/mcp",
        "--tls-cert",
        "?",
    ]);

    let _cli = McpStreamClient::try_new(config).unwrap();
}
