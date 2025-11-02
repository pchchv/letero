FROM rust:bookworm
RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /workspace
ADD migrations migrations
WORKDIR /workspace/server
ADD server .

RUN cargo build