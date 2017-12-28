use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
  let now = SystemTime::now();
  let spec = now.duration_since(UNIX_EPOCH).unwrap();
  spec.as_secs() * 1_000_000_000 + spec.subsec_nanos() as u64
}
