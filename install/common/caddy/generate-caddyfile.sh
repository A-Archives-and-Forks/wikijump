#!/bin/sh
set -e

body='
{
	"jsonrpc": "2.0",
	"method": "caddyfile",
	"id": 0,
	"params": {
		"debug": true,
		"local": true,
		"framerail_host": "framerail:3000",
		"wws_host": "wws:7000"
	}
}
'

# Send DEEPWELL request
curl -f http://deepwell:2747/jsonrpc \
	-X POST \
	--json "$body" \
		> /tmp/deepwell.json

# Determine if it's an error
error="$(jq .error /tmp/deepwell.json)"
if [ "$error" != null ]; then
	cat /tmp/deepwell.json
	exit 1
fi

# Call was a success, extract the Caddyfile
jq -r .result /tmp/deepwell.json > /tmp/Caddyfile
rm /tmp/deepwell.json
