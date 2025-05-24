FROM rust:1.87 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/payments-backend /app/payments-backend
COPY .env /app/.env

EXPOSE 3002

CMD ["/app/payments-backend"] 
