use rand::Rng;
use rand::distributions::{Alphanumeric, Standard};

pub fn gen_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn gen_vec(len: usize) -> Vec<u8> {
    rand::thread_rng()
        .sample_iter(&Standard)
        .take(len)
        .collect()
}