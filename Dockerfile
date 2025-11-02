FROM rust:bookworm
RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /
ADD migrations migrations
WORKDIR /server
ADD server .

RUN cargo build