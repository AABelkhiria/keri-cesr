#![no_main]
#![forbid(unsafe_code)]

use keri_cesr::{
    bytes::utf8_text,
    code::{DerivationCode, hard_code_size},
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Some(selector) = data.first().copied() {
        let _ = hard_code_size(selector);
    }
    if let Ok(text) = utf8_text(data) {
        let _ = text.parse::<DerivationCode>();
    }
});
