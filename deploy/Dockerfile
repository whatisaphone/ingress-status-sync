FROM rust:1.51.0-slim-buster AS build

WORKDIR /opt/build
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

################################################################################

FROM debian:buster-slim

COPY --from=build /opt/build/target/release/ingress-status-sync /opt/ingress-status-sync

ENTRYPOINT ["/opt/ingress-status-sync"]
