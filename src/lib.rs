//! # Security Core FFI
//!
//! A cryptographic security core (AES-256-GCM), written in Rust for memory
//! safety, exposed through an `extern "C"` interface for integration into
//! any ecosystem (C++, Python, Node.js, WebAssembly, Julia, etc.).
//!
//! Author: **Ciprian Ștefan Pleșca**

use aes_gcm::Aes256Gcm;

/// Opaque structure holding the security context (key + cipher) in memory.
pub struct SecurityContext {
    cipher: Aes256Gcm,
}

// ---------------------------------------------------------------------------
// Native FFI interface (C/C++/Python/Julia bindings)
//
// Only compiled for non-WebAssembly targets: `libc::c_uchar` and `size_t`
// are part of the native C interop layer and are not meaningful on
// `wasm32-unknown-unknown`, which has no C runtime.
// ---------------------------------------------------------------------------
#[cfg(not(target_arch = "wasm32"))]
mod native_ffi {
    use super::SecurityContext;
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Key,
    };
    use libc::{c_uchar, size_t};
    use std::ptr;
    use std::slice;

    /// Initializes a security context from a 32-byte (256-bit) key.
    ///
    /// # Safety
    /// Returns `null` if the pointer is null or the key length is not 32.
    #[no_mangle]
    pub extern "C" fn init_security_context(
        key_ptr: *const c_uchar,
        key_len: size_t,
    ) -> *mut SecurityContext {
        if key_ptr.is_null() || key_len != 32 {
            return ptr::null_mut();
        }

        let key_slice = unsafe { slice::from_raw_parts(key_ptr, key_len) };
        let key = Key::<Aes256Gcm>::from_slice(key_slice);
        let cipher = Aes256Gcm::new(key);

        Box::into_raw(Box::new(SecurityContext { cipher }))
    }

    /// Encrypts a payload using AES-256-GCM. A random nonce (12 bytes) is
    /// generated for every call and prepended to the ciphertext.
    ///
    /// Resulting layout: `[nonce (12B)] || [ciphertext || tag]`
    #[no_mangle]
    pub extern "C" fn encrypt_payload(
        ctx: *mut SecurityContext,
        data_ptr: *const c_uchar,
        data_len: size_t,
        out_len: *mut size_t,
    ) -> *mut c_uchar {
        if ctx.is_null() || data_ptr.is_null() {
            return ptr::null_mut();
        }

        let context = unsafe { &*ctx };
        let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        match context.cipher.encrypt(&nonce, data) {
            Ok(mut ciphertext) => {
                let mut final_payload = nonce.to_vec();
                final_payload.append(&mut ciphertext);

                unsafe { *out_len = final_payload.len() };

                let mut boxed_slice = final_payload.into_boxed_slice();
                let ptr = boxed_slice.as_mut_ptr();
                std::mem::forget(boxed_slice);
                ptr
            }
            Err(_) => ptr::null_mut(),
        }
    }

    /// Decrypts a payload produced by `encrypt_payload`.
    /// Expects the layout `[nonce (12B)] || [ciphertext || tag]`.
    #[no_mangle]
    pub extern "C" fn decrypt_payload(
        ctx: *mut SecurityContext,
        data_ptr: *const c_uchar,
        data_len: size_t,
        out_len: *mut size_t,
    ) -> *mut c_uchar {
        if ctx.is_null() || data_ptr.is_null() || data_len < 12 {
            return ptr::null_mut();
        }

        let context = unsafe { &*ctx };
        let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);

        match context.cipher.decrypt(nonce, ciphertext) {
            Ok(plaintext) => {
                unsafe { *out_len = plaintext.len() };
                let mut boxed_slice = plaintext.into_boxed_slice();
                let ptr = boxed_slice.as_mut_ptr();
                std::mem::forget(boxed_slice);
                ptr
            }
            Err(_) => ptr::null_mut(),
        }
    }

    /// Frees memory allocated by `encrypt_payload` / `decrypt_payload`.
    #[no_mangle]
    pub extern "C" fn free_buffer(ptr: *mut c_uchar, len: size_t) {
        if !ptr.is_null() {
            unsafe {
                let _ = Vec::from_raw_parts(ptr, len, len);
            }
        }
    }

    /// Frees the memory of a security context.
    #[no_mangle]
    pub extern "C" fn free_security_context(ctx: *mut SecurityContext) {
        if !ctx.is_null() {
            unsafe {
                let _ = Box::from_raw(ctx);
            }
        }
    }
}

// Re-export the native FFI functions at the crate root, so existing callers
// (tests/integration_test.rs, examples/*.rs, and the compiled cdylib symbols
// consumed by the C++/Python/Node.js bindings) keep working exactly as
// before — this module split is an internal reorganization, not a public
// API change.
#[cfg(not(target_arch = "wasm32"))]
pub use native_ffi::{
    decrypt_payload, encrypt_payload, free_buffer, free_security_context, init_security_context,
};

// ---------------------------------------------------------------------------
// WebAssembly interface (wasm-bindgen)
//
// Exposes the same security core directly to JavaScript/TypeScript, for
// client-side use (e.g. a Next.js component). Compiled separately from the
// native FFI path, via `wasm-pack build --target web`.
// ---------------------------------------------------------------------------
#[cfg(target_arch = "wasm32")]
mod wasm {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Key,
    };
    use wasm_bindgen::prelude::*;

    /// WASM wrapper around `SecurityContext`, exposed as a JavaScript object.
    #[wasm_bindgen]
    pub struct SecurityContext {
        cipher: Aes256Gcm,
    }

    #[wasm_bindgen]
    impl SecurityContext {
        /// Creates a new security context from a 32-byte key.
        #[wasm_bindgen(constructor)]
        pub fn new(key: &[u8]) -> Result<SecurityContext, JsValue> {
            if key.len() != 32 {
                return Err(JsValue::from_str("The key must be exactly 32 bytes"));
            }
            let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
            Ok(SecurityContext { cipher })
        }

        /// Encrypts data and returns `nonce || ciphertext || tag`.
        pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, JsValue> {
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let ciphertext = self
                .cipher
                .encrypt(&nonce, data)
                .map_err(|_| JsValue::from_str("Encryption failed"))?;

            let mut out = nonce.to_vec();
            out.extend_from_slice(&ciphertext);
            Ok(out)
        }

        /// Decrypts a payload produced by `encrypt` (`nonce || ciphertext || tag`).
        pub fn decrypt(&self, payload: &[u8]) -> Result<Vec<u8>, JsValue> {
            if payload.len() < 12 {
                return Err(JsValue::from_str("Invalid payload: too short"));
            }
            let (nonce_bytes, ciphertext) = payload.split_at(12);
            let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);

            self.cipher
                .decrypt(nonce, ciphertext)
                .map_err(|_| JsValue::from_str("Decryption failed: corrupted data or wrong key"))
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;
    use libc::size_t;
    use std::slice;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0x01u8; 32];
        let ctx = init_security_context(key.as_ptr(), key.len());
        assert!(!ctx.is_null());

        let msg = b"test message";
        let mut out_len: size_t = 0;
        let enc = encrypt_payload(ctx, msg.as_ptr(), msg.len(), &mut out_len);
        assert!(!enc.is_null());

        let mut dec_len: size_t = 0;
        let dec = decrypt_payload(ctx, enc, out_len, &mut dec_len);
        assert!(!dec.is_null());

        let dec_slice = unsafe { slice::from_raw_parts(dec, dec_len) };
        assert_eq!(dec_slice, msg);

        free_buffer(enc, out_len);
        free_buffer(dec, dec_len);
        free_security_context(ctx);
    }
}
