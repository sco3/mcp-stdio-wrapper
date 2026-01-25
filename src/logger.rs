use crate::config_defaults::default_mcp_wrapper_log_level;
use std::io::stderr;

use std::sync::{Mutex, Once};
use tracing::debug;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

static INIT: Once = Once::new();
static GUARD: Mutex<Option<WorkerGuard>> = Mutex::new(None);
fn init_logger_once(log_level: Option<&str>) {
    let def_level = default_mcp_wrapper_log_level();

    let level = log_level.unwrap_or(&def_level);
    if level == "off" {
        return;
    }

    let (non_blocking, guard) = non_blocking(stderr());

    let filter = EnvFilter::try_from_default_env() //
        .unwrap_or_else(|_| EnvFilter::new(level));

    let layer = fmt::layer() //
        .with_ansi(false)
        .with_writer(non_blocking);

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .try_init();

    debug!("Logger initialized with level: {level} to stderr");
    if let Ok(mut guard_lock) = GUARD.lock() {
        *guard_lock = Some(guard);
    }
}

/// initializes logger
pub fn init_logger(log_level: Option<&str>) {
    INIT.call_once(|| init_logger_once(log_level));
}
