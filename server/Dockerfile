FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY  --from=planner /app/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin pothole-detection-server

FROM debian:bullseye-slim AS runtime
RUN apt-get update && apt install -y sqlite3 ca-certificates
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pothole-detection-server /usr/local/bin
ENTRYPOINT ["pothole-detection-server"]
