mod config;
mod config_defaults;
mod config_from_cli;
mod config_from_env;
mod logger;
mod stdio_reader;
mod stdio_writer;

use crate::config::Config;
use crate::logger::init_logger;
use crate::stdio_reader::spawn_reader;
//use flume::bounded;
use crate::stdio_writer::spawn_writer;
use tracing::info;

#[tokio::main]
async fn main() {
    init_logger();
    let config = Config::from_cli();
    info!("{config:?}");

    info!("Start");

    let stdio_rx = spawn_reader();
    let signal = spawn_writer(stdio_rx);

    let _ = signal.await;

    info!("Finish");
}
