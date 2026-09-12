#![no_main]

use libfuzzer_sys::fuzz_target;
use security_core::SecurityContext;

fuzz_target!(|payload: Vec<u8>| {
    let key = [0x42u8; 32];
    let context = SecurityContext::new(&key).expect("fixed-size key is valid");
    let _ = context.decrypt(&payload);
});
