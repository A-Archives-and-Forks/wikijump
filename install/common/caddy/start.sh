#!/bin/sh

# It'll fork off and the child process will be tracked
echo 'Starting cron...'
crond

# Then we need to get the Caddyfile
echo 'Generating Caddyfile...'
wikijump-generate-caddyfile
mv /tmp/Caddyfile /etc/caddy/Caddyfile

echo 'Starting caddy...'
exec caddy run --config /etc/caddy/Caddyfile --adapter caddyfile
