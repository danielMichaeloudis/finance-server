# --- Build Stage ---
FROM rust:latest AS builder

WORKDIR /app

RUN rustup component add rustfmt clippy

#COPY Cargo.toml Cargo.lock ./

#RUN mkdir src && echo "fn main() {}" > src/main.rs
#RUN cargo build --release
#RUN rm -rf src

COPY . .
COPY .sqlx .sqlx

ENV SQLX_OFFLINE=true

RUN cargo build --release --verbose

FROM debian:bookworm-slim

RUN sed -i 's|http://deb.debian.org|https://deb.debian.org|g' /etc/apt/sources.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release ./target/release
COPY migrations ./migrations
COPY .env .env

CMD ["./target/release/finance-server"]