mod opecodes;
mod fetch;
mod instructions;

use self::opecodes::*;
use self::fetch::*;
use self::instructions::*;
use std::fmt::Debug;

use super::cpu_register::CpuRegister;
use super::types::{Data, Addr, Word};

pub fn run<T: CpuRegister>(register: &mut T) {

  let code = fetch(register);
  let ref opemap = opecodes::OPEMAP;
  let code = &*opemap.get(&code).unwrap();
  let operand = fetch_operand(&code, register);

  match code.name {
    Instruction::LDA if code.mode == Addressing::Immediate => lda_imm(operand),
    _ => panic!("Invalid code"),
  }
}