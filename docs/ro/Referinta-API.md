# Referință API (FFI)

*Autor: Ciprian Ștefan Pleșca*

Această pagină documentează, funcție cu funcție, întregul contract public al nucleului. Toate cele cinci funcții fac parte din suprafața stabilă a bibliotecii — orice modificare a semnăturii lor constituie o schimbare incompatibilă (breaking change) și necesită o versiune majoră nouă.

## `init_security_context`

```c
SecurityContext* init_security_context(const unsigned char* key_ptr, size_t key_len);
```

Inițializează un context de securitate cu o cheie AES-256 (32 octeți).
Returnează `NULL` dacă `key_ptr` e nul sau `key_len != 32`.

**Cost:** O(1) relativ la dimensiunea cheii — costul dominant este construirea structurii interne `Aes256Gcm`, nu o operație costisitoare din punct de vedere criptografic.
**Apelat:** o singură dată per sesiune de utilizare a contextului.

## `encrypt_payload`

```c
unsigned char* encrypt_payload(SecurityContext* ctx, const unsigned char* data_ptr, size_t data_len, size_t* out_len);
```

Criptează `data_ptr` (lungime `data_len`) folosind AES-256-GCM.
Rezultatul are formatul `nonce (12B) || ciphertext || tag (16B)`, cu lungimea totală scrisă în `out_len`.

**Observație:** lungimea rezultatului este întotdeauna `data_len + 28` octeți (12 pentru nonce, 16 pentru tag), ciphertext-ul propriu-zis având aceeași lungime ca textul original — GCM este un mod „stream-like", fără padding.
**Nonce:** generat automat, unic per apel, prin generatorul criptografic al sistemului de operare (`OsRng`); apelantul nu trebuie și nu poate furniza propriul nonce.

## `decrypt_payload`

```c
unsigned char* decrypt_payload(SecurityContext* ctx, const unsigned char* data_ptr, size_t data_len, size_t* out_len);
```

Decriptează un payload produs de `encrypt_payload`. Returnează `NULL` dacă autentificarea GCM eșuează (date corupte sau cheie greșită).

**Important:** un `NULL` la decriptare nu distinge între „date corupte accidental" și „date modificate adversarial" — și este intenționat așa. Dacă biblioteca ar oferi informații diferite pentru cele două cazuri, ar deschide un canal lateral (oracle) exploatabil de un atacator care trimite payload-uri modificate și observă diferența de comportament.

## `free_buffer`

```c
void free_buffer(unsigned char* ptr, size_t len);
```

Eliberează un buffer alocat de `encrypt_payload` / `decrypt_payload`. **Obligatoriu** de apelat pentru fiecare buffer primit, altfel apare memory leak.

**De ce este necesar un apel explicit:** memoria buffer-ului este alocată în interiorul Rust și predată apelantului prin `Box::into_raw`; din perspectiva Rust, ownership-ul a fost transferat, deci allocator-ul Rust nu o va elibera automat. `free_buffer` este singurul mod corect de a o returna alocatorului, indiferent din ce limbaj este apelată biblioteca.

## `free_security_context`

```c
void free_security_context(SecurityContext* ctx);
```

Eliberează contextul de securitate (inclusiv cheia din memorie).

**Notă de securitate:** eliberarea contextului distruge și copia cheii păstrate intern; totuși, ștergerea garantată din memorie (zeroing explicit înainte de eliberare, pentru a preveni recuperarea cheii dintr-un dump de memorie) depinde de implementare — consultați codul sursă al versiunii instalate dacă acest detaliu este critic pentru modelul dumneavoastră de amenințare.

## Tabel sumar

| Funcție | Intrare | Ieșire | Note |
|---|---|---|---|
| `init_security_context` | cheie 32B | `SecurityContext*` sau `NULL` | apelată o dată per sesiune |
| `encrypt_payload` | context + plaintext | `nonce\|\|ciphertext\|\|tag` | nonce unic per apel |
| `decrypt_payload` | context + payload criptat | plaintext sau `NULL` | verifică tag-ul GCM |
| `free_buffer` | pointer + lungime | — | eliberează un buffer returnat |
| `free_security_context` | context | — | eliberează cheia din memorie |

## Reguli generale de utilizare a memoriei

1. Orice pointer returnat de `encrypt_payload` sau `decrypt_payload` trebuie eliberat exact o dată, prin `free_buffer`.
2. Orice pointer `SecurityContext*` returnat de `init_security_context` trebuie eliberat exact o dată, prin `free_security_context`.
3. Niciun pointer nu trebuie folosit după ce a fost eliberat (use-after-free) și niciun pointer nu trebuie eliberat de două ori (double-free) — ambele reguli sunt garantate automat de Rust *în interiorul* nucleului, dar nu pot fi garantate automat *dincolo* de granița FFI, motiv pentru care fiecare binding trebuie să le respecte explicit (vezi tabelul de erori frecvente din [Ghidul de integrare](Ghid-de-Integrare.md)).

---

<div align="center">
<sub>

[Home](Home.md) · [Arhitectură](Arhitectura.md) · [Ghid de Integrare](Ghid-de-Integrare.md) · [Referință API](Referinta-API.md) · [FAQ](FAQ.md) · [Autor](Autor.md)

</sub>

<img src="https://img.shields.io/badge/AES--256--GCM-criptare%20autentificată-1f6feb?style=flat-square" alt="AES-256-GCM"/>
<img src="https://img.shields.io/badge/nucleu-Rust-orange?style=flat-square&logo=rust" alt="Nucleu Rust"/>
<img src="https://img.shields.io/badge/interfață-C%20FFI-6e5494?style=flat-square" alt="C FFI"/>
<img src="https://img.shields.io/badge/bindinguri-C%2B%2B%20%7C%20Python%20%7C%20Node.js-2ea043?style=flat-square" alt="Bindinguri"/>

<sub>
<b>security-core-ffi</b> — arhitectură &amp; documentație de <a href="Autor.md"><b>Ciprian Ștefan Pleșca</b></a><br/>
Cercetător independent · <a href="mailto:contact@agentflow-enterprise.com">contact@agentflow-enterprise.com</a>
</sub>

<sub>
Ai găsit o problemă în documentație? Deschide o discuție sau contactează direct autorul.<br/>
Pentru raportarea unei vulnerabilități de securitate, folosește <b>SECURITY.md</b> prin GitHub Security Advisories — nu această adresă de contact.
</sub>

<sub>© 2026 Ciprian Ștefan Pleșca. Documentație menținută alături de proiectul <code>security-core-ffi</code>.</sub>

</div>
