use std::io;

pub mod encryptor;
use encryptor::rot13::Rot13;
use encryptor::Encryptable;

fn main() {
  println!("Write your text to encrypt:");
  let mut user_input = String::new();
  io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");
  println!("Your encrypted text is: {}", Rot13(user_input).encrypt());
}