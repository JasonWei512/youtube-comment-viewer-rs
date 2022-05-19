#!/bin/bash
docker run --rm -it -v "$(pwd):/home/rust/src" messense/rust-musl-cross:aarch64-musl cargo build --release --target aarch64-unknown-linux-musl --features vendored-openssl