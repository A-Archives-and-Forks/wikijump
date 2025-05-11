#
# WWS build
#

FROM rust:latest AS rust

# Install files
COPY ./install/local/wws/wws-start /usr/local/bin/wikijump-wws-start
COPY ./install/local/wws/health-check.sh /usr/local/bin/wikijump-health-check

# Copy source
RUN mkdir /src
COPY ./wws /src/wws
WORKDIR /src/wws

# Cache rust dependencies
RUN mkdir .cargo && cargo vendor > .cargo/config.toml

# Build wws
RUN cargo build --release

#
# Final image
#

FROM debian:latest

# Install files
COPY --from=rust /src/wws/target/release/wws /usr/local/bin/wws
COPY ./install/dev/wws/health-check.sh /usr/local/bin/wikijump-health-check

USER daemon
EXPOSE 7000
CMD ["/usr/local/bin/wws"]
