FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
# N.B.: the ADD and COPY commands do not copy the directory, but its contents.
# To copy a directory, provide its name as the destination
ADD .sqlx ./.sqlx
ADD common_data ./common_data
ADD server ./server
ADD frontend_requests ./frontend_requests
ADD Cargo.toml ./
#RUN ls && sleep 100
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo install sqlx-cli
RUN apt-get update && apt install -y musl-tools
RUN cargo chef cook --release --recipe-path recipe.json

ADD .sqlx ./.sqlx
ADD common_data ./common_data
ADD server ./server
ADD frontend_requests ./frontend_requests
ADD Cargo.toml ./
# DO NOT copy .env for offline mode to work!

RUN apt-get update
RUN apt install -y libpython3.11
RUN apt install -y libpython3.11-dev
RUN cargo build --release --bin pothole-detection-server

FROM debian:bookworm-slim AS runtime


RUN apt-get update
RUN apt install -y ca-certificates
#RUN apt search openssl && exit 1
RUN apt install -y libssl3
RUN apt install -y libpython3.11
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin
COPY --from=builder /app/target/release/pothole-detection-server /usr/local/bin
COPY --from=builder /app/server/migrations ./migrations
ENTRYPOINT sqlx database setup && pothole-detection-server
