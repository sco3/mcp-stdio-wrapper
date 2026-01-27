use mcp_stdio_wrapper::mcp_workers_write::write_output;
use mcp_stdio_wrapper::post_result::PostResult;
use tracing_test::traced_test;

#[tokio::test]
async fn test_write() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = flume::unbounded::<String>();
    let tests = vec![("asdf", "asdf", false), ("data: asdf", "asdf", true)];

    for (out, expected, sse) in tests {
        let res = PostResult {
            sse,
            out: out.to_string(),
            session_id: Some("1".to_string()),
        };
        write_output(1, &tx, res).await;

        let actual = rx.recv_async().await?;
        println!("{out}");
        assert_eq!(expected, actual);
    }

    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_write_fail() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = flume::unbounded::<String>();
    let res = PostResult {
        sse: false,
        out: "asdf".to_string(),
        session_id: Some("1".to_string()),
    };
    drop(rx);
    write_output(1, &tx, res).await;
    //assert!(logs_contain("ERROR"));
    Ok(())
}
