use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use chacha20poly1305::aead::{Aead, KeyInit};

use crate::random::gen_vec;

pub fn encrypt(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let the_key = Key::from_slice(&key);

    let random = gen_vec(24);
    let the_vec = XNonce::from_slice(&random);

    let encrypter = XChaCha20Poly1305::new(&the_key);
    let encrypted = encrypter
        .encrypt(&the_vec, data.as_ref())
        .unwrap();

    let mut result = random.clone();
    result.extend_from_slice(&encrypted);

    result
}

pub fn decrypt(data: &[u8], key: &Vec<u8>) -> Vec<u8> {
    let the_key = Key::from_slice(&key);

    let (vec, encrypted) = data.split_at(24);
    let the_vec = XNonce::from_slice(vec);

    let decrypter = XChaCha20Poly1305::new(&the_key);

    decrypter
        .decrypt(&the_vec, encrypted)
        .unwrap()
}