# Secure Core FFI v0.3.13

## Title

Secure Core FFI v0.3.13 — Documented AES-256-GCM Core for Cross-Language Integration

## Academic release description

Secure Core FFI v0.3.13 is a public developer release of a Rust-based authenticated-encryption core using AES-256-GCM and exposed through a C-compatible foreign-function interface. This release synchronizes version metadata across the Rust core, Python packages, Node.js binding, WebAssembly package, README documentation, citation metadata, and the public launch page.

The release adds formal documentation for the system threat model, key-management responsibilities, browser/WebAssembly limitations, operational release readiness, and security-reporting boundaries. It clarifies the distinction between a cryptographic primitive and a complete security product: key storage, derivation, rotation, authorization, compliance, and independent audit remain outside the library's guarantees.

## Distribution

- GitHub Release artifacts for Linux, Windows, macOS, and WebAssembly.
- Python wheels published through the release workflow.
- Public WebAssembly npm package published with provenance-enabled CI.

## Verification

- Rust unit and integration tests.
- Native build matrix across Linux, Windows, and macOS.
- WebAssembly build.
- Python wheel matrix across supported Python versions.

## Security status

This is a developer release. It is not independently cryptographically audited and must be evaluated within the threat model of the integrating application.
