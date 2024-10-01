FROM rust:1.77

WORKDIR /zkp-server

COPY . .

RUN cargo build --release --bin server --bin client