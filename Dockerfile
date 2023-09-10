FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install sqlx-cli
RUN apt-get update && apt install -y musl-tools
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
# RUN sqlx database create
# RUN cd server && sqlx migrate run
RUN cargo build --release --target x86_64-unknown-linux-musl --bin pothole-detection-server

FROM debian:bullseye-slim AS runtime
RUN apt-get update && apt install -y sqlite3 ca-certificates strace
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pothole-detection-server /usr/local/bin
ENTRYPOINT sqlx database setup && pothole-detection-server
