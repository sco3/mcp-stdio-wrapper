#!/usr/bin/env -S bash

set -ueo pipefail

# export RUST_LOG=debug
# update exe to required executable
EXE="mcp_stdio_wrapper --url http://localhost:8000/mcp"

# commands

INIT='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0"}}}'
NOTIFY='{"jsonrpc":"2.0","method":"notifications/initialized"}'
LIST='{"jsonrpc":"2.0","id":2,"method":"tools/list"}'

CALL='{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "say_hello",
    "arguments": {},
    "_meta": {
      "progressToken": 0
    }
  }
}'

(
  echo "$INIT"
  sleep 0.1
  echo "$NOTIFY"
  sleep 0.1
  echo "$LIST"
  sleep 0.1
  echo "$CALL" | yq -o json -M -I 0
  sleep 0.1
) | $EXE
