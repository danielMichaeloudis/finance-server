
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

WORKDIR /app
COPY --from=builder /app/target/release/finance-server .
COPY migrations ./migrations

CMD ["./finance-server"]