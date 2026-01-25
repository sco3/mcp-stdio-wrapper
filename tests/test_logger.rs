use mcp_stdio_wrapper::logger::init_logger;
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
