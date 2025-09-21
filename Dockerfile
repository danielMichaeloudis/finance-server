# --- Build Stage ---
FROM rust:latest AS builder

WORKDIR /app

RUN rustup component add rustfmt clippy

RUN git clone --depth=1 https://github.com/danielMichaeloudis/ledgerly.git . 

ENV SQLX_OFFLINE=true

RUN cargo build --release --verbose

FROM debian:bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    libssl3

WORKDIR /app
COPY --from=builder /app/target/release ./target/release
COPY migrations ./migrations
COPY .env .env
COPY /src/website/js ./src/website/js

# Add wait-for-it
ADD https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh /wait-for-it.sh
RUN chmod +x /wait-for-it.sh

CMD ["/wait-for-it.sh", "postgres:5432", "--", "./target/release/ledgerly"]