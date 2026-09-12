# About the author

<img src="../assets/ciprian-stefan-plesca.jpg" alt="Ciprian Ștefan Pleșca" width="220" />

*[Română](../ro/Autor.md) · English*


## Ciprian Ștefan Pleșca

**Independent researcher · Author and designer of the `security-core-ffi` architecture/code**
**Contact:** contact@agentflow-enterprise.com

Ciprian Ștefan Pleșca is the author and architecture designer behind `security-core-ffi`, an authenticated encryption core written in Rust and exposed through a stable FFI interface, designed to be consumed identically from C++, Python, Node.js, and other languages capable of linking a C dynamic library.

This document — the extended project documentation — reflects a central concern of the author: a clear separation between security logic (which must be written once, correctly, in a memory-safe language) and the integration layers specific to each language or platform (which should never reimplement cryptographic primitives). This design philosophy — minimal trusted core, interchangeable consumers — is documented in detail on the [Architecture](Architecture.md) page.

### Contributions to this project

- Designing the `extern "C"` contract between the Rust core and the consuming languages.
- Defining the lifecycle of the security context (`init` → `encrypt`/`decrypt` → `free`).
- Writing and structuring the technical documentation published in this wiki, aimed both at technical readers integrating the library and at readers interested in the architectural decisions behind it.

### Contact and collaboration

For technical questions about the project, collaboration proposals, or feedback on the documentation, the author can be reached at:

**contact@agentflow-enterprise.com**

To report a security vulnerability, follow the procedure described in `SECURITY.md`, through the private reporting channel (GitHub Security Advisories), rather than through direct contact or public channels.

---

*This page is part of the public documentation of the `security-core-ffi` project and is maintained alongside the rest of the wiki.*

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
