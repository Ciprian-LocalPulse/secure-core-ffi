# Secure Core FFI v0.3.14

## Title

Secure Core FFI v0.3.14 — Synchronized Rust, Python, Node.js and WebAssembly Distribution

## Academic release description

Secure Core FFI v0.3.14 is a public developer release of a Rust-based AES-256-GCM authenticated-encryption core exposed through a C-compatible foreign-function interface. This release completes the distribution synchronization by publishing the Node.js binding alongside the Python and WebAssembly packages, while preserving the documented threat model, key-management boundaries, and reproducible multi-platform build matrix.

The release is intended for developers integrating a focused cryptographic primitive into applications that already provide key provenance, authorization, rotation, storage, recovery, and incident response. It does not claim independent cryptographic audit, regulatory compliance, or complete key-management functionality.

## Distribution

- GitHub Release artifacts for Linux, Windows, macOS, and WebAssembly.
- Python wheels published through the release workflow.
- Public WebAssembly npm package published with provenance-enabled CI.
- Public Node.js binding published from `bindings/nodejs`.

## Verification

- Rust unit and integration tests.
- Native build matrix across Linux, Windows, and macOS.
- WebAssembly build.
- Python wheel matrix across supported Python versions.
- Automated npm publication for both JavaScript packages.

## Security status

This is a developer release. It must be evaluated within the threat model of the integrating application and is not independently cryptographically audited.
