FROM rust:latest

WORKDIR /deps

COPY Cargo.toml Cargo.lock ./

RUN apt-get update && apt-get install -yq --no-install-recommends \
    clang \
    libclang-dev \
    cmake \
    build-essential \
    maven

RUN rustup component add rust-src

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo vendor /deps/vendor > vendor-config.toml


RUN bash -lc '\
  SYSROOT="$(rustc --print sysroot)" && \
  cd "$SYSROOT/lib/rustlib/src/rust/library/sysroot" && \
  cargo vendor --versioned-dirs --no-delete /deps/vendor'

RUN mkdir -p $CARGO_HOME && cat vendor-config.toml >> $CARGO_HOME/config.toml

RUN rm Cargo.toml Cargo.lock vendor-config.toml

WORKDIR /workspaces/offline-rust
