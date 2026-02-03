use mcp_stdio_wrapper::logger::{flush_logger, init_logger};
use tracing::info;
/// # Panics
/// * test failures

#[tokio::test]
pub async fn test_logger_init_off() {
    init_logger(Some("off"), None);
}
/// # Panics
/// * test failures

#[tokio::test]
pub async fn test_logger_init_info() {
    init_logger(Some("info"), None);
}
#[tokio::test]
async fn test_logger_init_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let log_file = temp_dir.path().join("out.log");
    let log_path = log_file.to_str().unwrap();

    init_logger(Some("info"), Some(log_path));
    info!("hello");
    info!("hello");
    flush_logger();
}
