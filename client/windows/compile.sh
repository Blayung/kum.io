#!/bin/bash
cargo build --target x86_64-pc-windows-gnu --release
cp ../target/x86_64-pc-windows-gnu/release/kum-io-client.exe .
