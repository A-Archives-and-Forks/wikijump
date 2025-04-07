#!/bin/sh
set -e

# Have DEEPWELL generate the Caddyfile
wikijump-generate-caddyfile

# Have Caddy install it
curl -f http://localhost:2019/load \
	-X POST \
	-H 'Content-Type: text/caddyfile' \
	--data-binary @/tmp/Caddyfile

# Delete temporary file
rm -f /tmp/Caddyfile
