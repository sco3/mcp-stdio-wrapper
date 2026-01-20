use mcp_stdio_wrapper::streamer::McpStreamClient;

#[cfg(feature = "manual")]
#[tokio::test]

async fn test_streamer() {
    let client = McpStreamClient::new("http://localhost:8000/mcp".to_owned());
    println!("Start {client:?}");
}
