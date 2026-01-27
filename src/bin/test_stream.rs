//use clap::Parser;
use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use std::fs::File;
use std::io::Read;
use tracing::{debug, error};

const URL: &str = "http://localhost:8080/servers/9779b6698cbd4b4995ee04a4fab38737/mcp";
pub const INIT: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"wrapper","version":"0.0.1"}}}"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(Some("debug"), None);

    let mut file = File::open("/home/dz/.local/cf-token.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config = Config {
        mcp_wrapper_log_file: None,
        mcp_tool_call_timeout: 1,
        mcp_wrapper_log_level: "debug".to_string(),
        concurrency: 1,
        mcp_server_url: URL.to_string(),
        mcp_auth: format!("Bearer {content}"),
    };

    match McpStreamClient::try_new(config) {
        Ok(client) => {
            debug!("Start {client:?}");

            let result = client.stream_post(INIT).await;
            match result {
                Ok(post_data) => {
                    debug!("Post {post_data:?}");
                }
                Err(e) => {
                    error!("Error: {e}");
                }
            }
        }
        Err(e) => {
            error!("Error: {e}");
        }
    }
    Ok(())
}
