FROM rust:1.60-slim-buster as builder
RUN apt-get update && apt-get install -y libc6-dev pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/pnp_server
COPY . .
RUN cargo install --path .

WORKDIR /usr/src/pnp_client
COPY ./pnp_client/. .
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown

FROM debian:buster
RUN apt-get update && apt-get install -y libc6-dev pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pnp_server /usr/local/bin/pnp_server
RUN mkdir /var/www/
COPY --from=builder /usr/src/pnp_server/files /var/www/.
COPY --from=builder /usr/src/pnp_client/target/wasm32-unknown-unknown/release/pnp_client.wasm /var/www/pnp_client.wasm
CMD ["pnp_server"]