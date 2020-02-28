mod opecodes;
mod fetch;
mod instructions;

use self::opecodes::*;
use self::fetch::*;
use self::instructions::*;
use std::fmt::Debug;

use super::cpu_register::CpuRegister;
use super::types::{Data, Addr, Word};
use super::cpu_bus::CpuBus;

pub fn run<T: CpuRegister, U: CpuBus>(register: &mut T, cpu_bus: &mut U) {

  let code = fetch(register, cpu_bus);
  let ref opemap = opecodes::OPEMAP;
  let code = &*opemap.get(&code).unwrap();
  let operand = fetch_operand(&code, register, cpu_bus);

  match code.name {
    Instruction::LDA if code.mode == Addressing::Immediate => lda_imm(operand, register),
    _ => panic!("Invalid code"),
  }
}