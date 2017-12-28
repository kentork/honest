use base64::{decode, encode};
use bincode;
use block::Block;
#[cfg(test)]
use transaction::Transaction;

pub fn serialize(data: &Block) -> String {
  let bytes = bincode::serialize(&data, bincode::Infinite).unwrap();
  encode(&bytes)
}
pub fn deserialize(data: &str) -> Block {
  let bytes = decode(&data).unwrap();
  let decoded: Block = bincode::deserialize(&bytes).unwrap();
  decoded
}

#[test]
fn check_serialize_block() {
  let block = Block {
    index: 10,
    timestamp: 9876543,
    proof: 56,
    previous_hash: "prevprev".to_string(),
    transactions: vec![
      Transaction {
        sender: "aaa".to_string(),
        recipient: "bbb".to_string(),
        amount: 123,
      },
      Transaction {
        sender: "ccc".to_string(),
        recipient: "ddd".to_string(),
        amount: 456,
      },
    ],
  };
  let base_string = serialize(&block);
  let restored: Block = deserialize(&base_string);

  assert_eq!(restored.index, 10);
  assert_eq!(restored.transactions.get(0).unwrap().recipient, "bbb");
}
