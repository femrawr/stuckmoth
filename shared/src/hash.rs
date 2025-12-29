use sha2::{Digest, Sha512_256};

pub fn hash_key(key: &[u8], salt: &Vec<u8>) -> Vec<u8> {
    let mut sha512 = Sha512_256::new();
    sha512.update(key);
    sha512.update(salt);

    sha512
        .finalize()
        .to_vec()
}