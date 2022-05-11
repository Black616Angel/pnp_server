FROM rust:1.60-slim-buster as builder

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

WORKDIR /usr/src
RUN apt-get update && apt-get install -y libc6-dev pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown

#for higher speeds on subsequent builds we prebuild an empty project and insert the sources later

## server first
# Create blank project
RUN USER=root cargo new pnp_server
WORKDIR /usr/src/pnp_server
# We want dependencies cached, so copy those first.
COPY Cargo.toml .
COPY Cargo.lock .

## then client
# Create blank project
RUN USER=root cargo new pnp_client --lib
WORKDIR /usr/src/pnp_server/pnp_client
# We want dependencies cached, so copy those first.
COPY pnp_client/Cargo.toml .
COPY pnp_client/Cargo.lock .

## now build both
WORKDIR /usr/src/pnp_server
RUN cargo build --release
WORKDIR /usr/src/pnp_server/pnp_client
RUN cargo build --release --target wasm32-unknown-unknown

## now the subsequent compilations should start from here
WORKDIR /usr/src/pnp_server/pnp_client
COPY ./pnp_client/src/. ./src/.
RUN cargo build --release --target wasm32-unknown-unknown

WORKDIR /usr/src/pnp_server
COPY ./src/. ./src/.
RUN cargo install --path .

FROM rust:1.60-slim-buster
RUN apt-get update && apt-get install -y libc6-dev pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev libssl-dev && rm -rf /var/lib/apt/lists/*
RUN mkdir /var/www/
COPY --from=builder /usr/local/cargo/bin/pnp_server /usr/local/bin/pnp_server
COPY ./files /var/www/.
COPY --from=builder /usr/src/pnp_server/pnp_client/target/wasm32-unknown-unknown/release/pnp_client.wasm /var/www/pnp_client.wasm
CMD ["pnp_server"]
EXPOSE 8080/tcp