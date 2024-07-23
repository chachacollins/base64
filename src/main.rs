use std::{env, ops::Index};

const ENCODE_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

struct Base64 {
    char: Vec<char>,
}

impl Index<u8> for Base64 {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        &self.char[index as usize]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let bytes = input.as_bytes();
    let encoded = encode_base64(bytes);
    println!("{}", encoded);
    println!("{:?}", bytes);
}

fn encode_base64(bytes: &[u8]) -> String {
    let mut encoded = String::new();
    for &byte in bytes {
        encoded.push_str(ENCODE_CHARS[(byte >> 2) & 0b1111]);
        encoded.push_str(ENCODE_CHARS[((byte << 4) & 0b11110000) | (byte >> 4)]);
        encoded.push_str(ENCODE_CHARS[((byte << 2) & 0b11000000) | (byte >> 6)]);
        encoded.push_str(ENCODE_CHARS[byte & 0b00111111]);
    }
    encoded
}
