use std::error::Error;

pub const SHIFT: i32 = 3; // Default shift value

pub struct CaesarCipher {
    shift: i32,
}

impl CaesarCipher {
    pub fn new(shift: i32) -> Self {
        Self { shift }
    }

    pub fn encrypt(&self, input: &str) -> String {
        let mut ciphertext = String::new();

        for char in input.chars() {
            match char {
                ' ' => ciphertext.push('*'),
                c if c.is_alphabetic() => {
                    let shift = self.shift % 26;
                    let ascii_value = match c.is_uppercase() {
                        true => 'A' as i32,
                        false => 'a' as i32,
                    };

                    let cipher_val = (ascii_value + (char as i32 - ascii_value + shift) % 26)
                        as u8 as char;
                    ciphertext.push(cipher_val);
                }
                _ => ciphertext.push(char),
            }
        }

        ciphertext
    }

    pub fn decrypt(&self, input: &str) -> String {
        self.encrypt(input)
    }
}