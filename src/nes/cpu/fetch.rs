use super::opecodes::*;
use super::super::cpu_register::CpuRegister;
use super::super::types::{Data, Addr, Word};

// fetch value from current pc addr
// Increment PC
pub fn fetch<T: CpuRegister>(register: &mut T) -> Data {
  let code: u8 = 0xA9;
  // TODO: PC++
  code
}

pub fn fetch_operand<T: CpuRegister>(code: &Opecode, register: &mut T) -> Word {
  match code.mode {
    Addressing::Immediate => fetch(register) as Word,
    _ => panic!("Invalid code"),
  }
}