use mcp_stdio_wrapper::main_init::init_main;
use mcp_stdio_wrapper::main_loop::main_loop;
use tokio::io::{stdin, stdout};

#[tokio::main]
async fn main() {
    let config = init_main(std::env::args());
    main_loop(config, stdin(), stdout()).await;
}
