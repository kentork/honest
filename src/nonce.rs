use hash;

pub struct Nonce {
  pub current: u64,
}

impl Nonce {
  pub fn find_next(&self) -> u64 {
    let mut nonce = 1;
    while !Nonce::verify(self.current, nonce) {
      nonce += 1;
    }
    nonce
  }

  pub fn verify(current: u64, next: u64) -> bool {
    let message = format!("{}{}", current, next);
    let digest = hash::sha256::digest(&message);
    digest.starts_with("000")
  }
}


#[test]
fn check_validation() {
  let is_valid = Nonce::verify(123456, 1077);
  assert!(is_valid)
}
#[test]
fn check_find_next() {
  let nonce = Nonce { current: 123456 };
  let next = nonce.find_next();

  assert_eq!(next, 1077)
}
