#!/bin/sh
  
docker run -it -v $(pwd):/mnt  \
    us-docker.pkg.dev/service-extensions-samples/plugins/wasm-tester:main  \
    --proto /mnt/tests.textpb   \
    --plugin /mnt/target/wasm32-wasip1/release/wasm_ip.wasm \
    --config /mnt/plugin.config 