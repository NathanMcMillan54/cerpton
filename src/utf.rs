// Nathan McMillan (2023)
use encoding_rs::mem::{
    convert_str_to_utf16, convert_utf16_to_str, convert_utf16_to_utf8, convert_utf8_to_utf16,
};

/// Converts a ``String`` of text to UTF16 bytes
pub fn string_to_utf16(string: String) -> Vec<u16> {
    let mut utf16: Vec<u16> = vec![0; string.len()];

    convert_str_to_utf16(string.as_str(), &mut utf16);

    utf16
}

/// Converts a ``String`` of text to UTF8.
/// This function calls ``string_to_utf16`` and converts it's result into UTF8
pub fn string_to_utf8(string: String) -> Vec<u8> {
    let utf16 = string_to_utf16(string);
    let mut utf8 = vec![0; utf16.len() * 3];

    convert_utf16_to_utf8(&utf16, &mut utf8);

    utf8
}

/// Converts a ``Vec<u8>`` (UTF8) to ``Vec<u16>`` so it can be turned into a ``String``
/// Returns an empty string if conversion fails
pub fn utf8_to_string(utf8: Vec<u8>) -> String {
    let mut utf16 = vec![0; utf8.len() + 1];

    convert_utf8_to_utf16(&utf8, &mut utf16);

    let mut string = String::from_utf8(vec![0; utf16.len() * 3]).unwrap_or(String::new());

    convert_utf16_to_str(&utf16, &mut string);

    string
}
