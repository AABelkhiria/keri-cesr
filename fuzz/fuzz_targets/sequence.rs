#![no_main]
#![forbid(unsafe_code)]

use keri_cesr::{bytes::utf8_text, sequence::SequenceNumber};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = SequenceNumber::parse_raw_prefix(data);
    let _ = SequenceNumber::parse_qb2(data);
    if let Ok(text) = utf8_text(data) {
        let _ = SequenceNumber::parse_qb64(text);
        let _ = SequenceNumber::from_hex(text);
    }
});
