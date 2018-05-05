use sha3::Sha3_256;

fn sha3() {
    let mut hasher = Sha3_256::default();
    hasher.input(b"234");
    let out = hasher.result();
    println!("{:x}", out);
}
