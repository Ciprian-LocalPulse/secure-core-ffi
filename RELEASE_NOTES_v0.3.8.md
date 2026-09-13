# Secure Core FFI v0.3.8

## Highlights

- Rust AES-256-GCM core with hardened FFI pointer and length validation.
- Native artifacts for Linux, Windows and macOS.
- Python wheels for CPython 3.10–3.13.
- WebAssembly package for browser and Next.js integrations.
- Standardized npm package name: `secure-core-ffi-wasm`.
- Public product page: https://secure-core-ffi.vercel.app/

## Verification

- Rust unit and integration tests pass.
- Formatting and strict Clippy checks pass.
- PyO3 binding validates with a locked Cargo dependency graph.

## Security scope

This release is an authenticated-encryption building block. Applications remain responsible for key storage, derivation, rotation, access control and threat modeling. It has not undergone an independent cryptographic audit.
