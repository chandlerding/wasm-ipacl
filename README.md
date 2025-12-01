# WebAssembly (WASM) IP Access Control List (ACL) Filter

This project implements a network security filter as a WebAssembly (WASM) module. It is designed to be used with a service mesh or proxy that supports `proxy-wasm`, such as Google Cloud Service Extension Plugin. The primary function of this filter is to inspect incoming HTTP requests and block those originating from IP addresses on a configurable , potentially large blocklist.

The IP matching is highly efficient, utilizing an `IpnetTrie` data structure which allows for fast lookups against a large list of IP network ranges (CIDRs).

## Purpose

The main goal of this project is to provide a dynamic and high-performance IP-based access control mechanism at the edge of a network or service mesh. By compiling the logic to WASM, the filter can be dynamically loaded and updated in proxies like Envoy without requiring a proxy restart.

This is particularly useful for:
*   Blocking traffic from known malicious actors.
*   Restricting access to services from specific geographic locations or networks.
*   Implementing dynamic security policies that can be updated in near real-time.

## How It Works

1.  **Blocklist Generation**: A list of IP addresses and CIDR ranges is read from a text file.
2.  **Configuration Building**: A helper utility, `configbuilder`, reads the blocklist, creates an `IpnetTrie` data structure from it, and serializes this trie into a compact binary format. This binary data becomes the configuration for the WASM plugin.
3.  **WASM Filter Logic**: The main WASM module (`wasm-ipacl`) is loaded by the proxy.
4.  **Configuration Loading**: On initialization, the WASM module loads the binary configuration data and deserializes it back into an in-memory `IpnetTrie`.
5.  **Request Inspection**: For each incoming HTTP request, the filter extracts the client IP address from the `x-forwarded-for` header. It specifically uses the rightmost IP address, which typically represents the most recent client in the proxy chain.
6.  **Access Control**: It performs a "longest match" lookup in the `IpnetTrie`. If the client IP is found within any of the blocklisted CIDR ranges, the request is denied with an `HTTP 403 Forbidden` response. Otherwise, the request is allowed to continue through the filter chain.

## Usage and Deployment

The project includes scripts and code to build the WASM module, generate its configuration, and deploy it.

### Prerequisites

*   Rust toolchain with `wasm32-wasip1` target installed (`rustup target add wasm32-wasip1`).
*   Docker.
*   Google Cloud SDK (`gcloud`).

### Building the Components

The `update.sh` script automates the entire build and deployment process. Here are the key steps it performs:

1.  **Build the Configuration Builder**:
    The `configbuilder` utility is compiled from `src/bin/configbuilder.rs`.
    ```bash
    cargo build --release --bin configbuilder
    ```

2.  **Generate the Plugin Configuration**:
    A blocklist of IPs is defined in `tests/blocklist`. The `configbuilder` reads this file and outputs the serialized binary configuration.
    ```bash
    target/release/builder > tests/plugin.config < tests/blocklist
    ```

3.  **Build the WASM Module**:
    The core filter logic in `src/lib.rs` is compiled to the `wasm32-wasip1` target.
    ```bash
    cargo build --target wasm32-wasip1 --release
    ```

4.  **Test/Benchmark**:
    Use local docker container to test/benchmark the plugin's performance.
    ```bash
    tests/bench.sh 
    ```

### Deployment

Follow the [document](https://docs.cloud.google.com/service-extensions/docs/prepare-plugin-code#upload-code)  to deploy the compiled WASM module & configuration as a service extension plugin.