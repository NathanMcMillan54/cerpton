// Nathan McMillan (2024)
use crate::{libcerpton_encode, Decoder, Encoder};

const LATIN_TEXT: &str = "A quick brown fox jumps over the lazy dog"; // Uses every letter in the latin alphabet
const CYRILLIC_TEXT: &str = "Я П'ю МЕД"; // "I drink honey"
const ARABIC_TEXT: &str = "أنا أكل الحبوب"; // Should be: "I eat cereal" 

const SAMPLE_TEXT: &str = "This is some text!!!";
const SAMPLE_SETTING1: (i32, i32) = (2, 1);
const SAMPLE_SETTING2: (i32, i32) = (11, 7);

// Tests some of the setting values
#[test]
fn test_alphabet_encode() {
    let mut encoder = Encoder::new(SAMPLE_SETTING1.0, SAMPLE_SETTING1.1);
    encoder.set_alphabet();
    assert_eq!(encoder.setting_good(), true);

    let encoded_latin_text = encoder.encode(String::from(LATIN_TEXT));
    let encoded_cyrillic_text = encoder.encode(String::from(CYRILLIC_TEXT));
    let encoded_arabic_text = encoder.encode(String::from(ARABIC_TEXT));

    assert_ne!(LATIN_TEXT, encoded_latin_text);
    assert_ne!(CYRILLIC_TEXT, encoded_cyrillic_text);
    assert_ne!(ARABIC_TEXT, encoded_arabic_text);
}

#[test]
fn decode() {
    let mut encoder = Encoder::new(SAMPLE_SETTING2.0, SAMPLE_SETTING2.1);
    encoder.set_alphabet();
    assert_eq!(encoder.setting_good(), true);

    let encoded_latin_text = encoder.encode(String::from(LATIN_TEXT));
    let encoded_cyrillic_text = encoder.encode(String::from(CYRILLIC_TEXT));
    let encoded_arabic_text = encoder.encode(String::from(ARABIC_TEXT));

    let mut decoder = Decoder::new(SAMPLE_SETTING2.0, SAMPLE_SETTING2.1);
    decoder.set_decoder();
    
    let latin_text = decoder.decode(encoded_latin_text);
    let cyrillic_text = decoder.decode(encoded_cyrillic_text);
    let arabic_text = decoder.decode(encoded_arabic_text);

    assert_eq!(LATIN_TEXT, latin_text);
    assert_eq!(CYRILLIC_TEXT, cyrillic_text);
    assert_eq!(ARABIC_TEXT, arabic_text);
}

#[test]
fn test_multiple_encodes() {
    let mut encoder = Encoder::new(SAMPLE_SETTING1.0, SAMPLE_SETTING1.1);
    encoder.set_alphabet();

    let encoded_text = encoder.encode(SAMPLE_TEXT.to_string());
    let multi_encoded_text = libcerpton_encode([SAMPLE_SETTING1.0, SAMPLE_SETTING1.1, 2, 0, 0, 0], SAMPLE_TEXT.to_string());

    assert_ne!(encoded_text, multi_encoded_text);
}
