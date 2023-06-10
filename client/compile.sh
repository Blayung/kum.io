#!/bin/bash
rm -rf ../docs
cargo web deploy --release --use-system-emscripten --output ../docs
