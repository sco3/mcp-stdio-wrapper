use mcp_stdio_wrapper::streamer::McpStreamClient;

const URL: &'static str = "http://localhost:8000/mcp";
const INIT: &'static str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"demo","version":"0.0.1"}}}"#;

#[tokio::main]
async fn main() {
    let client = McpStreamClient::new(URL.to_owned());
    println!("Start {client:?}");

    let (tx, rx) = flume::unbounded::<String>();

    let result = client.stream_post(INIT.to_string(), tx).await;
    match result {
        Ok(post_data) => {
            println!("Post {post_data:?}");
            while let Ok(event_data) = rx.recv_async().await {
                println!("{event_data}");
            }
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}
