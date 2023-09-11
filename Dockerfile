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
RUN cargo install sqlx-cli
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt install -y musl-tools
RUN cargo chef cook --release --recipe-path recipe.json
RUN apt-get update
RUN apt install -y libpython3.11
RUN apt install -y libpython3.11-dev

ADD .sqlx ./.sqlx
ADD common_data ./common_data
ADD server ./server
ADD frontend_requests ./frontend_requests
ADD Cargo.toml ./
# DO NOT copy .env for offline mode to work!

RUN cargo build --release --bin pothole-detection-server

FROM debian:bookworm-slim AS runtime


RUN apt-get update
RUN apt install -y ca-certificates
#RUN apt search openssl && exit 1
RUN apt install -y libssl3
RUN apt install -y libpython3.11
RUN apt install -y python3.11
RUN apt install -y python3-pip
RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/server/requirements.txt /
RUN pip3 install -r /requirements.txt --break-system-packages

RUN apt-get update
RUN apt install -y libgl1
RUN apt install -y libglib2.0-0



WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin
COPY --from=builder /app/target/release/pothole-detection-server /usr/local/bin
COPY --from=builder /app/server/migrations ./migrations
COPY server/best.pt ./

RUN pip3 install dill --break-system-packages
RUN pip3 install pillow --break-system-packages


ENTRYPOINT sqlx database setup && pothole-detection-server
