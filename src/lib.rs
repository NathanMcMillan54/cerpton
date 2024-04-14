// By Nathan McMillan (2023)
pub use crate::decode::Decoder;
pub use crate::encode::Encoder;

pub mod alphabet;
pub mod decode;
pub mod encode;
pub mod utf;

#[cfg(test)]
pub mod tests;

pub fn libcerpton_decode(info: [i32; 6], decode: String) -> String {
    let mut decoder = Decoder::new(info[0], info[1]);

    decoder.set_decoder();

    let mut decoded_text = decoder.decode(decode);

    if info[2] > 1 {
        for _ in 0..info[2] {
            decoded_text = decoder.decode(decoded_text);
        }
        return decoded_text;
    } else {
        decoded_text
    }
}

pub fn libcerpton_encode(info: [i32; 6], encode: String) -> String {
    let mut encoder = Encoder::new(info[0], info[1]);

    encoder.set_alphabet();

    let mut encoded_text = encoder.encode(encode);

    if info[2] > 1 {
        for _ in 0..info[2] {
            encoded_text = encoder.encode(encoded_text);
        }

        return encoded_text;
    } else {
        encoded_text
    }
}
