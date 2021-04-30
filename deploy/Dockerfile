FROM rust:1.51.0-alpine3.13 AS build

WORKDIR /opt/build
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

################################################################################

FROM alpine:3.13

COPY --from=build /opt/build/target/release/ingress-status-sync /opt/ingress-status-sync

ENTRYPOINT ["/opt/ingress-status-sync"]
