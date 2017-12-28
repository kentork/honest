use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
  pub sender: String,
  pub recipient: String,
  pub amount: u32,
}
impl fmt::Display for Transaction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "sender:{},recipient:{},amount:{}",
      self.sender,
      self.recipient,
      self.amount
    )
  }
}


#[test]
fn format_transaction() {
  let t = Transaction {
    sender: "aaa".to_string(),
    recipient: "bbb".to_string(),
    amount: 123,
  };
  let s = format!("{}", t);
  assert_eq!(s, "sender:aaa,recipient:bbb,amount:123");
}
