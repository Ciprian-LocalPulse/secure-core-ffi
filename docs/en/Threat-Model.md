# Threat Model

## Scope

Secure Core FFI provides authenticated encryption and decryption using AES-256-GCM. It exposes a small Rust implementation through a C ABI and language bindings. The library protects plaintext confidentiality and integrity when the caller supplies a strong, secret 32-byte key and follows the memory-ownership contract.

## Assets

- Plaintext and ciphertext confidentiality.
- Ciphertext integrity and authenticity.
- Application-managed encryption keys.
- FFI process memory and returned buffers.

## Trust boundaries

1. The application supplies keys, plaintext, ciphertext, lengths, and output buffers.
2. The Rust core validates lengths and owns the cryptographic operation.
3. Bindings translate the ABI contract into language-specific objects.
4. Key storage, access control, rotation, backup, and deletion remain outside this repository.

## In-scope threats

- Tampered or truncated ciphertext.
- Invalid key lengths and malformed FFI inputs.
- Accidental nonce reuse inside the library.
- Memory misuse at allocation and release boundaries.
- Dependency or release-artifact tampering.

## Out of scope

- Compromise of the host operating system or process memory.
- Weak, leaked, reused, or improperly stored application keys.
- Password-based key derivation and key rotation.
- Side-channel resistance against a hostile co-resident process.
- Compliance claims, certification, or independent cryptographic assurance.

## Security assumptions

- The caller provides a uniformly random 32-byte key or a key produced by an approved KDF.
- A unique fresh nonce is generated for each encryption operation under a given key.
- Consumers release returned buffers exactly once and close contexts when finished.
- Release artifacts are obtained from the tagged project workflow and verified according to the consumer's supply-chain policy.

## Required consumer controls

Consumers must define key provenance, storage, rotation, revocation, access control, backup, logging policy, and incident response. Production deployments should also perform an application-specific threat model and independent review.
