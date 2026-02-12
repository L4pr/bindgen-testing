FROM rust:trixie

WORKDIR /deps

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo vendor > vendor-config.toml

RUN mkdir -p $CARGO_HOME && cat vendor-config.toml >> $CARGO_HOME/config.toml

RUN rm Cargo.toml Cargo.lock vendor-config.toml

WORKDIR /workspaces/your-app