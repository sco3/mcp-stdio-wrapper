use mcp_stdio_wrapper::config::Config;
use mcp_stdio_wrapper::main_init::init_main;
use mcp_stdio_wrapper::main_loop::main_loop;
use mockito::Server;
use std::sync::Arc;

#[test]
fn test_init_main() {
    // Simulate command line arguments
    let fake_args = ["wrapper", "--url", "file:///tmp"];
    let config = init_main(fake_args.iter());
    assert_eq!(config.mcp_server_url, "file:///tmp");
}
