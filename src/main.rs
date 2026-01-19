mod config;
mod config_from_env;
mod logger;
mod stdio_reader;
mod config_defaults;

use crate::config::Config;
use crate::logger::init_logger;
use flume::bounded;
use tracing::info;

fn main() {
    init_logger();
    info!("Start");
    let config = Config::from_env();
    info!("{config:?}");

    let (_tx, _rx) = bounded::<String>(config.concurrency);
    info!("Finish");
}
