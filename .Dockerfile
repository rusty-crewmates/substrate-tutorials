FROM rust as base
ADD Cargo.toml Cargo.lock /home/substrate-tutorials/
ADD exercises /home/substrate-tutorials/exercises
RUN apt update && apt install -y git clang curl libssl-dev llvm libudev-dev pkg-config make\
    && rm -rf /var/lib/apt/lists/*\
    && rustup default stable\
    && rustup update\
    && rustup update nightly\
    && rustup target add wasm32-unknown-unknown --toolchain nightly\
    && cargo fetch --manifest-path /home/substrate-tutorials/Cargo.toml\
    && cargo test --manifest-path /home/substrate-tutorials/Cargo.toml --no-run; exit 0\
    && rm -rf /home/substrate-tutorials/exercises/*
