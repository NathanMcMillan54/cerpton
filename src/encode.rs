// By Nathan McMillan (2023)
use crate::alphabet::*;

/// Cerpton encoder
pub struct Encoder {
    s1: i32,
    s2: i32,
    pub values: Vec<i32>,
    index: usize,
    first_loop: bool,
    pub(crate) alphabet: Vec<Alphabet>,
}

impl Encoder {
    /// Will panic if ``s1`` and ``s2`` are the same values
    pub fn new(s1: i32, s2: i32) -> Self {
        if s1 == s2 || s1 <= 0 || s2 <= 0 {
            panic!("Both setting digits can't be the same or <= 0 ({}, {})", s1, s2);
        }

        let mut unused_values = vec![];

        for v in 0..ALPHABET_LEN + 1 {
            unused_values.push(v);
        }

        return Encoder {
            s1: s1,
            s2: s2,
            values: unused_values,
            index: 0,
            first_loop: true,
            alphabet: vec![],
        };
    }

    /// Resets the values in the encoder then calls ``set_alphabet``
    pub fn change_setting(&mut self, s1: i32, s2: i32) {
        self.s1 = s1;
        self.s2 = s2;

        self.index = 0;
        self.first_loop = true;
        self.alphabet = vec![];
        self.values = vec![];

        for v in 0..ALPHABET_LEN + 1 {
            self.values.push(v);
        }

        self.set_alphabet();
    }

    /// Checks if the setting (``s1`` ``s2``) makes a valid alphabet
    pub fn setting_good(&self) -> bool {
        for a in &self.alphabet {
            let dup_check = self.is_duplicate(a.1);
            if dup_check.0 {
                return false;
            }
        }

        return true;
    }

    /// Sets the encoder to the default setting, anything that is encoded and decoded will be
    /// turned into plain text.
    pub fn default_laphabet(&mut self) {
        for character in ALPHABET {
            self.alphabet.push((character.0, character.1, false));
        }
    }

    pub fn done(&self) -> bool {
        self.alphabet.iter().all(|character| character.2)
    }

    /// Shuffles the alphabet after the setting is set, must be called after creating a new ``Encoder``
    pub fn set_alphabet(&mut self) {
        self.default_laphabet();

        let mut s1 = self.s1;
        let mut s2 = self.s2;

        while !self.done() {
            self.set_part_alphabet(s1, s2);

            for i in 0..self.alphabet.len() {
                if !self.alphabet[i].2 {
                    s1 = i as i32;
                    s2 = i as i32 + self.s2;

                    if s1 > ALPHABET_LEN {
                        s1 = ALPHABET_LEN
                    }

                    if s2 > ALPHABET_LEN {
                        s2 = ALPHABET_LEN;
                    }

                    break;
                }
            }

            self.first_loop = false;
        }

        for i in 0..self.alphabet.len() {
            let value = self.alphabet[i].1;

            if self.values.is_empty() {
                break;
            }

            let dup_check = self.is_duplicate(value);

            if dup_check.0 {
                self.alphabet[dup_check.1].1 = self.values[0] as u32;
                self.values.remove(0);
            }
        }
    }

    fn is_duplicate(&self, value: u32) -> (bool, usize) {
        let mut first = false;
        let mut second = false;
        let mut index = 0;

        for i in 0..self.alphabet.len() {
            if self.alphabet[i].1 == value {
                if first {
                    second = true;
                    index = i;
                } else {
                    first = true;
                }
            }
        }

        (second, index)
    }

    fn remove_value(&mut self, value: i32) {
        for i in 0..self.values.len() - 1 {
            if self.values[i] == value {
                self.values.remove(i);
            }
        }
    }

    // Sets some characters in the alphabet
    fn set_part_alphabet(&mut self, s1: i32, s2: i32) {
        let mut cn = s1;
        let mut ln = s2;

        while cn < ALPHABET_LEN {
            if cn >= ALPHABET_LEN {
                continue;
            }

            if self.alphabet[self.index].2 {
                self.index += 1;
                continue;
            }

            if self.first_loop {
                self.remove_value(cn);
            } else {
                self.remove_value(cn - 1);
            }

            if self.index >= self.alphabet.len() {
                break;
            }

            if self.first_loop {
                self.alphabet[self.index].1 = cn as u32;
            } else {
                self.alphabet[self.index].1 = cn as u32 - 1;
            }
            self.alphabet[self.index].2 = true;

            let next_cn = cn + ln;
            ln = cn;
            cn = next_cn;
            self.index += 1;
        }
    }

    fn get_char(&self, c: char) -> char {
        for i in 0..self.alphabet.len() {
            if self.alphabet[i].0 == c {
                for character in &ALPHABET {
                    if character.1 == self.alphabet[i].1 {
                        return character.0;
                    }
                }
            }
        }

        return c;
    }

    /// Encodes a string a retunrs it
    pub fn encode(&self, text: String) -> String {
        let mut ret = String::new();

        for c in text.chars() {
            ret.push(self.get_char(c));
        }

        return ret;
    }
}
