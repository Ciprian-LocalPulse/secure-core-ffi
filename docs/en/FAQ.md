# Frequently Asked Questions (FAQ)

*Author: Ciprian Ștefan Pleșca*

**Q: Why Rust and not C/C++ for the core?**
A: Rust provides compile-time-verified memory safety guarantees (no buffer overflows, use-after-free, or data races), eliminating an entire class of vulnerabilities typical of cryptographic modules written in plain C, while still keeping native performance — there is no runtime cost for these guarantees, unlike a garbage-collected language.

**Q: What encryption algorithm is used?**
A: AES-256-GCM (AEAD) — authenticated encryption with 256-bit keys, which guarantees both data confidentiality (no one without the key can read the plaintext) and integrity (any modification of the ciphertext is detected at decryption, which fails explicitly instead of silently producing corrupted data).

**Q: Who manages key rotation?**
A: The application integrating the library. The core receives an already generated or derived key; it does not (yet) implement an internal derivation or rotation mechanism. This is a deliberate choice, not an omission: rotation policy varies enormously between applications (from "never" to "every session"), and imposing a fixed policy inside the core would reduce flexibility without a clear security benefit. See the roadmap for integrating a derivation library (Argon2/HKDF) as an option, not an obligation.

**Q: Is it safe to reuse a nonce?**
A: You don't need to ask yourself this question in practice — the core automatically generates a fresh, cryptographically random nonce for every `encrypt_payload` call, preventing accidental reuse. Reusing a nonce under the same key is generally catastrophic for AES-GCM security (it can lead to recovery of the authentication key), which is why the library does not expose any option to supply a manual nonce.

**Q: What happens if the payload to decrypt was truncated or partially corrupted?**
A: `decrypt_payload` returns `NULL`. GCM authentication tag verification fails for any modification of the data, whether it's accidental corruption (transmission error) or an intentional modification — the library does not distinguish between the two cases, and it shouldn't, for the reasons explained in the [API Reference](API-Reference.md).

**Q: Can I integrate the library into WebAssembly?**
A: Yes. The current release includes a `wasm-bindgen` interface and the CI workflow builds the WebAssembly target. See `bindings/wasm-nextjs/README.md` for browser and Next.js integration.

**Q: How do I report a vulnerability?**
A: See `SECURITY.md` — private reporting through GitHub Security Advisories, not through direct contact or public channels, to allow a fix before disclosure.

**Q: Has the library been independently audited?**
A: As of this writing, there is no published external audit. The absence of an audit does not mean the absence of quality, but it is relevant information for any team evaluating the library for a security-critical system — treat this as a known limitation, not a footnote.

**Q: Why does the FFI API expose only five functions?**
A: To reduce the attack surface of the contract and limit the number of ways a binding can get the integration wrong. The full reasoning is discussed in the design principles section of [Architecture](Architecture.md).

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
