#[macro_use]
extern crate bencher;

mod models;
mod encryption;
use bencher::Bencher;
use models::Message;
use std::{fs::File, io::Read};
use std::io::{Error};

fn read_json_file() -> Result<Vec<Message>, Error> {
    let mut file = match File::open("src/json/messages.json") {
        Ok(file) => file,
        Err(err) => return Err(Error::from(err)),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => return Err(Error::from(err)),
    };

    let messages: Vec<Message> = serde_json::from_str(&contents)?;
    Ok(messages)
}

fn encryption_benchmark(b: &mut Bencher) {
    b.iter(|| {
        match read_json_file() {
            Ok(messages) => {
                for message in messages {
                    let key_pair = &message.key_pair;
                    let text = &message.text;
                    let (ciphertext, iv) = encryption::encrypt_with_string_key(key_pair.u1_shared_secret_key.to_string(), text.to_string());
                    let decrypted_text = encryption::decrypt_with_string_key(key_pair.u2_shared_secret_key.to_string(), ciphertext, iv);
                    assert_eq!(decrypted_text, text.to_string());
                }
            }
            Err(error) => {
                eprintln!("Error reading messages.json: {}", error);
            }
        }
    });
}

benchmark_group!(benches, encryption_benchmark);
benchmark_main!(benches);
