FROM rust:1.74-alpine

# Install external dependencies on Alpine Linux.
RUN apk add musl-dev;\
    rustup target add wasm32-unknown-unknown;\
    cargo install --locked trunk wasm-bindgen-cli

# Define port for yew client.
EXPOSE 8080

# Setup working directory & initialize binary Cargo crate.
WORKDIR /var/www/yew-client

# Compile Cargo crate.
COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./index.html .
RUN trunk build --release

# Start the server.
COPY ./entrypoint.sh .
ENTRYPOINT ./entrypoint.sh
