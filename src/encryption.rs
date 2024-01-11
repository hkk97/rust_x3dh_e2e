extern crate hex;
use rand::{RngCore, thread_rng};
use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead}; // Add Aead here
use aes_gcm::aead::{generic_array::GenericArray};

pub fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
    let bytes: Vec<u8> = hex_string
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            u8::from_str_radix(std::str::from_utf8(chunk).expect("Invalid hex string"), 16)
                .expect("Invalid hex character")
        })
        .collect();

    if bytes.is_empty() {
        panic!("Hex string conversion resulted in an empty byte vector");
    }
    bytes
}

fn perform_encryption(key: &[u8], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));

    let mut iv = [0u8; 12];
    thread_rng().fill_bytes(&mut iv);
    let nonce = GenericArray::from_slice(&iv);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("Encryption failure!");

    (ciphertext.to_vec(), iv.to_vec())
}

pub fn encrypt_with_string_key(shared_secret_key: String, plaintext: String) -> (Vec<u8>, Vec<u8>) {
    let key = hex_string_to_bytes(&shared_secret_key);
    let plaintext_bytes = plaintext.as_bytes();

    perform_encryption(&key, &plaintext_bytes)
}

pub fn encrypt_with_bytes_key(shared_secret_key: Vec<u8>, plaintext: String) -> (Vec<u8>, Vec<u8>) {
    let key = shared_secret_key;
    let plaintext_bytes = plaintext.as_bytes();

    perform_encryption(&key, &plaintext_bytes)
}

pub fn decrypt(shared_secret_key: Vec<u8>, ciphertext: Vec<u8>, iv: Vec<u8>) -> String {
    let key = GenericArray::from_slice(&shared_secret_key);
    let cipher = Aes256Gcm::new(key);

    let nonce = GenericArray::from_slice(&iv);

    let decrypted_text = cipher.decrypt(nonce, &*ciphertext).expect("Decryption failure!");
    String::from_utf8(decrypted_text).expect("Invalid UTF-8")
}