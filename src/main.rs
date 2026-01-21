use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::mcp_workers::spawn_workers;
use mcp_stdio_wrapper::stdio_reader::spawn_reader;
use mcp_stdio_wrapper::stdio_writer::spawn_writer;
use std::sync::Arc;

use mcp_stdio_wrapper::streamer::McpStreamClient;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    let config = Config::from_cli();
    init_logger(Some(&config.mcp_wrapper_log_level));
    info!("{config:?}");

    info!("Start");

    let mcp_client = Arc::new(McpStreamClient::new(config.mcp_server_url));
    debug!("Mcp client: {mcp_client:?}");

    // (Reader -> Worker)
    let (reader_tx, reader_rx) = flume::unbounded::<String>();
    // (Worker -> Writer)
    let (writer_tx, writer_rx) = flume::unbounded::<String>();

    spawn_reader(reader_tx);

    // create several workers (limit with concurrenty parameter)
    spawn_workers(config.concurrency, &mcp_client, &reader_rx, &writer_tx);
    let exit = spawn_writer(writer_rx);

    let _ = exit.await;

    info!("Finish");
}
