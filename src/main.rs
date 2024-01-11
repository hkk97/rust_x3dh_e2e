mod encryption;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_string_to_bytes() {
        let hex_shared_secret_key: String = "350ff7adbe9e77f14c3d0e3418bbbbda565c367d5915f0698a95ecced1bc0748".to_string();
        let bytes_shared_secret_key: Vec<u8> = vec![53, 15, 247, 173, 190, 158, 119, 241, 76, 61, 14, 52, 24, 187, 187, 218, 86, 92, 54, 125, 89, 21, 240, 105, 138, 149, 236, 206, 209, 188, 7, 72];
        let res = encryption::hex_string_to_bytes(&hex_shared_secret_key);
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
        let (ciphertext, iv) = encryption::encrypt_with_bytes_key(u1_shared_secret_key, plain_text.to_string());
        let decrypted_text = encryption::decrypt_with_bytes_key(u2_hared_secret_key, ciphertext, iv);
        assert_eq!(decrypted_text, plain_text);
    }

    #[test]
    fn test_encrypt_decrypt_with_string_key() {
        let hex_shared_secret_key: String = "350ff7adbe9e77f14c3d0e3418bbbbda565c367d5915f0698a95ecced1bc0748".to_string();
        let bytes_shared_secret_key: Vec<u8> = vec![53, 15, 247, 173, 190, 158, 119, 241, 76, 61, 14, 52, 24, 187, 187, 218, 86, 92, 54, 125, 89, 21, 240, 105, 138, 149, 236, 206, 209, 188, 7, 72];
        let plain_text = "Hello world!";
        let (ciphertext, iv) = encryption::encrypt_with_string_key(hex_shared_secret_key.to_string(), plain_text.to_string());
        let decrypted_text = encryption::decrypt_with_bytes_key(bytes_shared_secret_key, ciphertext, iv);
        assert_eq!(decrypted_text, plain_text);
    }
}

fn main() {}