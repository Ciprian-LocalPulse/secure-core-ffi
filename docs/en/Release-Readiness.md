# Release Readiness

## v0.3.15 public status

- GitHub Actions build matrix: native Linux, Windows, macOS, WebAssembly, and Python wheels.
- Public distribution: GitHub Release, PyPI, and the WASM npm package.
- Landing page: Vercel deployment with current release links.
- Core verification: unit and integration tests for round trips, invalid keys, malformed input, and corrupted ciphertext.

## Before production adoption

- Complete an independent cryptographic and FFI security review.
- Run continuous fuzzing and sanitizers against the C ABI and all bindings.
- Establish a supported-version and deprecation policy.
- Verify release checksums and the Cargo dependency manifest before deployment.
- Document key ownership, rotation, revocation, backup, and incident response for the consuming application.
- Validate the complete integration in the target operating system, runtime, and deployment model.

## Release rule

The project may publish developer releases before an audit, but must not describe them as audited, compliant, or complete key-management products. A stable `1.0.0` should follow a documented review and compatibility decision.
