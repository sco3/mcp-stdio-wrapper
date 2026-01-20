mod config;
mod config_defaults;
mod config_from_cli;
mod config_from_env;
mod logger;
mod mcp_workers;
mod stdio_reader;
mod stdio_writer;
mod streamer;
mod post_result;

use crate::config::Config;
use crate::logger::init_logger;
use crate::stdio_reader::spawn_reader;
//use flume::bounded;
use crate::mcp_workers::spawn_workers;
use crate::stdio_writer::spawn_writer;


use mcp_stdio_wrapper::streamer::McpStreamClient;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    init_logger();
    let config = Config::from_cli();
    info!("{config:?}");

    info!("Start");

    let mcp_client = McpStreamClient::new(config.mcp_server_url);
    debug!("Mcp client: {mcp_client:?}");

    // (Reader -> Worker)
    let (reader_tx, reader_rx) = flume::unbounded::<String>();
    // (Worker -> Writer)
    let (writer_tx, writer_rx) = flume::unbounded::<String>();

    spawn_reader(reader_tx);
    spawn_workers(config.concurrency, &reader_rx, &writer_tx);
    let exit = spawn_writer(writer_rx);

    let _ = exit.await;

    info!("Finish");
}
