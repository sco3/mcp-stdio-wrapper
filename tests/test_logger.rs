use mcp_stdio_wrapper::logger::init_logger;
/// # Panics
/// * test failures
///

const LOG_FILE: Option<&str> = Some("/tmp/test_logger.log");

#[tokio::test]
pub async fn test_logger_init_off() {
    init_logger(Some("off"), LOG_FILE);
}
/// # Panics
/// * test failures

#[tokio::test]
pub async fn test_logger_init_info() {
    init_logger(Some("info"), LOG_FILE);
}
