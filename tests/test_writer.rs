use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::stdio_writer::spawn_writer;
use tokio_test::io::Builder;
///
/// # Errors
/// * test fails
/// # Panics
/// * test fails
#[tokio::test]
pub async fn test_writer() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(Some("debug"));
    let (tx, rx) = flume::unbounded::<String>();

    let out = Builder::new().write(b"test\n").build();

    let writer = spawn_writer(rx, out);
    tx.send_async("test".to_string()).await?;
    drop(tx);
    writer.await?;
    Ok(())
}
