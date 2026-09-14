# Compatibility Policy

## Current status

The project is pre-1.0. Releases use Semantic Versioning as an intention, with the following additional rule: a release tag identifies a reproducible source revision and its generated distribution artifacts.

## Stable surface

The native C ABI declared in `include/security_core.h` is the compatibility boundary. Changes to exported function names, parameter types, pointer ownership, payload layout, or error behavior are breaking changes and require a major-version decision.

The encrypted payload format is:

```text
12-byte nonce || ciphertext || 16-byte authentication tag
```

Consumers must preserve these bytes exactly. No backward compatibility is promised for undocumented internal Rust types or generated build paths.

## Supported environments

- Rust: stable toolchain used by CI.
- Native release artifacts: Linux x86_64, Windows x86_64, and macOS arm64.
- Python wheels: CPython 3.10 through 3.13 on the CI matrix.
- Node.js binding: Node.js 18 or newer, with a matching native library.
- WebAssembly: modern browsers and bundlers supporting standard WebAssembly modules.

## Deprecation

Breaking changes are documented in `CHANGELOG.md`. A supported release receives fixes only when a security or release-blocking issue is confirmed. Applications with long-term compatibility requirements should pin an exact version and retain the corresponding checksum manifest.
