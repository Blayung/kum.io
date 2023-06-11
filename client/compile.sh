#!/bin/bash
source /usr/lib/emsdk/emsdk_env.sh
rm -rf ../docs
cargo web deploy --release --use-system-emscripten --output ../docs
