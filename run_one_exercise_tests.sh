#!/bin/bash

if ([ -z "$1" ] || [ ! -d "$1" ]); then echo "Please provide the path to a valid exercise! (ex: ./exercises/ex00-writing-tests)"; exit; fi

EXERCISE=$1

echo "FROM ghcr.io/rusty-crewmates/substrate-tutorials:latest

ADD $EXERCISE  /home/substrate-tutorials/$EXERCISE

CMD [\"cargo\", \"test\", \"--manifest-path\", \"/home/substrate-tutorials/Cargo.toml\"]
   " > Dockerfile

docker build -t substrate-test-exercise .
docker run -t --rm substrate-test-exercise
rm Dockerfile



