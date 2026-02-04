use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use std::fs::File;
use std::io::Write;

/// Tests the streamer post failure case.
/// # Errors
/// Returns an error if the mock server setup fails.
/// # Panics
/// Panics if the mock server does not receive the expected request.
#[tokio::test]
#[should_panic(expected = "Failed to read cert file")]
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
#[tokio::test]
#[should_panic]
pub async fn test_streamer_cert_bad() {
    let temp_dir = tempfile::tempdir().unwrap();
    let log_file = temp_dir.path().join("bad-pem.log");
    let log_path = log_file.to_str().unwrap();
    let mut file = File::create(&log_file).unwrap();
    let broken = b"-----BEGIN CERTIFICATE-----\n\
This is clearly not base64 data and will fail DER decoding\n\
-----END CERTIFICATE";

    let _ = file.write_all(broken);

    let config = Config::from_cli([
        "test",
        "--url",
        "https://localhost:3000/mcp",
        "--tls-cert",
        log_path,
    ]);

    let _cli = McpStreamClient::try_new(config).unwrap();
}
