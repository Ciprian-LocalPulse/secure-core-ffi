//! Integration tests for `security_core`.
//! Run with: `cargo test --test integration_test`

use security_core::{
    decrypt_payload, encrypt_payload, free_buffer, free_security_context,
    init_security_context,
};
use std::slice;

#[test]
fn roundtrip_encrypt_decrypt_returns_same_message() {
    let key = [0x42u8; 32];
    let ctx = init_security_context(key.as_ptr(), key.len());
    assert!(!ctx.is_null(), "context must initialize with a valid key");

    let message = b"End-to-end integration: confidential data";
    let mut enc_len: usize = 0;
    let enc_ptr = encrypt_payload(ctx, message.as_ptr(), message.len(), &mut enc_len);
    assert!(!enc_ptr.is_null());
    assert!(enc_len > message.len(), "the encrypted payload must include nonce + tag");

    let mut dec_len: usize = 0;
    let dec_ptr = decrypt_payload(ctx, enc_ptr, enc_len, &mut dec_len);
    assert!(!dec_ptr.is_null());

    let dec_slice = unsafe { slice::from_raw_parts(dec_ptr, dec_len) };
    assert_eq!(dec_slice, message);

    free_buffer(enc_ptr, enc_len);
    free_buffer(dec_ptr, dec_len);
    free_security_context(ctx);
}

#[test]
fn invalid_key_returns_null_context() {
    let short_key = [0u8; 16]; // wrong length (must be 32)
    let ctx = init_security_context(short_key.as_ptr(), short_key.len());
    assert!(ctx.is_null());
}

#[test]
fn decryption_of_corrupted_data_fails_gracefully() {
    let key = [0x07u8; 32];
    let ctx = init_security_context(key.as_ptr(), key.len());
    assert!(!ctx.is_null());

    let corrupted_payload = [0u8; 40]; // fabricated nonce + "ciphertext", invalid tag
    let mut out_len: usize = 0;
    let ptr = decrypt_payload(ctx, corrupted_payload.as_ptr(), corrupted_payload.len(), &mut out_len);
    assert!(ptr.is_null(), "decryption must fail gracefully for corrupted data");

    free_security_context(ctx);
}
