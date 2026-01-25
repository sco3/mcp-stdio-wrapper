use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::logger::init_logger;

use mcp_stdio_wrapper::main_loop::main_loop;
use mcp_stdio_wrapper::streamer::McpStreamClient;
use tokio::io::{stdin, stdout};
use tracing::{debug, error};

#[tokio::main]
async fn main() {
    let config = Config::from_cli(std::env::args());
    init_logger(Some(&config.mcp_wrapper_log_level));
    debug!("{config:?}");

    debug!("Start");
    let concurrency = config.concurrency;
    match McpStreamClient::try_new(config) {
        Ok(client) => {
            main_loop(concurrency, client, stdin(), stdout()).await;
        }
        Err(e) => {
            error!("Error {e}");
        }
    }
    debug!("Finish");
}
