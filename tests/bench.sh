#!/bin/sh
# Run this script in the repo root directory, e.g. cd .. ; tests/bench.sh
# make sure to build both the plugin and also configuration follow the guide in README.md before run this

docker run -it -v $(pwd):/mnt  \
    us-docker.pkg.dev/service-extensions-samples/plugins/wasm-tester:main  \
    --proto /mnt/tests/tests.textpb   \
    --plugin /mnt/target/wasm32-wasip1/release/wasm_ip.wasm \
    --config /mnt/tests/plugin.config 
