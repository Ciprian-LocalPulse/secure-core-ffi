# Secure Core FFI v0.3.12

## Highlights

- Corrected npm registry authentication in the release workflow.
- Keeps npm publishing public, provenance-enabled, and based on the generated WASM package.
- Synchronizes the 0.3.12 package metadata across Rust, Python, Node.js, and WebAssembly bindings.
- Keeps the launch page and release metadata aligned with the tagged release.

## Verification

- Rust unit and integration tests pass locally.
- Native, WebAssembly, and Python wheel matrix jobs are validated in GitHub Actions.
