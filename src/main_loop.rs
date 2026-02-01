use crate::mcp_workers::spawn_workers;
use crate::stdio_reader::spawn_reader;
use crate::stdio_writer::spawn_writer;
use crate::streamer::McpStreamClient;
use std::sync::Arc;
use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::debug;

pub async fn main_loop<R, W>(concurrency: usize, client: McpStreamClient, reader: R, writer: W)
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    let mcp_client = Arc::new(client);
    debug!("Mcp client: {mcp_client:?}");

    // (Reader -> Worker)
    let (reader_tx, reader_rx) = flume::unbounded::<Bytes>();
    // (Worker -> Writer)
    let (writer_tx, writer_rx) = flume::unbounded::<Bytes>();

    spawn_reader(reader_tx, reader);

    // create several workers (limit with concurrenty parameter)
    spawn_workers(concurrency, &mcp_client, &reader_rx, writer_tx);

    let exit = spawn_writer(writer_rx, writer);

    let _ = exit.await;
}
