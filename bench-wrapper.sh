#!/usr/bin/env -S bash

set -ueo pipefail

source ./token-from-file.sh

./target/release/bench \
	bench.toml \
	mcp_stdio_wrapper \
	--url "http://localhost:8080/servers/9779b6698cbd4b4995ee04a4fab38737/mcp" \
	--auth "$AUTH" \
	--log-level off \
#	--log-file out.log
