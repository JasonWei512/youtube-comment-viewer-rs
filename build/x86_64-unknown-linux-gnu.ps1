#!/bin/bash
docker run --rm -it -v "$(pwd):/src" --workdir /src rust:buster cargo build --release --target x86_64-unknown-linux-gnu 