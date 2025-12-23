# Build Stage
FROM rust:1.91.0-alpine AS builder

RUN apk add --no-cache musl-dev perl make gcc openssl-dev

WORKDIR /app

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./

# Copy all crates
COPY crates ./crates

# Build all binaries
RUN cargo build --release --locked

# Final Stage for API
FROM alpine:3.21 AS api
RUN apk add --no-cache libgcc openssl
WORKDIR /app
COPY --from=builder /app/target/release/api /app/api
EXPOSE 3000
CMD ["./api"]

# Final Stage for Worker
FROM alpine:3.21 AS worker
RUN apk add --no-cache libgcc openssl
WORKDIR /app
COPY --from=builder /app/target/release/worker /app/worker
CMD ["./worker"]
