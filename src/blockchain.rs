use block::Block;
use transaction::Transaction;
use nonce::Nonce;
use unixtime;
use serializer;
use consensus;
use identification;
use hash;

pub struct BlockChain {
  identifier: String,
  blocks: Vec<Block>,
  current_transactions: Vec<Transaction>,
}

impl BlockChain {
  pub fn new() -> BlockChain {
    let mut instance = BlockChain {
      identifier: String::new(),
      blocks: Vec::new(),
      current_transactions: Vec::new(),
    };
    instance.new_block(0, 1);
    instance
  }

  pub fn identify(&mut self, password: &str, passphrase: &str) -> String {
    self.identifier = identification::generate(&password, &passphrase);
    self.identifier.clone()
  }

  pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: u32) -> u64 {
    self.current_transactions.push(Transaction {
      sender: sender.to_string(),
      recipient: recipient.to_string(),
      amount: amount,
    });
    self.blocks.len() as u64
  }

  pub fn check_latest(&self, another: &str) -> bool {
    let another_block = serializer::block::deserialize(another);

    if self.blocks.last().unwrap().index < another_block.index {
      true
    } else {
      false
    }
  }

  pub fn receive_blocks(&mut self, another: &str) -> bool {
    let another_chains = serializer::chain::deserialize(another);

    if self.blocks.len() < another_chains.len() && consensus::verify_chains(&another_chains) {
      self.blocks = another_chains;
      true
    } else{
      false
    }
  }

  pub fn send_latest(&self) -> String {
    serializer::block::serialize(&self.blocks.last().unwrap())
  }

  pub fn send_chain(&self) -> String {
    serializer::chain::serialize(&self.blocks)
  }

  pub fn mine(&mut self) -> u64 {
    let recipient = self.identifier.to_string();
    self.new_transaction("0", &recipient, 2);

    let current_proof = self.blocks.last().unwrap().proof;
    let nonce = Nonce { current: current_proof };
    let next_proof = nonce.find_next();
    self.new_block(unixtime::nano::now(), next_proof);

    self.blocks.len() as u64
  }

  fn new_block(&mut self, timestamp: u64, proof: u64) -> u64 {
    let current_index = self.blocks.len() as u64;
    let next_transactions = self.current_transactions.to_vec();

    let next = match self.blocks.last() {
      Some(previous) => {
        Block {
          index: current_index,
          timestamp: timestamp,
          proof: proof,
          previous_hash: previous.hash(),
          transactions: next_transactions
        }
      }
      None => {
        Block {
          index: 0,
          timestamp: timestamp,
          proof: proof,
          previous_hash: "genesis".to_string(),
          transactions: Vec::new(),
        }
      }
    };
    self.blocks.push(next);

    self.current_transactions = Vec::new();

    (self.blocks.len() - 1) as u64
  }
}


#[test]
fn check_identify() {
  let mut bc = BlockChain::new();
  let id = bc.identify("hoge", "huga");

  assert_eq!(id, "$argon2i$m=4096,t=3,p=1$Njc4M2M2YTlhOGI5MDkxNjY3ZDg4NDU0NTUxZDU3YjZhYzk0MTI2YzM2Y2QyMzhjZTNlYmVlM2VmN2ZhZjgyMA$ZDDkQds+UQpsmCKpFV1B6mGrKC7GeixVPC3Dsv2zdSA");
}

#[test]
fn create_genesis() {
  let bc = BlockChain::new();
  let genesis = bc.blocks.last().unwrap();

  assert_eq!(genesis.index, 0);
  assert_eq!(genesis.timestamp, 0);
  assert_eq!(genesis.proof, 1);
  assert_eq!(genesis.previous_hash, "genesis");
  assert_eq!(genesis.transactions.len(), 0);
}

#[test]
fn create_block() {
  let mut bc = BlockChain::new();

  let now = 1_417_176_009_000_000_000;

  let one = bc.new_transaction("aaa", "bbb", 123);
  let two = bc.new_transaction("ccc", "ddd", 456);
  let b1 = bc.new_block(now, 888);
  let b1_contents = format!("{}", bc.blocks.get(1).unwrap());

  let now = 1_417_177_213_000_000_000;

  let tree = bc.new_transaction("eee", "fff", 0);
  let b2 = bc.new_block(now, 765);
  let b2_contents = format!("{}", bc.blocks.get(2).unwrap());

  assert_eq!(one, 1);
  assert_eq!(two, 1);
  assert_eq!(tree, 2);

  assert_eq!(b1, 1);
  assert_eq!(b2, 2);

  assert_eq!(b1_contents, "index:1,timestamp:1417176009000000000,proof:888,previous_hash:3ed614bd4685275a66806471a13d0f673a161ec772ea90ef17c50b0d28a52320,transactions:[sender:aaa,recipient:bbb,amount:123;sender:ccc,recipient:ddd,amount:456;]");
  assert_eq!(b2_contents, "index:2,timestamp:1417177213000000000,proof:765,previous_hash:9349c7659697ceb6935c4865de29b62526b690250cbcee59e7bc9e63b2735e87,transactions:[sender:eee,recipient:fff,amount:0;]");
}

#[test]
fn do_mining() {
  let mut bc = BlockChain::new();
  let id = bc.identify("kimmy", "asdfghjkl");
  bc.mine();

  let block = bc.blocks.last().unwrap();

  assert_eq!(block.index, 1);
  assert_eq!(block.proof, 2823);
  assert_eq!(block.previous_hash, "3ed614bd4685275a66806471a13d0f673a161ec772ea90ef17c50b0d28a52320");
  assert_eq!(block.transactions.len(), 1);
  assert_eq!(block.transactions.get(0).unwrap().sender, "0");
  assert_eq!(block.transactions.get(0).unwrap().recipient, id);
  assert_eq!(block.transactions.get(0).unwrap().amount, 2);
}
