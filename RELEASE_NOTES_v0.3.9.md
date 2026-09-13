# Secure Core FFI v0.3.9

## Highlights

- Production launch page deployed through Vercel.
- Native Linux, Windows and macOS artifacts.
- Python wheels for CPython 3.10–3.13.
- WebAssembly packaging with the canonical `secure-core-ffi-wasm` name.
- Release jobs are gated to version tags and cannot fail on ordinary `main` pushes.

## Security scope

This is an authenticated-encryption building block. Applications remain responsible for key storage, derivation, rotation, access control and threat modeling. No independent cryptographic audit is claimed.
