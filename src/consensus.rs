use block::Block;
use nonce::Nonce;
#[cfg(test)]
use transaction::Transaction;

pub fn verify_chains(chains: &Vec<Block>) -> bool {
  let mut iterator = chains.iter();

  match iterator.next() {
    Some(first) => {
      let mut _index = first.index;
      let mut _hash = first.hash();
      let mut _nonce = first.proof;

      iterator
        .map(|block| match block {
          &Block {
            index,
            ref previous_hash,
            proof,
            ..
          } if (_index + 1) == index && _hash == *previous_hash && Nonce::verify(_nonce, proof) =>
          {
            _index = index;
            _hash = block.hash();
            _nonce = proof;
            true
          }
          _ => false,
        })
        .all(|x| x)
    }
    None => false,
  }
}

#[test]
fn check_verify_chains() {
  let blocks = vec![
    Block {
      index: 0,
      timestamp: 9876543,
      proof: 123456,
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
      index: 1,
      timestamp: 345678,
      proof: 1077,
      previous_hash: "2c6adf905ae484206000e72f9f21dccc1469d63594fa80ebaf1607725e35abd7".to_string(),
      transactions: vec![
        Transaction {
          sender: "eee".to_string(),
          recipient: "fff".to_string(),
          amount: 789,
        },
      ],
    },
  ];
  let is_valid = verify_chains(&blocks);

  assert!(is_valid)
}
