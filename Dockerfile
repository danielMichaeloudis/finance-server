
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

#For caching dependancies 
RUN cargo build --release || true 
RUN rm -rf src

COPY . .
COPY .sqlx .sqlx

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/finance-server .
COPY migrations ./migrations

CMD ["./finance-server"]