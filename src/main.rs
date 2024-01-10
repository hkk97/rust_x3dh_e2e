extern crate hex;
use rand::{RngCore, thread_rng};
use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead}; // Add Aead here
use aes_gcm::aead::{generic_array::GenericArray};


fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_string_to_bytes() {
        let hex_shared_secret_key: String = "350ff7adbe9e77f14c3d0e3418bbbbda565c367d5915f0698a95ecced1bc0748".to_string();
        let bytes_shared_secret_key: Vec<u8> = vec![53, 15, 247, 173, 190, 158, 119, 241, 76, 61, 14, 52, 24, 187, 187, 218, 86, 92, 54, 125, 89, 21, 240, 105, 138, 149, 236, 206, 209, 188, 7, 72];
        let res = hex_string_to_bytes(&hex_shared_secret_key);
        assert_eq!(res, bytes_shared_secret_key);
    }

    #[test]
    fn test_encrypt_decrypt_with_bytes_key() {
        let u1_shared_secret_key: Vec<u8> = vec![
            139, 36, 97, 126, 75, 155, 72, 221, 93, 241, 50, 58, 53, 57, 233, 245, 233, 0, 234, 90,
            194, 214, 10, 156, 78, 92, 216, 237, 118, 130, 201, 92,
        ];
        let u2_hared_secret_key: Vec<u8> = vec![
            139, 36, 97, 126, 75, 155, 72, 221, 93, 241, 50, 58, 53, 57, 233, 245, 233, 0, 234, 90,
            194, 214, 10, 156, 78, 92, 216, 237, 118, 130, 201, 92,
        ];
        let plain_text = "Hello world!";
        let (ciphertext, iv) = encrypt_with_bytes_key(u1_shared_secret_key, plain_text.to_string());
        let decrypted_text = decrypt(u2_hared_secret_key, ciphertext, iv);
        assert_eq!(decrypted_text, plain_text);
    }

    #[test]
    fn test_encrypt_decrypt_with_string_key() {
        let hex_shared_secret_key: String = "350ff7adbe9e77f14c3d0e3418bbbbda565c367d5915f0698a95ecced1bc0748".to_string();
        let bytes_shared_secret_key: Vec<u8> = vec![53, 15, 247, 173, 190, 158, 119, 241, 76, 61, 14, 52, 24, 187, 187, 218, 86, 92, 54, 125, 89, 21, 240, 105, 138, 149, 236, 206, 209, 188, 7, 72];
        let plain_text = "Hello world!";
        let (ciphertext, iv) = encrypt_with_string_key(hex_shared_secret_key.to_string(), plain_text.to_string());
        let decrypted_text = decrypt(bytes_shared_secret_key, ciphertext, iv);
        assert_eq!(decrypted_text, plain_text);
    }
}
