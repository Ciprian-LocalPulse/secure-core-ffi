/**
 * Node.js binding for the Rust security core (security_core), using koffi.
 * Cryptographic core author: Ciprian Stefan Plesca
 *
 * koffi loads the precompiled native library directly (no node-gyp build
 * step required), unlike the older ffi-napi/ref-napi stack.
 *
 * Install dependencies: npm install koffi
 */

const koffi = require("koffi");
const path = require("path");

function libFileName() {
  if (process.platform === "win32") return "security_core.dll";
  if (process.platform === "darwin") return "libsecurity_core.dylib";
  return "libsecurity_core.so";
}

// Opaque pointer types
const VoidPtr = koffi.pointer("void");
const UCharPtr = koffi.pointer("uint8_t");
// Marked with koffi.out(): this is a size_t* the native side WRITES INTO.
// Without koffi.out(), koffi treats the pointer as input-only and never
// copies the written value back into the JS array we pass in.
const SizeTOutPtr = koffi.out(koffi.pointer("size_t"));

class SecurityCore {
  constructor(key, libPath) {
    if (!Buffer.isBuffer(key) || key.length !== 32) {
      throw new Error("Cheia trebuie sa fie un Buffer de exact 32 de octeti (256 biti)");
    }

    const resolvedPath = libPath || path.join(__dirname, "..", "..", "target", "release", libFileName());
    const lib = koffi.load(resolvedPath);

    this._init_security_context = lib.func("init_security_context", VoidPtr, [UCharPtr, "size_t"]);
    this._encrypt_payload = lib.func("encrypt_payload", UCharPtr, [VoidPtr, UCharPtr, "size_t", SizeTOutPtr]);
    this._decrypt_payload = lib.func("decrypt_payload", UCharPtr, [VoidPtr, UCharPtr, "size_t", SizeTOutPtr]);
    this._free_buffer = lib.func("free_buffer", "void", [UCharPtr, "size_t"]);
    this._free_security_context = lib.func("free_security_context", "void", [VoidPtr]);

    this.ctx = this._init_security_context(key, key.length);
    if (!this.ctx) {
      throw new Error("Nu s-a putut initializa contextul de securitate (cheie invalida?)");
    }
  }

  encrypt(buffer) {
    const outLen = [0];
    const ptr = this._encrypt_payload(this.ctx, buffer, buffer.length, outLen);
    if (!ptr) {
      throw new Error("Criptarea a esuat in nucleul nativ");
    }
    const len = outLen[0];
    const result = Buffer.from(koffi.decode(ptr, koffi.array("uint8_t", len)));
    this._free_buffer(ptr, len);
    return result;
  }

  decrypt(buffer) {
    const outLen = [0];
    const ptr = this._decrypt_payload(this.ctx, buffer, buffer.length, outLen);
    if (!ptr) {
      throw new Error("Decriptarea a esuat: cheie gresita sau date corupte");
    }
    const len = outLen[0];
    const result = Buffer.from(koffi.decode(ptr, koffi.array("uint8_t", len)));
    this._free_buffer(ptr, len);
    return result;
  }

  close() {
    if (this.ctx) {
      this._free_security_context(this.ctx);
      this.ctx = null;
    }
  }
}

module.exports = { SecurityCore };
