use mcp_stdio_wrapper::logger::init_logger;

#[tokio::test]
pub async fn test_logger_init_off() {
    init_logger(Some("off"));
}

#[tokio::test]
pub async fn test_logger_init_info() {
    init_logger(Some("info"));
}
