
FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm -rf src

COPY ./src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/finance-server .

CMD ["./finance-server"]