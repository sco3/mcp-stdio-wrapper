
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
cargo llvm-cov --html

firefox /home/dz/prj/mcp-stdio-wrapper/target/llvm-cov/html/index.html