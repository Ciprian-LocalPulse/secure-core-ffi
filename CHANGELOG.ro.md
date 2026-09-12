# Changelog

🇬🇧 [Read in English](CHANGELOG.md)


Toate schimbările notabile ale acestui proiect sunt documentate în acest fișier.

Formatul se bazează pe [Keep a Changelog](https://keepachangelog.com/ro/1.0.0/),
iar proiectul respectă [Semantic Versioning](https://semver.org/lang/ro/).

## [Unreleased]

### Adăugat
- Modul `wasm-bindgen` pentru compilare țintă `wasm32`, expunând `SecurityContext` (constructor, `encrypt`, `decrypt`) direct în JavaScript/TypeScript
- Ghid și exemplu de integrare WebAssembly în Next.js (`bindings/wasm-nextjs/`), incluzând configurare `next.config.js` și componentă React
- Folder `tests/integration_test.rs` cu teste de integrare (roundtrip, cheie invalidă, date corupte)
- Folder `examples/` cu programe Rust independente (`basic_usage.rs`, `multi_message.rs`)
- `Makefile` cu comenzi centralizate de build/test pentru nucleu nativ, Wasm, Python și Node.js
- `bindings/python/requirements.txt` și `bindings/nodejs/package.json`
- Folder `assets/` cu schema vizuală a fluxului Rust → WebAssembly → Next.js
- Job CI dedicat pentru build WebAssembly (`wasm-pack build --target web`)
- `.gitignore` extins (artefacte Wasm, `pkg/`, biblioteci compilate native)

### Planificat
- Derivare de chei (Argon2 / HKDF)
- Binding oficial Julia
- Fuzzing automat al interfeței FFI (cargo-fuzz)
- Publicare pachet npm pentru modulul Wasm

## [0.1.0] - 2026-09-12

### Adăugat
- Nucleu de securitate în Rust (`src/lib.rs`) implementând AES-256-GCM prin `extern "C"`:
  - `init_security_context`
  - `encrypt_payload`
  - `decrypt_payload`
  - `free_buffer`
  - `free_security_context`
- Binding C++ de exemplu (`bindings/cpp/example.cpp`)
- Binding Python orientat obiect, bazat pe `ctypes` (`bindings/python/security_core.py`)
- Binding Node.js bazat pe `ffi-napi` (`bindings/nodejs/security_core.js`)
- Dockerfile multi-stage pentru build reproductibil
- Suită de teste unitare Rust (roundtrip encrypt/decrypt)
- Documentație README cu diagrame Mermaid (arhitectură, secvență, structură payload)
- Fișiere de conformitate: `LICENSE` (Apache-2.0), `SECURITY.md`, `CITATION.cff`
- Pagini Wiki inițiale (Home, Arhitectură, Ghid de integrare, FAQ)

### Autor
- Arhitectură și cod sursă original conceput de **Ciprian Ștefan Pleșca**
