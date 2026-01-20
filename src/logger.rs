use std::io;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// initializes logger to debug by default
pub(crate) fn init_logger() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(io::stderr))
        .init();

    tracing::debug!("Logger initialized with DEBUG level on stderr");
}
