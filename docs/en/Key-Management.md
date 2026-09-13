# Key Management Guidance

Secure Core FFI deliberately does not store, derive, rotate, escrow, or revoke keys. It accepts a 32-byte AES-256 key supplied by the integrating application.

## Minimum requirements

- Generate keys with a cryptographically secure random source or an approved KDF.
- Never hard-code keys in source code, images, examples, or client-side bundles.
- Keep keys outside logs, telemetry, crash reports, URLs, and analytics payloads.
- Restrict key access to the smallest service or process that needs encryption.
- Define rotation and revocation before storing production data.
- Erase application-owned key material when its lifecycle ends, subject to the runtime's guarantees.

## Browser and WebAssembly deployments

Client-side encryption does not make a browser-held key secret from the browser user, extensions, or a compromised client. Use WebAssembly only when the threat model accepts client-side key exposure. Server-side key management remains necessary for server-controlled secrets.

## Recommended operational design

Use a managed KMS or HSM for high-value keys, envelope encryption for large datasets, versioned key identifiers, audit logs that exclude plaintext and key material, and a tested recovery procedure. The library should receive only the short-lived key material required for the operation.

## What this library guarantees

Given a valid key and a correct call contract, the core provides authenticated encryption with AES-256-GCM and a fresh nonce per encryption operation. It does not guarantee safe key storage, identity, authorization, recovery, or regulatory compliance.
