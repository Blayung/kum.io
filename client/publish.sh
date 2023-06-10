#!/bin/bash
cp src/index.html ../docs
if ! [[ -d "../docs/pkg" ]]; then
    mkdir ../docs/pkg
fi
cp pkg/kum_io_client_bg.wasm ../docs/pkg
cp pkg/kum_io_client.js ../docs/pkg
