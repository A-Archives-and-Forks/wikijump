# Set up DNS ACME challenge providers
FROM public.ecr.aws/docker/library/caddy:builder-alpine AS builder
ARG PROVIDER=digitalocean

RUN xcaddy build --with github.com/caddy-dns/${PROVIDER}

FROM public.ecr.aws/docker/library/caddy:latest
COPY --from=builder /usr/bin/caddy /usr/bin/caddy

# Install dependencies
RUN apk add --no-cache curl jq

# Install files
COPY ./install/dev/caddy/start.sh /usr/local/bin/wikijump-start-caddy
COPY ./install/dev/caddy/health-check.sh /usr/local/bin/wikijump-health-check
COPY ./install/dev/caddy/generate-caddyfile.sh /usr/local/bin/wikijump-generate-caddyfile
COPY ./install/dev/caddy/update-caddyfile.sh /usr/local/bin/wikijump-update-caddyfile
COPY ./install/dev/caddy/request.json /etc/caddy-request.json

# Set up cron
COPY ./install/dev/caddy/crontab /etc/caddy/crontab
RUN crontab -u daemon /etc/caddy/crontab && \
    rm /etc/caddy/crontab

# Run cron and caddy
CMD ["/usr/local/bin/wikijump-start-caddy"]
