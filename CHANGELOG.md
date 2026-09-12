# Changelog

🇷🇴 [Citește în Română](CHANGELOG.ro.md)

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- `wasm-bindgen` module for the `wasm32` compilation target, exposing `SecurityContext` (constructor, `encrypt`, `decrypt`) directly in JavaScript/TypeScript
- WebAssembly integration guide and example for Next.js (`bindings/wasm-nextjs/`), including `next.config.js` setup and a React component
- `tests/integration_test.rs` folder with integration tests (roundtrip, invalid key, corrupted data)
- `examples/` folder with standalone Rust programs (`basic_usage.rs`, `multi_message.rs`)
- `Makefile` with centralized build/test commands for the native core, Wasm, Python, and Node.js
- `bindings/python/requirements.txt` and `bindings/nodejs/package.json`
- `assets/` folder with the visual diagram of the Rust → WebAssembly → Next.js flow
- Dedicated CI job for WebAssembly builds (`wasm-pack build --target web`)
- Extended `.gitignore` (Wasm artifacts, `pkg/`, compiled native libraries)
- Full bilingual documentation (English default, Romanian via `*.ro.md` files and `wiki/ro/`)

### Planned
- Key derivation (Argon2 / HKDF)
- Official Julia binding
- Automated fuzzing of the FFI interface (cargo-fuzz)
- npm package publication for the Wasm module

## [0.1.0] - 2026-09-12

### Added
- Rust security core (`src/lib.rs`) implementing AES-256-GCM via `extern "C"`:
  - `init_security_context`
  - `encrypt_payload`
  - `decrypt_payload`
  - `free_buffer`
  - `free_security_context`
- Example C++ binding (`bindings/cpp/example.cpp`)
- Object-oriented Python binding based on `ctypes` (`bindings/python/security_core.py`)
- Node.js binding based on `ffi-napi` (`bindings/nodejs/security_core.js`)
- Multi-stage Dockerfile for reproducible builds
- Rust unit test suite (encrypt/decrypt roundtrip)
- README documentation with Mermaid diagrams (architecture, sequence, payload structure)
- Compliance files: `LICENSE` (Apache-2.0), `SECURITY.md`, `CITATION.cff`
- Initial Wiki pages (Home, Architecture, Integration Guide, FAQ)

### Author
- Architecture and original source code designed by **Ciprian Ștefan Pleșca**
