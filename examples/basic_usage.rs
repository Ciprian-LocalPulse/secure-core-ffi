//! Native Rust example (no intermediate FFI) using the security core directly.
//! Run with: `cargo run --example basic_usage`

use security_core::{
    decrypt_payload, encrypt_payload, free_buffer, free_security_context, init_security_context,
};
use std::slice;

fn main() {
    let key = [0x11u8; 32]; // in a real scenario: generated with a CSPRNG
    let message = b"Example of direct usage of the security core";

    unsafe {
        let ctx = init_security_context(key.as_ptr(), key.len());
        assert!(!ctx.is_null(), "initialization failed");

        let mut enc_len = 0usize;
        let enc_ptr = encrypt_payload(ctx, message.as_ptr(), message.len(), &mut enc_len);
        assert!(!enc_ptr.is_null());
        println!("Encrypted message ({} bytes): {:x?}", enc_len, slice::from_raw_parts(enc_ptr, enc_len));

        let mut dec_len = 0usize;
        let dec_ptr = decrypt_payload(ctx, enc_ptr, enc_len, &mut dec_len);
        assert!(!dec_ptr.is_null());
        let dec_text = String::from_utf8_lossy(slice::from_raw_parts(dec_ptr, dec_len));
        println!("Decrypted message: {}", dec_text);

        free_buffer(enc_ptr, enc_len);
        free_buffer(dec_ptr, dec_len);
        free_security_context(ctx);
    }
}
