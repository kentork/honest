// use argon2rs::verifier::Encoded;
use hash;

// pub fn generate(password: &str, passphrase: &str) -> String {
//   let halite = format!("i said {}", passphrase);
//   let salt = hash::sha256::digest(&halite);
//   let salt_bytes = salt.as_bytes();
//   let bytes = Encoded::default2i(password.as_bytes(), &salt_bytes, &[], &[]).to_u8();
//   String::from_utf8(bytes).unwrap()
// }

pub fn generate(password: &str, passphrase: &str) -> String {
  let message = format!("{} {}", password, passphrase);
  hash::sha256::digest(&message)
}
