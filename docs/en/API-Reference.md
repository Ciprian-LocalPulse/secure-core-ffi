# API Reference (FFI)

*Author: Ciprian Ștefan Pleșca*

This page documents, function by function, the entire public contract of the core. All five functions are part of the library's stable surface — any change to their signatures constitutes a breaking change and requires a new major version.

## `init_security_context`

```c
SecurityContext* init_security_context(const unsigned char* key_ptr, size_t key_len);
```

Initializes a security context with an AES-256 key (32 bytes).
Returns `NULL` if `key_ptr` is null or `key_len != 32`.

**Cost:** O(1) relative to key size — the dominant cost is constructing the internal `Aes256Gcm` structure, not an expensive cryptographic operation.
**Called:** once per session using the context.

## `encrypt_payload`

```c
unsigned char* encrypt_payload(SecurityContext* ctx, const unsigned char* data_ptr, size_t data_len, size_t* out_len);
```

Encrypts `data_ptr` (length `data_len`) using AES-256-GCM.
The result has the format `nonce (12B) || ciphertext || tag (16B)`, with the total length written to `out_len`.

**Note:** the result length is always `data_len + 28` bytes (12 for the nonce, 16 for the tag); the ciphertext itself has the same length as the original plaintext — GCM is a "stream-like" mode, with no padding.
**Nonce:** generated automatically, unique per call, via the operating system's cryptographic generator (`OsRng`); the caller cannot and must not supply its own nonce.

## `decrypt_payload`

```c
unsigned char* decrypt_payload(SecurityContext* ctx, const unsigned char* data_ptr, size_t data_len, size_t* out_len);
```

Decrypts a payload produced by `encrypt_payload`. Returns `NULL` if GCM authentication fails (corrupted data or wrong key).

**Important:** a `NULL` on decryption does not distinguish between "accidentally corrupted data" and "adversarially modified data" — and this is intentional. If the library returned different information for the two cases, it would open a side channel (oracle) exploitable by an attacker who submits modified payloads and observes the behavioral difference.

## `free_buffer`

```c
void free_buffer(unsigned char* ptr, size_t len);
```

Frees a buffer allocated by `encrypt_payload` / `decrypt_payload`. **Mandatory** to call for every buffer received, otherwise a memory leak occurs.

**Why an explicit call is needed:** the buffer's memory is allocated inside Rust and handed to the caller via `Box::into_raw`; from Rust's perspective, ownership has been transferred, so the Rust allocator will not free it automatically. `free_buffer` is the only correct way to return it to the allocator, regardless of which language calls the library.

## `free_security_context`

```c
void free_security_context(SecurityContext* ctx);
```

Frees the security context (including the key in memory).

**Security note:** freeing the context also destroys the internally held copy of the key; however, guaranteed erasure from memory (explicit zeroing before deallocation, to prevent key recovery from a memory dump) is implementation-dependent — consult the source code of the installed version if this detail is critical to your threat model.

## Summary table

| Function | Input | Output | Notes |
|---|---|---|---|
| `init_security_context` | 32B key | `SecurityContext*` or `NULL` | called once per session |
| `encrypt_payload` | context + plaintext | `nonce\|\|ciphertext\|\|tag` | unique nonce per call |
| `decrypt_payload` | context + encrypted payload | plaintext or `NULL` | verifies the GCM tag |
| `free_buffer` | pointer + length | — | frees a returned buffer |
| `free_security_context` | context | — | frees the key from memory |

## General memory usage rules

1. Any pointer returned by `encrypt_payload` or `decrypt_payload` must be freed exactly once, via `free_buffer`.
2. Any `SecurityContext*` pointer returned by `init_security_context` must be freed exactly once, via `free_security_context`.
3. No pointer should be used after being freed (use-after-free), and no pointer should be freed twice (double-free) — both rules are guaranteed automatically by Rust *inside* the core, but cannot be guaranteed automatically *across* the FFI boundary, which is why every binding must respect them explicitly (see the common-errors table in the [Integration Guide](Integration-Guide.md)).

---

<div align="center">
<sub>

[Home](Home.md) · [Architecture](Architecture.md) · [Integration Guide](Integration-Guide.md) · [API Reference](API-Reference.md) · [FAQ](FAQ.md) · [Author](Author.md)

</sub>

<img src="https://img.shields.io/badge/AES--256--GCM-authenticated%20encryption-1f6feb?style=flat-square" alt="AES-256-GCM"/>
<img src="https://img.shields.io/badge/core-Rust-orange?style=flat-square&logo=rust" alt="Rust core"/>
<img src="https://img.shields.io/badge/interface-C%20FFI-6e5494?style=flat-square" alt="C FFI"/>
<img src="https://img.shields.io/badge/bindings-C%2B%2B%20%7C%20Python%20%7C%20Node.js-2ea043?style=flat-square" alt="Bindings"/>

<sub>
<b>security-core-ffi</b> — architecture &amp; documentation by <a href="Author.md"><b>Ciprian Ștefan Pleșca</b></a><br/>
Independent researcher · <a href="mailto:contact@agentflow-enterprise.com">contact@agentflow-enterprise.com</a>
</sub>

<sub>
Found an issue in the documentation? Open a discussion or contact the author directly.<br/>
To report a security vulnerability, use <b>SECURITY.md</b> via GitHub Security Advisories — not this contact address.
</sub>

<sub>© 2026 Ciprian Ștefan Pleșca. Documentation maintained alongside the <code>security-core-ffi</code> project.</sub>

</div>
