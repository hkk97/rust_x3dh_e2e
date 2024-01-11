use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub key_pair: KeyPair,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyPair {
    pub u1_shared_secret_key: String,
    pub u2_shared_secret_key: String,
}
