use super::opecodes::*;
use super::super::cpu_register::CpuRegister;
use super::CpuBus;
use super::super::types::{Data, Addr, Word};

// fetch value from current pc addr
// Increment PC
pub fn fetch<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Data {
  let code: u8 = 0xA9;
  // TODO: PC++
  code
}

pub fn fetch_operand<T: CpuRegister, U: CpuBus>(code: &Opecode, register: &mut T, bus: &mut U) -> Word {
  match code.mode {
    Addressing::Immediate => fetch(register, bus) as Word,
    _ => panic!("Invalid code"),
  }
}