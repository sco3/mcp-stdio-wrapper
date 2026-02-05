use mcp_stdio_wrapper::main_init::init_main;
use mcp_stdio_wrapper::main_loop::main_loop;
use tokio::io::{stdin, stdout, BufReader, BufWriter};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[tokio::main]
async fn main() {
    let config = init_main(std::env::args());
    let reader = BufReader::with_capacity(256 * 1024, stdin());
    let writer = BufWriter::with_capacity(512 * 1024, stdout());
    main_loop(config, reader, writer).await;
}
