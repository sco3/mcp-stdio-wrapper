use crate::mcp_workers::spawn_workers;
use crate::stdio_reader::spawn_reader;
use crate::stdio_writer::spawn_writer;
use crate::streamer::McpStreamClient;
use std::sync::Arc;
use tokio::io::AsyncWrite;
use tracing::debug;

pub async fn main_loop<W>(concurrency: usize, client: McpStreamClient, writer: W)
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    let mcp_client = Arc::new(client);
    debug!("Mcp client: {mcp_client:?}");

    // (Reader -> Worker)
    let (reader_tx, reader_rx) = flume::unbounded::<String>();
    // (Worker -> Writer)
    let (writer_tx, writer_rx) = flume::unbounded::<String>();

    spawn_reader(reader_tx);

    // create several workers (limit with concurrenty parameter)
    spawn_workers(concurrency, &mcp_client, &reader_rx, writer_tx);

    let exit = spawn_writer(writer_rx, writer);

    let _ = exit.await;
}
