FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:unstable-slim
COPY --from=builder /usr/local/cargo/bin/cryptcoin /usr/local/bin/cryptcoin

CMD ["cryptcoin"]