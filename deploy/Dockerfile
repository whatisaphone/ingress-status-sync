FROM rust:1.51.0-slim-buster AS build

RUN apt-get update \
    && apt-get install -y \
        libssl-dev \
        pkg-config \
    && rm -r /var/lib/apt/lists

WORKDIR /opt/build
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

################################################################################

FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y libssl1.1 \
    && rm -r /var/lib/apt/lists

COPY --from=build /opt/build/target/release/ingress-status-sync /opt/ingress-status-sync

ENTRYPOINT ["/opt/ingress-status-sync"]
