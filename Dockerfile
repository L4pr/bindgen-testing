FROM rust:latest

WORKDIR /deps

COPY Cargo.toml Cargo.lock ./

RUN apt-get update && apt-get install -yq --no-install-recommends \
    clang \
    libclang-dev \
    cmake \
    build-essential \
    maven

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo vendor /deps/vendor > vendor-config.toml

RUN mkdir -p $CARGO_HOME && cat vendor-config.toml >> $CARGO_HOME/config.toml

RUN rm Cargo.toml Cargo.lock vendor-config.toml

WORKDIR /workspaces/offline-rust
