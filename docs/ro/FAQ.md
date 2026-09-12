# Întrebări frecvente (FAQ)

*Autor: Ciprian Ștefan Pleșca*

**Î: De ce Rust și nu C/C++ pentru nucleu?**
R: Rust oferă garanții de siguranță a memoriei verificate la compilare (fără buffer overflow, use-after-free, data race), eliminând o clasă întreagă de vulnerabilități tipice modulelor de criptografie scrise în C pur, păstrând totuși performanța nativă — nu există un cost de runtime pentru aceste garanții, spre deosebire de un limbaj cu garbage collector.

**Î: Ce algoritm de criptare este folosit?**
R: AES-256-GCM (AEAD) — criptare autentificată cu chei de 256 de biți, care garantează simultan confidențialitatea datelor (nimeni fără cheie nu poate citi textul original) și integritatea lor (orice modificare a ciphertext-ului este detectată la decriptare, care eșuează explicit în loc să producă date corupte tăcut).

**Î: Cine gestionează rotația cheilor?**
R: Aplicația care integrează biblioteca. Nucleul primește o cheie deja generată sau derivată; nu implementează (încă) un mecanism intern de derivare sau rotație. Această alegere este deliberată, nu o omisiune: politica de rotație variază enorm între aplicații (de la „niciodată" la „la fiecare sesiune"), iar impunerea unei politici fixe în nucleu ar reduce flexibilitatea fără un beneficiu clar de securitate. Vezi roadmap-ul pentru integrarea unei biblioteci de derivare (Argon2/HKDF) ca opțiune, nu ca obligație.

**Î: Este sigur să reutilizez un nonce?**
R: Nu este necesar să vă puneți această întrebare în practică — nucleul generează automat un nonce nou și criptografic aleator pentru fiecare apel `encrypt_payload`, prevenind reutilizarea accidentală. Reutilizarea unui nonce sub aceeași cheie este, în general, catastrofală pentru securitatea AES-GCM (poate duce la recuperarea cheii de autentificare), motiv pentru care biblioteca nu expune deloc opțiunea de a furniza un nonce manual.

**Î: Ce se întâmplă dacă payload-ul de decriptat a fost trunchiat sau corupt parțial?**
R: `decrypt_payload` returnează `NULL`. Verificarea tag-ului de autentificare GCM eșuează pentru orice modificare a datelor, indiferent dacă este vorba de corupție accidentală (transmisie eronată) sau de o modificare intenționată — biblioteca nu distinge între cele două cazuri și nu ar trebui să o facă, din motivele explicate în [Referința API](Referinta-API.md).

**Î: Pot integra biblioteca în WebAssembly?**
R: Da. Versiunea curentă include o interfață `wasm-bindgen`, iar workflow-ul CI compilează ținta WebAssembly. Consultă `bindings/wasm-nextjs/README.md` pentru integrarea în browser și Next.js.

**Î: Cum raportez o vulnerabilitate?**
R: Consultă `SECURITY.md` — raportare privată prin GitHub Security Advisories, nu prin contact direct sau canale publice, pentru a permite remedierea înainte de dezvăluire.

**Î: Biblioteca este auditată independent?**
R: La momentul redactării acestei pagini, nu există un audit extern publicat. Absența unui audit nu înseamnă absența calității, dar este o informație relevantă pentru orice echipă care evaluează biblioteca pentru un sistem critic din punct de vedere al securității — tratați acest aspect ca pe o limitare cunoscută, nu ca pe un detaliu de nișă.

**Î: De ce API-ul FFI expune doar cinci funcții?**
R: Pentru a reduce suprafața de atac a contractului și pentru a limita numărul de moduri în care un binding poate greși integrarea. Detaliile motivației sunt discutate pe larg în secțiunea de principii de design din [Arhitectură](Arhitectura.md).

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
