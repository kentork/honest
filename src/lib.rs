// extern crate argon2rs;
extern crate base64;
extern crate bincode;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sha2;

mod blockchain;
mod block;
mod transaction;
mod nonce;
mod consensus;
mod hash;
mod unixtime;
mod serializer;
mod identification;

use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;
use blockchain::BlockChain;

lazy_static! {
  static ref BLOCK_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());
}


#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
  let mut buf = Vec::with_capacity(size);
  let ptr = buf.as_mut_ptr();
  mem::forget(buf);
  return ptr as *mut c_void;
}
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
  unsafe {
    let _buf = Vec::from_raw_parts(ptr, 0, cap);
  }
}
#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
  unsafe {
    let _ = CString::from_raw(ptr);
  }
}

#[no_mangle]
pub fn identify(user: &str, passphrase: &str) -> *mut c_char {
  let identification = BLOCK_CHAIN
    .lock()
    .unwrap()
    .identify(user, passphrase)
    .to_string();

  let c_string = CString::new(identification).unwrap();
  c_string.into_raw()
}

#[no_mangle]
pub fn new_transaction(sender: &str, recipient: &str, amount: u32) -> u64 {
  BLOCK_CHAIN
    .lock()
    .unwrap()
    .new_transaction(sender, recipient, amount)
}

#[no_mangle]
pub fn mine() -> u64 {
  BLOCK_CHAIN.lock().unwrap().mine()
}

#[no_mangle]
pub fn check_latest(block: &str) -> bool {
  BLOCK_CHAIN.lock().unwrap().check_latest(block)
}

#[no_mangle]
pub fn resolve_chain(chain: &str) -> bool {
  BLOCK_CHAIN.lock().unwrap().receive_blocks(chain)
}

#[no_mangle]
pub fn send_latest() -> *mut c_char {
  let blocks = BLOCK_CHAIN.lock().unwrap().send_latest().to_string();

  let c_string = CString::new(blocks).unwrap();
  c_string.into_raw()
}

#[no_mangle]
pub fn send_chain() -> *mut c_char {
  let blocks = BLOCK_CHAIN.lock().unwrap().send_chain().to_string();

  let c_string = CString::new(blocks).unwrap();
  c_string.into_raw()
}
