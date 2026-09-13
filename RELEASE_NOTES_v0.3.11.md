# Secure Core FFI v0.3.11

## Highlights

- Fixed the release workflow artifact-name expression that caused zero-job failures.
- Tag-only release trigger for native, Python and WebAssembly artifacts.
- Public launch page deployed through Vercel.
- Canonical WASM package name: `secure-core-ffi-wasm`.

## Security scope

This is an authenticated-encryption building block. Applications remain responsible for key storage, derivation, rotation, access control and threat modeling. No independent cryptographic audit is claimed.
