use mcp_stdio_wrapper::logger::init_logger;
use mcp_stdio_wrapper::stdio_reader::spawn_reader;
use tokio_test::io::Builder;
///
/// # Errors
/// * test fails
/// # Panics
/// * test fails
#[tokio::test]
pub async fn test_reader() -> Result<(), Box<dyn std::error::Error>> {
    let data = "test";
    init_logger(Some("debug"));
    let (tx, rx) = flume::unbounded::<String>();

    let stdio = Builder::new().read(data.as_bytes()).build();
    let _reader = spawn_reader(tx, stdio);

    let line = rx.recv_async().await?;
    assert_eq!(line, data);

    Ok(())
}
