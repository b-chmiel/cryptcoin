# syntax = docker/dockerfile:experimental
FROM rust:slim as builder
ENV APP_NAME cryptcoin

WORKDIR /app

RUN cargo install cargo-chef

COPY Cargo.lock .
COPY Cargo.toml .

RUN --mount=type=cache,target=/root/.cargo \
	--mount=type=cache,target=/usr/local/cargo/registry \
	cargo chef prepare --recipe-path recipe.json && \
	cargo chef cook --release --recipe-path recipe.json

COPY src/ src/
RUN --mount=type=cache,target=/root/.cargo \
	--mount=type=cache,target=/usr/local/cargo/registry \
	cargo build --release

COPY Settings.prod.toml .

FROM debian:unstable-slim
COPY --from=builder /app/target/release/cryptcoin /usr/local/bin/
COPY --from=builder /app/Settings.prod.toml /etc/cryptcoin/

ENV ENV_FILE /etc/cryptcoin/Settings.prod.toml
ENV RUST_BACKTRACE 1
ENTRYPOINT ["cryptcoin"]