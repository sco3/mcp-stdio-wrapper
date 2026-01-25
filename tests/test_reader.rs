use mcp_stdio_wrapper::stdio_reader::spawn_reader;
#[tokio::test]
///
/// # Errors
///
/// Returns an error if reading from the channel fails.
///
/// # Panics
///
/// Panics if the received line does not match the expected data.
async fn test_reader_break_coverage() {
    let (tx, rx) = flume::unbounded::<String>();

    // We provide two lines.
    // Line 1 will be sent successfully.
    // Line 2 will trigger the send error because we drop(rx) in between.
    let stdio = tokio_test::io::Builder::new()
        .read(b"line1\n")
        .wait(std::time::Duration::from_millis(10)) // Give us time to drop
        .read(b"line2\n")
        .build();

    let handle = spawn_reader(tx, stdio);

    // 1. Receive the first line
    let first = rx.recv_async().await.expect("Should receive line1");
    assert_eq!(first, "line1");

    // 2. Kill the receiver
    drop(rx);

    // 3. Wait for the background task to try to send "line2" and fail
    let _ = handle.await;

    // After awaiting the handle, the code MUST have executed the break
    // because that is the only way the task can finish when data is still
    // available in the reader.
}
