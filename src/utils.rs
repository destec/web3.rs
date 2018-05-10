use sha3::{Sha3_256, Digest};

pub fn sha3(&str) {
    let mut hasher = Sha3_256::default();
    hasher.input(&str);
    let out = hasher.result();
    println!("{:x}", out);
}