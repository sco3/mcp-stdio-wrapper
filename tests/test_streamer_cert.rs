use base64::{engine::general_purpose, Engine as _};
use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use nom::AsBytes;
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
// Note: reqwest::Certificate::from_pem() is very lenient and accepts many formats
// The actual "Invalid PEM" error at line 29-35 is hard to trigger in practice
// because from_pem() will parse almost anything and defer errors to the client builder
// This test covers the error path that occurs when PEM parsing succeeds but
// the certificate is invalid when added to the client builder (line 36-42)
#[tokio::test]
#[should_panic(expected = "Cannot create http client")]
pub async fn test_streamer_cert_invalid_certificate() {
    let temp_dir = tempfile::tempdir().unwrap();
    let log_file = temp_dir.path().join("invalid-cert.pem");
    let log_path = log_file.to_str().unwrap();
    let mut file = File::create(&log_file).unwrap();
    const BLOB: &[&str] = &[
        "LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSUNsamNDQlg0Q0NRQ0t6OFpy",
        "ISEhSU5WQUxJRCEhIUJBU0U2NCEhIURBVEEhISFIRQpMLS0tLS1FTkQgQ0VSVElG",
        "SUNBVEUtLS0tLQo=",
    ];
    let broken = general_purpose::STANDARD
        .decode(BLOB.join(""))
        .expect("Failed to decode test data");

    let _ = file.write_all(broken.as_bytes());

    let config = Config::from_cli([
        "test",
        "--url",
        "https://localhost:3000/mcp",
        "--tls-cert",
        log_path,
    ]);

    let _cli = McpStreamClient::try_new(config).unwrap();
}
