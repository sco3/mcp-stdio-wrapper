use mcp_stdio_wrapper::config::Config;

#[tokio::test]
pub async fn test_config() {
    let _config = Config::from_env();
    let args = vec![
        //
        "wrapper", "--url", "url",
    ]
    .into_iter()
    .map(|s| s.to_string());
    let _config = Config::from_cli(args);
}
