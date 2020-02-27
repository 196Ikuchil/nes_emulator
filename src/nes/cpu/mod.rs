mod opecodes;
mod fetch;
mod instructions;

use self::opecodes::*;
use self::fetch::*;
use self::instructions::*;
use std::fmt::Debug;

pub fn run() {

  let code = fetch();
  let ref opemap = opecodes::OPEMAP;
  let code = &*opemap.get(&code).unwrap();
  let operand = fetch_operand(&code);

  match code.name {
    Instruction::LDA if code.mode == Addressing::Immediate => lda_imm(operand),
    _ => panic!("Invalid code"),
  }
}