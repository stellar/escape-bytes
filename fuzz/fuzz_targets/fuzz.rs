#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &[u8]| {
    let escaped = escape_bytes::escape(input);
    let unescaped = escape_bytes::unescape(&escaped);

    // Roundtripped to same.
    assert_eq!(unescaped, Ok(input.to_vec()));

    // Escaped length consistency.
    let escaped_len = escape_bytes::escaped_len(input);
    assert_eq!(escaped_len, escaped.len());

    // Unescaped length consistency.
    let unescaped_len = escape_bytes::unescaped_len(escaped);
    assert_eq!(unescaped_len, Ok(unescaped.unwrap().len()));
});
