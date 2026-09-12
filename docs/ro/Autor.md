# Despre autor

<img src="../assets/ciprian-stefan-plesca.jpg" alt="Ciprian Ștefan Pleșca" width="220" />

*Română · [English](../en/Author.md)*


## Ciprian Ștefan Pleșca

**Cercetător independent · Autor și conceptor al arhitecturii/codului `security-core-ffi`**
**Contact:** contact@agentflow-enterprise.com

Ciprian Ștefan Pleșca este autorul și conceptorul arhitecturii din spatele proiectului `security-core-ffi`, un nucleu de criptare autentificată scris în Rust și expus printr-o interfață FFI stabilă, gândit să fie consumat identic din C++, Python, Node.js și alte limbaje care pot lega o bibliotecă dinamică în C.

Lucrarea de față — documentația extinsă a proiectului — reflectă o preocupare centrală a autorului: separarea clară între logica de securitate (care trebuie scrisă o singură dată, corect, într-un limbaj memory-safe) și straturile de integrare specifice fiecărui limbaj sau platformă (care nu ar trebui niciodată să reimplementeze primitive criptografice). Această filozofie de design — nucleu minimal de încredere, consumatori interschimbabili — este documentată în detaliu în pagina de [Arhitectură](Arhitectura.md).

### Contribuții în cadrul acestui proiect

- Proiectarea contractului `extern "C"` dintre nucleul Rust și limbajele consumatoare.
- Definirea ciclului de viață al contextului de securitate (`init` → `encrypt`/`decrypt` → `free`).
- Redactarea și structurarea documentației tehnice publicate în această wiki, adaptată atât pentru cititori tehnici care integrează biblioteca, cât și pentru cititori interesați de deciziile de arhitectură din spatele ei.

### Contact și colaborare

Pentru întrebări tehnice legate de proiect, propuneri de colaborare sau raportarea unor observații privind documentația, autorul poate fi contactat la adresa:

**contact@agentflow-enterprise.com**

Pentru raportarea unei vulnerabilități de securitate, urmați procedura descrisă în `SECURITY.md`, prin canalul privat de raportare (GitHub Security Advisories), și nu prin contact direct sau canale publice.

---

*Această pagină este parte a documentației publice a proiectului `security-core-ffi` și este menținută alături de restul wiki-ului.*

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
