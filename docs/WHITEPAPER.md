# security-core-ffi: A Language-Agnostic, Memory-Safe Core for Authenticated Encryption

**Author:** Ciprian Ștefan Pleșca
**Affiliation:** Independent Researcher
**Contact:** contact@agentflow-enterprise.com
**Status:** Living document — revised alongside the project's source code

---

## Abstract

Applications that need authenticated encryption across multiple languages typically face two unsatisfying choices: reimplement the cryptographic logic separately in each language, risking subtle divergence between implementations, or delegate encryption entirely to the transport layer, leaving data-at-rest unprotected. This paper describes `security-core-ffi`, a small authenticated-encryption core written in Rust and exposed through a five-function C ABI, designed so that the same encryption logic can be linked identically from C++, Python, Node.js, and any language capable of calling a C dynamic library. We describe the design rationale, the threat model the core addresses, the memory-ownership contract at the language boundary, and the limitations that remain — deliberately — outside its scope.

## 1. Introduction

Cryptographic correctness is unusually unforgiving: a single reused nonce, an unchecked buffer, or a key retained too long in memory can silently undermine guarantees that look correct on paper. These failures are rarely due to choosing the wrong algorithm — AES-GCM has been standardized and scrutinized for over a decade — but to how the algorithm is wired into real systems, particularly systems composed of multiple languages maintained by different teams over time.

`security-core-ffi` starts from a narrow, testable claim: if the sensitive part of encryption (key handling, nonce generation, authentication tag verification) is implemented exactly once, in a language whose compiler rejects an entire class of memory-safety bugs at compile time, then every consuming language can be treated as a thin, replaceable layer around that core, rather than as a separate place where the same mistakes can be made again.

## 2. Design

### 2.1 The contract

The core exposes exactly five `extern "C"` functions:

```c
SecurityContext* init_security_context(const unsigned char* key_ptr, size_t key_len);
unsigned char*   encrypt_payload(SecurityContext* ctx, const unsigned char* data_ptr, size_t data_len, size_t* out_len);
unsigned char*   decrypt_payload(SecurityContext* ctx, const unsigned char* data_ptr, size_t data_len, size_t* out_len);
void             free_buffer(unsigned char* ptr, size_t len);
void             free_security_context(SecurityContext* ctx);
```

A minimal surface is a deliberate constraint, not an accident of scope: every additional function is a new place where a binding can misuse the contract, and every additional parameter is a new invariant that must be documented, tested, and remembered by whoever writes the next language binding.

### 2.2 Cryptographic scheme

The core uses AES-256-GCM, an AEAD (Authenticated Encryption with Associated Data) construction standardized in NIST SP 800-38D. Each call to `encrypt_payload` generates a fresh 96-bit nonce via the operating system's cryptographic random source, prepends it to the ciphertext, and appends a 128-bit authentication tag. The caller never supplies, stores, or reuses a nonce — removing, by construction, the single most common way GCM implementations fail in practice.

### 2.3 Memory ownership across the FFI boundary

Every buffer returned by the core is allocated inside Rust and its ownership is explicitly transferred to the caller (`Box::into_raw`, `mem::forget`). This is the one place where Rust's compile-time guarantees cannot follow the data: once a pointer crosses into C++, Python, or JavaScript, no compiler is checking that it is freed exactly once. The core's response to this is to make the contract as small and as explicit as possible — one free function per allocation type — and to document the two failure modes explicitly (leak on omission, undefined behavior on double-free) rather than attempting to paper over them.

## 3. Threat model

**In scope:**
- Confidentiality and integrity of encrypted payloads, contingent on key secrecy, as guaranteed by AES-256-GCM.
- Elimination of memory-safety bugs (buffer overflow, use-after-free, data races) *within the Rust core itself*.
- Prevention of accidental nonce reuse, via mandatory automatic nonce generation.

**Explicitly out of scope:**
- Key generation, derivation, storage, and rotation — left to the integrating application, since policy here varies by deployment and imposing one inside the core would trade flexibility for a benefit that is not clearly general.
- Correctness of the FFI binding itself in each consuming language. A binding written carelessly can still introduce memory bugs on its own side of the boundary; the core reduces the number of places where this can happen, but cannot eliminate it as a category.
- Side-channel resistance beyond what the underlying `aes-gcm` Rust crate and CPU-level AES-NI instructions provide. No additional masking or blinding is implemented at this layer.

Stating these boundaries explicitly is itself part of the contribution: much of the mistrust directed at cryptographic libraries in practice stems from an unstated mismatch between what a library guarantees and what a system built on it assumes it guarantees.

## 4. Related considerations

The general strategy of isolating a small, memory-safe core behind a stable C ABI is not novel in itself — it echoes long-standing engineering practice of minimizing trusted computing bases. What this project contributes is a concrete, minimal instantiation of that strategy applied specifically to authenticated encryption, with an explicit and documented ownership contract at the FFI boundary, rather than a general-purpose cryptography library that tries to cover every mode, cipher, and use case.

## 5. Limitations and future work

- No internal key derivation function (e.g., HKDF or Argon2) is implemented; integrating one is on the roadmap as an opt-in addition, not a default, to avoid imposing a derivation policy that may not fit every deployment.
- WebAssembly as a compilation target is planned but not yet supported.
- No independent third-party security audit has been published for this project at the time of writing. This is stated here plainly because it is directly relevant to any risk assessment a prospective integrator would need to make.

## 6. Conclusion

`security-core-ffi` is a narrow answer to a narrow problem: how to write authenticated encryption logic once, in a memory-safe language, and reuse it unmodified across a multi-language system. Its contribution is less about novel cryptography and more about a disciplined boundary — a small, auditable contract between a trusted core and everything built on top of it. The full architecture, API reference, and integration guide are maintained in the project's [documentation](docs/en/Home.md).

---

*This whitepaper is maintained alongside the project's source code and documentation. Corrections and technical discussion are welcome via the project's issue tracker or by contacting the author directly.*
