# Contributing

Thank you for improving Secure Core FFI.

## Security-sensitive changes

Do not report vulnerabilities in public issues. Follow [SECURITY.md](SECURITY.md). Cryptographic changes, FFI pointer ownership changes, payload-format changes, and build/release changes require explicit tests and documentation updates.

## Development workflow

1. Create a focused branch from `main`.
2. Update tests and documentation with the code change.
3. Run:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-targets
   ```

4. For binding changes, run the relevant Python, Node.js, or WebAssembly checks.
5. Describe compatibility and security implications in the pull request.

## Pull request rules

- Keep changes narrowly scoped.
- Do not add keys, tokens, or real encrypted customer data.
- Preserve the native ABI unless the change is explicitly versioned as breaking.
- Include a test for invalid input or failure behavior when changing FFI code.

## Documentation

User-facing behavior belongs in `README.md` and `docs/en/`. Romanian documentation should be updated whenever a matching Romanian document exists.
