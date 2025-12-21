# Build Stage
FROM rust:1.91.0-alpine AS builder

RUN apk add --no-cache musl-dev perl make gcc openssl-dev

WORKDIR /usr/
RUN USER=root cargo new app
WORKDIR /usr/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

# Bundle Stage
FROM alpine:3.22.0

ARG BUILD_NUMBER
ENV DD_VERSION="${BUILD_NUMBER}"

COPY --from=builder /usr/app/target/release/api /usr/app

CMD ["/usr/app"]
