//! Shared AES-256-GCM logic, used by both the native FFI interface and the
//! WebAssembly bindings. This is the single place where encryption actually
//! happens — everything else is just translation to a given ABI.

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use std::fmt;

/// Size, in bytes, of the random nonce prepended to every ciphertext.
pub const NONCE_LEN: usize = 12;

/// Errors that can occur while using a [`SecurityContext`].
#[derive(Debug)]
pub enum CoreError {
    /// The key passed to `SecurityContext::new` was not exactly 32 bytes.
    InvalidKeyLength,
    /// The payload passed to `decrypt` was shorter than the nonce itself,
    /// so it can't possibly contain a valid ciphertext.
    PayloadTooShort,
    /// The underlying AEAD encryption operation failed.
    EncryptionFailed,
    /// The underlying AEAD decryption operation failed (wrong key or
    /// tampered/corrupted data — AES-GCM doesn't distinguish the two).
    DecryptionFailed,
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            CoreError::InvalidKeyLength => "the key must be exactly 32 bytes",
            CoreError::PayloadTooShort => "payload too short to contain a valid nonce",
            CoreError::EncryptionFailed => "encryption failed",
            CoreError::DecryptionFailed => "decryption failed: corrupted data or wrong key",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for CoreError {}

/// Holds the security context (the AES-256-GCM cipher, keyed) in memory.
///
/// This type carries no `unsafe` code and no FFI-specific types, so it can
/// be shared as-is between the native `extern "C"` layer, the WASM bindings,
/// and any pure-Rust consumer (like the Python bindings crate).
pub struct SecurityContext {
    cipher: Aes256Gcm,
}

impl SecurityContext {
    /// Creates a new security context from a 32-byte (256-bit) key.
    pub fn new(key: &[u8]) -> Result<Self, CoreError> {
        if key.len() != 32 {
            return Err(CoreError::InvalidKeyLength);
        }
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        Ok(SecurityContext { cipher })
    }

    /// Encrypts `data` using AES-256-GCM. A random nonce is generated for
    /// every call and prepended to the ciphertext.
    ///
    /// Resulting layout: `[nonce (12B)] || [ciphertext || tag]`
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CoreError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, data)
            .map_err(|_| CoreError::EncryptionFailed)?;

        let mut out = nonce.to_vec();
        out.extend_from_slice(&ciphertext);
        Ok(out)
    }

    /// Decrypts a payload produced by `encrypt`.
    /// Expects the layout `[nonce (12B)] || [ciphertext || tag]`.
    pub fn decrypt(&self, payload: &[u8]) -> Result<Vec<u8>, CoreError> {
        if payload.len() < NONCE_LEN {
            return Err(CoreError::PayloadTooShort);
        }
        let (nonce_bytes, ciphertext) = payload.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);

        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CoreError::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let ctx = SecurityContext::new(&[0x01u8; 32]).unwrap();
        let msg = b"test message";

        let enc = ctx.encrypt(msg).unwrap();
        let dec = ctx.decrypt(&enc).unwrap();

        assert_eq!(dec, msg);
    }

    #[test]
    fn rejects_bad_key_length() {
        assert!(matches!(
            SecurityContext::new(&[0u8; 16]),
            Err(CoreError::InvalidKeyLength)
        ));
    }

    #[test]
    fn rejects_short_payload() {
        let ctx = SecurityContext::new(&[0x01u8; 32]).unwrap();
        assert!(matches!(
            ctx.decrypt(&[0u8; 4]),
            Err(CoreError::PayloadTooShort)
        ));
    }
}
