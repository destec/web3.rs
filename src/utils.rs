use hex;
use sha3::{Sha3_256, Digest};

pub fn sha3(str: &[u8]) {
    let mut hasher = Sha3_256::default();
    hasher.input(str);
    let out = hasher.result();
    println!("{:x}", out);
}

pub fn to_hex(source: String) -> String {
    hex::encode(source)
}