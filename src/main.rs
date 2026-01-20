mod config;
mod config_defaults;
mod config_from_cli;
mod config_from_env;
mod logger;
mod stdio_reader;

use crate::config::Config;
use crate::logger::init_logger;
use crate::stdio_reader::spawn_reader;
//use flume::bounded;
use tracing::{debug, info};
#[tokio::main]
async fn main() {
    init_logger();
    info!("Start");
    let config = Config::from_cli();
    info!("{config:?}");

    //let (_tx, _rx) = bounded::<String>(config.concurrency);

    let stdio_rx = spawn_reader();
    while let Ok(line) = stdio_rx.recv_async().await {
        debug!("Received: {}", line);

        if line == "quit" {
            break;
        }
    }
    info!("Finish");
}
