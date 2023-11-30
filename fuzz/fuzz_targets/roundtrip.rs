#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &[u8]| {
    let escaped = escape_bytes::escape(input);
    let unescaped = escape_bytes::unescape(&escaped);
    assert_eq!(unescaped, Ok(input.to_vec()));
});
