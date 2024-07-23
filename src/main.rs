use std::env;

const ENCODE_CHARS: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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
        encoded.push(ENCODE_CHARS[(byte >> 2) & 0b1111]);
        encoded.push(ENCODE_CHARS[((byte << 4) & 0b11110000) | (byte >> 4)]);
        encoded.push(ENCODE_CHARS[((byte << 2) & 0b11000000) | (byte >> 6)]);
        encoded.push(ENCODE_CHARS[byte & 0b00111111]);
    }
    encoded
}
