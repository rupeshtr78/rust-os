#!/bin/bash

# add file rust-toolchain
# add unstable section in .cargo/config.toml

rustc --version --verbose
rustup component add rust-src --toolchain nightly-x86_64-apple-darwin
cargo build --target x86_64-rtr_os.json

cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rtr_os/debug/bootimage-rust-os-v1.bin
