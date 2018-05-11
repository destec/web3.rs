extern crate web3rs;
// use web3rs::{utils};

#[cfg(test)]
mod utils_test {
    // use super::utils;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn to_hex() {
        assert_eq!(web3rs::utils::to_hex("234"), "0xea");
    }
}
