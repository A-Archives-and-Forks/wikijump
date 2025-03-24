#
# WWS build
#

# This image is modified for development, it retains the
# full rust container and rebuilds as needed, to ease
# iteration during development.

FROM rust:latest AS rust

# Install helpers
RUN cargo install cargo-watch sqlx-cli

# Install files
COPY ./install/local/router/wws-start /usr/local/bin/wikijump-wws-start
COPY ./install/local/router/health-check.sh /usr/local/bin/wikijump-health-check

# Copy source
# Don't build until container execution (see cargo-watch)
RUN mkdir /src
COPY ./wws /src/wws
WORKDIR /src/wws

EXPOSE 8000
CMD ["/usr/local/bin/wikijump-wws-start"]
