FROM caddy:alpine

# Install dependencies
RUN apk add --no-cache curl jq

# Install files
COPY ./install/local/caddy/start.sh /usr/local/bin/wikijump-start-caddy
COPY ./install/local/caddy/health-check.sh /usr/local/bin/wikijump-health-check
COPY ./install/local/caddy/generate-caddyfile.sh /usr/local/bin/wikijump-generate-caddyfile
COPY ./install/local/caddy/update-caddyfile.sh /usr/local/bin/wikijump-update-caddyfile

# Set up cron
COPY ./install/local/caddy/crontab /etc/caddy/crontab
RUN crontab -u daemon /etc/caddy/crontab && \
    rm /etc/caddy/crontab

# Run cron and caddy
CMD ["/usr/local/bin/wikijump-start-caddy"]
