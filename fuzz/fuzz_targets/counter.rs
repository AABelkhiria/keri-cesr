#![no_main]
#![forbid(unsafe_code)]

use keri_cesr::{bytes::utf8_text, counter::Counter};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Counter::parse_qb2(data);
    if let Ok(text) = utf8_text(data) {
        let _ = Counter::parse_qb64(text);
    }
});
