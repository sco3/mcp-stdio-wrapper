mod logger;
mod stdio_reader;

use crate::logger::init_logger;
use flume::bounded;
use tracing::info;

fn main() {
    init_logger();
    info!("Start");
    let num_workers = 10; //TODO dz move to config 
    let (_tx, _rx) = bounded::<String>(num_workers);
    info!("Finish");
}
