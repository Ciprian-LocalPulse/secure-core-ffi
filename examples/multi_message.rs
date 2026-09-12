//! Example: encrypting multiple messages with the same security context,
//! demonstrating that every call generates a unique nonce (different
//! ciphertexts even for identical messages).
//! Run with: `cargo run --example multi_message`

use security_core::{encrypt_payload, free_buffer, free_security_context, init_security_context};
use std::slice;

fn main() {
    let key = [0x99u8; 32];
    let messages: [&[u8]; 3] = [b"message one", b"message two", b"message one"]; // last two identical

    unsafe {
        let ctx = init_security_context(key.as_ptr(), key.len());
        assert!(!ctx.is_null());

        for (i, message) in messages.iter().enumerate() {
            let mut out_len = 0usize;
            let ptr = encrypt_payload(ctx, message.as_ptr(), message.len(), &mut out_len);
            assert!(!ptr.is_null());

            let ciphertext = slice::from_raw_parts(ptr, out_len);
            println!(
                "Message #{i}: {} encrypted bytes -> {:x?}",
                out_len, ciphertext
            );

            free_buffer(ptr, out_len);
        }

        free_security_context(ctx);
    }

    println!("\nNote: even though messages #0 and #2 are identical in plaintext,");
    println!("the ciphertexts differ, because every call generates a new nonce.");
}
