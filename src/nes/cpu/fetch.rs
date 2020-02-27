use super::opecodes::*;
use super::super::types::{Data, Addr, Word};

// fetch value from current pc addr
// Increment PC
pub fn fetch() -> Data {
  let code: u8 = 0xA9;
  // TODO: PC++
  code
}

pub fn fetch_operand(code: &Opecode) -> Word {
  match code.mode {
    Addressing::Immediate => fetch() as Word,
    _ => panic!("Invalid code"),
  }
}