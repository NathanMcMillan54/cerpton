// By Nathan McMillan (2023)
use crate::alphabet::ALPHABET;
use crate::Encoder;

/// Used for Decodeing text made from ``Encoder``. This uses some of the same functions ``Encoder`` uses to generate 
/// the ``alphabet``
pub struct Decoder {
    s1: i32,
    s2: i32,
    pub(crate) alphabet: Vec<(char, u32, bool)>,
}

impl Decoder {
    pub fn new(s1: i32, s2: i32) -> Self {
        return Decoder {
            s1: s1,
            s2: s2,
            alphabet: vec![],
        };
    }

    /// Must be called after making a new ``Decoder``, calls ``Encoder.set_alphabet`` to get the alphabet
    pub fn set_decoder(&mut self) {
        let mut encoder = Encoder::new(self.s1, self.s2);
        encoder.set_alphabet();

        self.alphabet = encoder.alphabet;
    }

    fn get_char(&self, c: char) -> char {
        for a in &ALPHABET {
            if c == a.0 {
                for character in &self.alphabet {
                    if character.1 == a.1 {
                        return character.0;
                    }
                }
            }
        }

        return c;
    }

    // Decodeds a peice of text
    pub fn decode(&self, input: String) -> String {
        let mut ret = String::new();

        for c in input.chars() {
            ret.push(self.get_char(c));
        }

        ret
    }
}
