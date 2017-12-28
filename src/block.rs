use std::fmt;
use transaction::Transaction;
use hash;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Block {
  pub index: u64,
  pub timestamp: u64,
  pub transactions: Vec<Transaction>,
  pub proof: u64,
  pub previous_hash: String,
}

impl Block {
  pub fn hash(&self) -> String {
    let block_string = format!("{}", self);
    hash::sha256::digest(&block_string)
  }
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "index:{},timestamp:{},proof:{},previous_hash:{}",
      self.index, self.timestamp, self.proof, self.previous_hash
    ).unwrap();

    write!(f, ",transactions:[").unwrap();
    for i in &self.transactions {
      write!(f, "{};", i).unwrap();
    }
    write!(f, "]").unwrap();

    Ok(())
  }
}


#[test]
fn format_block() {
  let t1 = Transaction {sender: "aaa".to_string(), recipient: "bbb".to_string(), amount: 123};
  let t2 = Transaction {sender: "ccc".to_string(), recipient: "ddd".to_string(), amount: 456};
  let b = Block {index: 10, timestamp: 9876543, proof: 56, previous_hash: "prevprev".to_string(), transactions: vec![t1, t2]};

  let s = format!("{}", b);
  assert_eq!(s, "index:10,timestamp:9876543,proof:56,previous_hash:prevprev,transactions:[sender:aaa,recipient:bbb,amount:123;sender:ccc,recipient:ddd,amount:456;]");
}

#[test]
fn check_hash() {
  let t1 = Transaction {sender: "aaa".to_string(), recipient: "bbb".to_string(), amount: 123};
  let t2 = Transaction {sender: "ccc".to_string(), recipient: "ddd".to_string(), amount: 456};
  let b = Block {index: 10, timestamp: 9876543, proof: 56, previous_hash: "prevprev".to_string(), transactions: vec![t1, t2]};

  let h = b.hash();
  assert_eq!(h, "cb656cc6c75f10048e4f8a29057331bc9bf55fff1d67c862911cd1d708a35b71");
}