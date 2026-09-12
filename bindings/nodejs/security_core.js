/**
 * Node.js binding for the Rust security core (security_core), using ffi-napi.
 * Cryptographic core author: Ciprian Stefan Plesca
 *
 * Install dependencies: npm install ffi-napi ref-napi ref-array-napi
 */

const ffi = require("ffi-napi");
const ref = require("ref-napi");
const ArrayType = require("ref-array-napi");

const UCharArray = ArrayType(ref.types.uchar);
const size_t = ref.types.size_t;
const voidPtr = ref.refType(ref.types.void);
const sizeTPtr = ref.refType(size_t);

function libFileName() {
  if (process.platform === "win32") return "security_core.dll";
  if (process.platform === "darwin") return "libsecurity_core.dylib";
  return "libsecurity_core.so";
}

class SecurityCore {
  constructor(key, libPath) {
    if (key.length !== 32) {
      throw new Error("Cheia trebuie sa aiba exact 32 de octeti (256 biti)");
    }

    const path = libPath || `../../target/release/${libFileName()}`;
    this.lib = ffi.Library(path, {
      init_security_context: [voidPtr, [UCharArray, size_t]],
      encrypt_payload: [ref.refType(ref.types.uchar), [voidPtr, UCharArray, size_t, sizeTPtr]],
      decrypt_payload: [ref.refType(ref.types.uchar), [voidPtr, UCharArray, size_t, sizeTPtr]],
      free_buffer: ["void", [ref.refType(ref.types.uchar), size_t]],
      free_security_context: ["void", [voidPtr]],
    });

    const keyBuf = new UCharArray(Array.from(key));
    this.ctx = this.lib.init_security_context(keyBuf, key.length);
    if (this.ctx.isNull()) {
      throw new Error("Nu s-a putut initializa contextul de securitate");
    }
  }

  encrypt(buffer) {
    const dataBuf = new UCharArray(Array.from(buffer));
    const outLen = ref.alloc(size_t);
    const ptr = this.lib.encrypt_payload(this.ctx, dataBuf, buffer.length, outLen);
    const len = outLen.deref();
    const result = Buffer.from(ref.reinterpret(ptr, len));
    this.lib.free_buffer(ptr, len);
    return result;
  }

  decrypt(buffer) {
    const dataBuf = new UCharArray(Array.from(buffer));
    const outLen = ref.alloc(size_t);
    const ptr = this.lib.decrypt_payload(this.ctx, dataBuf, buffer.length, outLen);
    const len = outLen.deref();
    const result = Buffer.from(ref.reinterpret(ptr, len));
    this.lib.free_buffer(ptr, len);
    return result;
  }

  close() {
    if (this.ctx && !this.ctx.isNull()) {
      this.lib.free_security_context(this.ctx);
      this.ctx = ref.NULL;
    }
  }
}

module.exports = { SecurityCore };
