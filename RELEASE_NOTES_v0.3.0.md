# Security Core FFI v0.3.0

## Release summary

Version 0.3.0 consolidates `secure-core-ffi` as a reproducible, cross-language
authenticated-encryption component. The release preserves the AES-256-GCM
payload format while strengthening the native FFI contract and completing the
integration-quality layer around the Rust core.

## Technical contribution

- Hardened validation of raw pointers and buffer lengths at the C ABI boundary.
- Documented the safety contract of every unsafe FFI entry point.
- Added the official C/C++ header `include/security_core.h`.
- Added integration coverage for the Node.js `koffi` binding and Python
  `ctypes` binding.
- Added installable Python package metadata and a native PyO3 check in CI.
- Added a `cargo-fuzz` target for malformed decryption payloads.
- Extended CI with formatting, Clippy, native tests, WASM checks, and binding
  integration tests.
- Synchronized bilingual README, changelog, citation metadata, and package
  versions to `0.3.0`.

## Verification

- Rust unit and integration tests: 9 passing.
- `cargo fmt --check`: passing.
- Clippy with warnings denied: passing.
- WASM library check: passing.
- Node.js binding roundtrip and tamper tests: passing.
- Fuzz target compilation: passing.

## Scope and limitations

This release provides an authenticated-encryption core and integration
interfaces. Password-based key derivation (Argon2/HKDF), a Julia binding, and
publication of a generated WASM npm artifact remain intentionally separate
roadmap items because they require additional API and distribution decisions.

## Academic relevance

The release provides a compact experimental platform for studying the
separation of cryptographic implementation from language-specific interfaces,
memory-safe FFI design, authenticated-encryption payload framing, and
cross-language reproducibility. It is not a substitute for an independent
cryptographic audit or a complete key-management system.
