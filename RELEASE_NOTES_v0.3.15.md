# Secure Core FFI v0.3.15

## Title

Secure Core FFI v0.3.15 — Reproducible Cross-Language Release with Integrity Metadata

## Academic release description

Secure Core FFI v0.3.15 is a public developer release of a Rust-based AES-256-GCM authenticated-encryption core exposed through a C-compatible foreign-function interface. The release strengthens the project’s operational documentation and distribution process without expanding its cryptographic claims.

It adds a compatibility policy, a release-integrity procedure, contributor and support guidance, and a documented Node.js package boundary. GitHub Release artifacts now include a SHA-256 checksum manifest and a machine-readable Cargo dependency manifest to support reproducible deployment review.

## Distribution and verification

- Native archives for Linux x86_64, Windows x86_64, and macOS arm64.
- WebAssembly archive and public npm package.
- Python wheels through the release workflow.
- Node.js binding package with explicit native-library installation guidance.
- SHA-256 checksums and Cargo dependency metadata attached to the GitHub Release.

## Security status

This is a developer release. The core provides AES-256-GCM authenticated encryption when integrated with a valid 32-byte key and the documented ABI contract. Key management, authorization, rotation, compliance, and independent cryptographic audit remain outside the library’s guarantees.
