#!/bin/bash

if ([ -z "$1" ] || [ ! -d "$1" ]); then echo "Please provide the path to a valid exercise! (ex: ./exercises/ex00-writing-tests)"; exit; fi

EXERCISE=$1

echo "FROM rust as base

ADD Cargo.toml Cargo.lock /home/substrate-tutorials/
ADD exercises /home/substrate-tutorials/exercises

RUN apt update && apt install -y git clang curl libssl-dev llvm libudev-dev pkg-config make\
    && rm -rf /var/lib/apt/lists/*\
    && rustup default stable\
    && rustup update\
    && rustup update nightly\
    && rustup target add wasm32-unknown-unknown --toolchain nightly\
    && cargo fetch --manifest-path /home/substrate-tutorials/Cargo.toml
RUN cargo test --manifest-path /home/substrate-tutorials/Cargo.toml --no-run; exit 0
RUN rm -rf /home/substrate-tutorials/exercises/*

FROM base as test

ADD $EXERCISE  /home/substrate-tutorials/$EXERCISE

CMD [\"cargo\", \"test\", \"--manifest-path\", \"/home/substrate-tutorials/Cargo.toml\"]
   " > Dockerfile

docker build -t substrate-testing-image --target test . 2> /dev/null
docker run -t --rm substrate-testing-image
rm Dockerfile



