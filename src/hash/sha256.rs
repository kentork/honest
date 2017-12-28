use sha2::{Sha256, Digest};

pub fn digest(s: &str) -> String {
  let digest_bytes = Sha256::digest_str(&s);
  format!("{:x}", digest_bytes)
}


#[test]
fn check_hash() {
  let h = digest("SHA-2 (Secure Hash Algorithm 2) is a set of cryptographic hash functions designed by the United States National Security Agency (NSA).");
  assert_eq!(h, "412575befc2caad471682f48753523d34a3fc58fc0f38a237401622457db229b");
}