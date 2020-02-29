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
    Instruction::LDA => lda(operand, register, cpu_bus),
    _ => panic!("Invalid code"),
  }
}

#[cfg(test)]
mod test {
  use super::super::cpu_register::Register;
  use super::*;

  struct MockBus {
    pub memory: Vec<Data>,
  }

  impl MockBus {
    fn new() -> Self {
      MockBus {
        memory: vec!(0; 65535)
      }
    }
  }

  impl CpuBus for MockBus {
    fn read(&mut self, a: Addr) -> Data {
      self.memory[a as usize]
    }

    fn read_word(&mut self, a: Addr) -> Word {
      let top = self.read(a) as u16;
      let low = self.read(a + 1) as u16;
      ( top << 8 | low ) as Word
    }

    fn write(&mut self, a: Addr, d: Data)  {
      self.memory[a as usize] = d
    }
  }

  #[test]
  fn test_run_lda_imm() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xA9;
    b.memory[0x81] = 0xFF;
    run(&mut r ,&mut b);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_lda_zpg_x() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_X(0x01);
    b.memory[0x80] = 0xB5;
    b.memory[0x81] = 0x11;
    b.memory[0x12] = 0xFF;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF)
  }
}