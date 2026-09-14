# Changelog

🇬🇧 [Read in English](CHANGELOG.md)


Toate schimbările notabile ale acestui proiect sunt documentate în acest fișier.

Formatul se bazează pe [Keep a Changelog](https://keepachangelog.com/ro/1.0.0/),
iar proiectul respectă [Semantic Versioning](https://semver.org/lang/ro/).

## [Unreleased]

### Planificat
- Derivare de chei (Argon2 / HKDF)
- Binding oficial Julia
- Audit criptografic independent

### Finalizat
- Header C oficial în `include/security_core.h`
- Teste de integrare pentru binding-urile Node.js și Python ctypes
- Țintă `cargo-fuzz` pentru payload-uri de decriptare malformate
- Metadate de pachet Python pentru binding-ul ctypes
- Publicare publică a pachetului WASM npm prin CI cu provenance

## [0.3.15] - 2026-09-14

### Adăugat
- Manifest de checksum-uri și metadate Cargo atașate release-urilor GitHub.
- Politică de compatibilitate, instrucțiuni pentru integritatea release-urilor, ghid de contribuții, politică de suport și documentație pentru pachetul Node.js.

### Modificat
- Referințe corectate pentru WebAssembly și binding-ul Node.js în documentația engleză și română.

## [0.3.13] - 2026-09-13

### Adăugat
- Model de amenințări, ghid de gestionare a cheilor, checklist de pregătire pentru release și documentație de răspuns la incidente.
- Sincronizarea metadatelor versiunii în Rust, Python, Node.js, WebAssembly, README și pagina de lansare.

### Documentație
- Clarificarea faptului că biblioteca este o primitivă de criptare autentificată și nu oferă stocare, rotație de chei, certificare sau audit independent.

## [0.3.14] - 2026-09-13

### Adăugat
- Publicarea automată a binding-ului Node.js, pentru sincronizarea distribuției npm cu pachetele Rust, Python și WebAssembly.

## [0.3.0] - 2026-09-12

### Adăugat
- Header-ul oficial `include/security_core.h` pentru consumatori C și C++.
- Teste de integrare Node.js și script npm de testare cu `koffi`.
- Metadate de pachet Python ctypes și teste de integrare.
- Țintă `cargo-fuzz` pentru payload-uri de decriptare malformate.
- CI pentru Rust nativ, Clippy, formatare, Node.js, Python ctypes, PyO3 și WASM.

### Corectat
- Validarea pointerilor și lungimilor FFI a fost întărită, iar contractele ABI unsafe sunt documentate.

## [0.2.0] - 2026-09-12

### Adăugat
- Modul `wasm-bindgen` pentru compilare țintă `wasm32`, expunând `SecurityContext` (constructor, `encrypt`, `decrypt`) direct în JavaScript/TypeScript
- Ghid și exemplu de integrare WebAssembly în Next.js (`bindings/wasm-nextjs/`), incluzând configurare `next.config.js` și componentă React
- Folder `tests/integration_test.rs` cu teste de integrare (roundtrip, cheie invalidă, date corupte)
- Folder `examples/` cu programe Rust independente (`basic_usage.rs`, `multi_message.rs`)
- `Makefile` cu comenzi centralizate de build/test pentru nucleu nativ, Wasm, Python și Node.js
- `bindings/python/requirements.txt` și `bindings/nodejs/package.json`
- Folder `assets/` cu schema vizuală a fluxului Rust → WebAssembly → Next.js
- Job CI dedicat pentru build WebAssembly (`wasm-pack build --target web`)
- Documentație bilingvă completă (engleză implicit, română prin fișiere `*.ro.md` și `docs/ro/`)
- Set extins de documente: `WHITEPAPER.md`, `MANIFESTO.md`, `DONATE.md`, `CODE_OF_CONDUCT.md`
- `.github/workflows/ci.yml` — build și test automate la fiecare push/PR (nativ + WebAssembly)
- `.gitignore` care acoperă artefactele de build Rust, output Wasm, fișiere locale Node.js și Python

### Corectat
- Adăugat fișierul `.gitignore`, care lipsea anterior, pentru a preveni includerea accidentală a artefactelor de build în commit-uri

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
