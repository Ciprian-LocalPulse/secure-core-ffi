# Changelog

🇷🇴 [Citește în Română](CHANGELOG.ro.md)

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Planned
- Key derivation (Argon2 / HKDF)
- Official Julia binding
- npm package publication for the Wasm module

### Completed
- Official C header in `include/security_core.h`
- Node.js and Python ctypes binding integration tests
- `cargo-fuzz` target for malformed decrypt payloads
- Python package metadata for the ctypes binding

## [0.3.0] - 2026-09-12

### Added
- Official `include/security_core.h` header for C and C++ consumers.
- Node.js integration test suite and npm test script using `koffi`.
- Python ctypes package metadata and integration tests.
- `cargo-fuzz` target for malformed decryption payloads.
- CI coverage for native Rust, Clippy, formatting, Node.js, Python ctypes, PyO3, and WASM.

### Fixed
- Hardened FFI pointer and length validation and documented unsafe ABI contracts.

## [0.2.0] - 2026-09-12

### Added
- `wasm-bindgen` module for the `wasm32` compilation target, exposing `SecurityContext` (constructor, `encrypt`, `decrypt`) directly in JavaScript/TypeScript
- WebAssembly integration guide and example for Next.js (`bindings/wasm-nextjs/`), including `next.config.js` setup and a React component
- `tests/integration_test.rs` folder with integration tests (roundtrip, invalid key, corrupted data)
- `examples/` folder with standalone Rust programs (`basic_usage.rs`, `multi_message.rs`)
- `Makefile` with centralized build/test commands for the native core, Wasm, Python, and Node.js
- `bindings/python/requirements.txt` and `bindings/nodejs/package.json`
- `assets/` folder with the visual diagram of the Rust → WebAssembly → Next.js flow
- Dedicated CI job for WebAssembly builds (`wasm-pack build --target web`)
- Full bilingual documentation (English default, Romanian via `*.ro.md` files and `docs/ro/`)
- Extended documentation set: `WHITEPAPER.md`, `MANIFESTO.md`, `DONATE.md`, `CODE_OF_CONDUCT.md`
- `.github/workflows/ci.yml` — automated build and test on every push/PR (native + WebAssembly)
- `.gitignore` covering Rust build artifacts, Wasm output, Node.js and Python local files

### Fixed
- Added a `.gitignore` file, which was previously missing, to prevent build artifacts from being committed

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
