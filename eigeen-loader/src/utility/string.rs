/// Convert a string to a UTF16-LE byte vector.
///
/// No \0 terminator.
pub fn to_wstring_bytes(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// Convert a string to a UTF16-LE byte vector.
///
/// With \0 terminator.
pub fn to_wstring_bytes_with_nul(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
