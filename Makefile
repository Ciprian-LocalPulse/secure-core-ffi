# Makefile — security-core-ffi
# Author: Ciprian Ștefan Pleșca

.PHONY: all build build-native build-wasm build-python build-nodejs test examples clean

all: build-native build-wasm build-python build-nodejs

## Build the native library (.so / .dll / .dylib) for the C++/Python/Node.js bindings
build-native:
	cargo build --release

## Build the WebAssembly module for Next.js / browser integration
build-wasm:
	wasm-pack build --target web --out-dir bindings/wasm-nextjs/pkg

## Prepare the environment for the Python binding
build-python:
	pip install -r bindings/python/requirements.txt --break-system-packages || true

## Prepare the environment for the Node.js binding
build-nodejs:
	cd bindings/nodejs && npm install

## Run unit tests + Rust integration tests
test:
	cargo test --release

## Run the included Rust examples (examples/)
examples:
	cargo run --example basic_usage
	cargo run --example multi_message

## Clean build artifacts
clean:
	cargo clean
	rm -rf bindings/wasm-nextjs/pkg
	rm -rf bindings/nodejs/node_modules
