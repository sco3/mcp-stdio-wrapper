use std::io;
use tracing_subscriber;

/// initializes logger
pub (crate) fn init_logger() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr) // Redirects output to stderr
        .init();

    tracing::info!("Logger output set to stderr");
}
