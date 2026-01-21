use std::env;
use std::fs::File;
use std::io;
use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter, Layer};

/// initializes logger to debug by default
pub fn init_logger() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let filter = tracing_subscriber::EnvFilter::from_default_env();

    let layer = if let Ok(path) = env::var("MCP_LOG_FILE") {
        let file = File::create(path).expect("Failed to create log file");
        fmt::layer().with_ansi(false).with_writer(file).boxed()
    } else {
        fmt::layer()
            .with_ansi(false)
            .with_writer(std::io::stderr)
            .boxed()
    };

    registry().with(filter).with(layer).init();
    tracing::debug!("Logger initialized");
}
