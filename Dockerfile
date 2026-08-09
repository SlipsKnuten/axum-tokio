# Build stage
FROM rust:1-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release


# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/axum-tokio .

EXPOSE 3000

CMD ["./axum-tokio"]
