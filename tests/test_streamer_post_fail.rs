use mockito::Server;
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
}
