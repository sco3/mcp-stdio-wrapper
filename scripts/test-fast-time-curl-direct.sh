#!/usr/bin/env -S bash

set -ueo pipefail

PORT="${PORT:-8080}"

netstat -ltnp 2>/dev/null | grep -q ":${PORT} " && {
	echo "Port ${PORT} is busy, pkill -9 fast-time-server may help"
	exit 1
}

#MCPGATEWAY_BEARER_TOKEN="$(uvx --from mcp-contextforge-gateway python -m mcpgateway.utils.create_jwt_token --username admin@example.com --exp 10080 --secret my-test-key)"
MCPGATEWAY_BEARER_TOKEN="some-string"

fast-time-server -transport=http -port=$PORT -auth-token="$MCPGATEWAY_BEARER_TOKEN" 2>~/tmp/fast-time-server.log &
PID="$!"

trap 'kill "$PID" 2>/dev/null' EXIT

URL="http://localhost:${PORT}/"

INIT='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"demo","version":"0.0.1"}}}'

NOTIFY='{"jsonrpc": "2.0","method": "notifications/initialized"}'
LIST='{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
CALL='{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_system_time","arguments":{"timezone":"UTC"}}}'

HEADERS=(
	-H "Authorization: Bearer $MCPGATEWAY_BEARER_TOKEN"
	-H "Content-Type: application/json"
	-H "Accept: application/json, application/x-ndjson, text/event-stream"
)

TEMP_TXT=$(mktemp --suffix=.txt)
trap 'rm -f "$TEMP_TXT"' EXIT

curl -N "$URL" "${HEADERS[@]}" -d "$INIT" -D $TEMP_TXT
SESSION_ID=$(grep -i "mcp-session-id" $TEMP_TXT | cut -d' ' -f2 | tr -d '\r')
# echo "Session ID: $SESSION_ID"
HEADERS+=(-H "Mcp-Session-Id: $SESSION_ID")

printf "\n---\n"
curl -N "$URL" "${HEADERS[@]}" -d "$NOTIFY"
printf "\n---\n"
curl -N "$URL" "${HEADERS[@]}" -d "$LIST"
printf "\n---\n"
curl -N "$URL" "${HEADERS[@]}" -d "$CALL"

kill -9 $PID
