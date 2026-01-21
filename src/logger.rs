use std::io;
use tracing::debug;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// initializes logger
pub fn init_logger(log_level: Option<&str>) {
    let log_level = log_level.unwrap_or("info");

    let filter = EnvFilter::try_from_default_env() //
        .unwrap_or_else(|_| EnvFilter::new(log_level)); // use config

    let writer = io::stderr;
    let dest = "stderr";

    let layer = fmt::layer()
        .with_ansi(false) // b/w
        .with_writer(writer);

    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();

    debug!("Logger initialized with log level: {log_level} to {dest}",);
}
