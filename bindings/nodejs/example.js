/**
 * Quick manual test for the koffi-based Node.js bindings.
 * Run from bindings/nodejs after `npm install` and after building the
 * native library with `cargo build --release`.
 */
const { SecurityCore } = require("./security_core");

const key = Buffer.alloc(32, 1); // demo key: 32 bytes of 0x01
const core = new SecurityCore(key);

const plaintext = Buffer.from("mesaj de test");
const ciphertext = core.encrypt(plaintext);
const decrypted = core.decrypt(ciphertext);

console.log("Plaintext original:", plaintext.toString());
console.log("Decriptat:         ", decrypted.toString());
console.log("Roundtrip OK:", plaintext.equals(decrypted));

core.close();
