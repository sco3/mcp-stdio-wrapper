#!/usr/bin/env -S bash

set -ueo pipefail

source ./token-from-file.sh

./target/release/bench \
	bench.toml \
	-p \
	-i ${ITERS:=4} \
	-- \
	uv \
	--project $HOME/prj/mcp-context-forge run -m mcpgateway.wrapper --url "http://localhost:8080/servers/9779b6698cbd4b4995ee04a4fab38737/mcp" \
	--auth "$AUTH" \
	--log-level off
