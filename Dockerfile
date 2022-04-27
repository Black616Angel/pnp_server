FROM rust:1.60-slim-buster as builder

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

WORKDIR /usr/src/pnp_server
RUN apt-get update && apt-get install -y libc6-dev pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN cargo install --path .

WORKDIR /usr/src/pnp_client
COPY ./pnp_client/. .
RUN cargo build --release --target wasm32-unknown-unknown

FROM rust:1.60-slim-buster
RUN apt-get update && apt-get install -y libc6-dev pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pnp_server /usr/local/bin/pnp_server
RUN mkdir /var/www/
COPY ./files /var/www/.
COPY --from=builder /usr/src/pnp_client/target/wasm32-unknown-unknown/release/pnp_client.wasm /var/www/root/pnp_client.wasm
CMD ["pnp_server"]
EXPOSE 8080/tcp