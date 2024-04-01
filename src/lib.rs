// By Nathan McMillan (2023)
pub use crate::decode::Decoder;
pub use crate::encode::Encoder;

pub mod alphabet;
pub mod decode;
pub mod encode;
pub mod utf;

#[cfg(test)]
pub mod tests;

#[no_mangle]
pub extern "C" fn libcerpton_decode(info: [i32; 6], decode: &'static str) -> &'static str {
    let mut decoder = Decoder::new(info[0], info[1]);

    decoder.set_decoder();

    let mut decoded_text = decoder.decode(decode.to_string());

    if info[2] > 1 {
        for _ in 0..info[2] {
            decoded_text = decoder.decode(decoded_text);
        }
        return Box::leak(decoded_text.into_boxed_str());
    } else {
        Box::leak(decoded_text.into_boxed_str())
    }
}

#[no_mangle]
pub extern "C" fn libcerpton_encode(info: [i32; 6], encode: &'static str) -> &'static str {
    let mut encoder = Encoder::new(info[0], info[1]);

    encoder.set_alphabet();

    let mut encoded_text = encoder.encode(encode.to_string());

    if info[2] > 1 {
        for _ in 0..info[2] {
            encoded_text = encoder.encode(encoded_text);
        }

        return Box::leak(encoded_text.into_boxed_str());
    } else {
        Box::leak(encoded_text.into_boxed_str())
    }
}
