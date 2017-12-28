use base64::{decode, encode};
use bincode;
use block::Block;
#[cfg(test)]
use transaction::Transaction;

pub fn serialize(data: &Vec<Block>) -> String {
  let bytes = bincode::serialize(&data, bincode::Infinite).unwrap();
  encode(&bytes)
}
pub fn deserialize(data: &str) -> Vec<Block> {
  let bytes = decode(&data).unwrap();
  let decoded: Vec<Block> = bincode::deserialize(&bytes).unwrap();
  decoded
}

#[test]
fn check_serialize_chain() {
  let blocks = vec![
    Block {
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
    },
    Block {
      index: 11,
      timestamp: 345678,
      proof: 78,
      previous_hash: "prev".to_string(),
      transactions: vec![
        Transaction {
          sender: "eee".to_string(),
          recipient: "fff".to_string(),
          amount: 789,
        },
      ],
    },
  ];
  let base_string = serialize(&blocks);
  let restored: Vec<Block> = deserialize(&base_string);

  assert_eq!(restored.len(), 2);
  assert_eq!(restored.get(0).unwrap().index, 10);
  assert_eq!(
    restored
      .get(1)
      .unwrap()
      .transactions
      .get(0)
      .unwrap()
      .recipient,
    "fff"
  );
}
