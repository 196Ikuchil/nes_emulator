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
    Instruction::LDX if code.mode == Addressing::Immediate => ldx_imm(operand, register),
    Instruction::LDX => ldx(operand, register, cpu_bus),
    Instruction::LDY if code.mode == Addressing::Immediate => ldy_imm(operand, register),
    Instruction::LDY => ldy(operand, register, cpu_bus),
    Instruction::STA => sta(operand, register, cpu_bus),
    Instruction::STX => stx(operand, register, cpu_bus),
    Instruction::STY => sty(operand, register, cpu_bus),
    Instruction::TAX => tax(register),
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

  #[test]
  fn test_run_ldx_imm() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xA2;
    b.memory[0x81] = 0xFF;
    run(&mut r ,&mut b);
    assert_eq!(r.get_X(), 0xFF)
  }

  #[test]
  fn test_run_ldx_zpg_y() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_Y(0x01);
    b.memory[0x80] = 0xB6;
    b.memory[0x81] = 0x11;
    b.memory[0x12] = 0xFF;
    run(&mut r, &mut b);
    assert_eq!(r.get_X(), 0xFF)
  }

  #[test]
  fn test_run_ldy_imm() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xA0;
    b.memory[0x81] = 0xFF;
    run(&mut r ,&mut b);
    assert_eq!(r.get_Y(), 0xFF)
  }

  #[test]
  fn test_run_ldy_zpg_x() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_X(0x01);
    b.memory[0x80] = 0xB4;
    b.memory[0x81] = 0x11;
    b.memory[0x12] = 0xFF;
    run(&mut r, &mut b);
    assert_eq!(r.get_Y(), 0xFF)
  }

  #[test]
  fn test_run_sta_zpg_x(){
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_A(0xFF);
    r.set_X(0x01);
    b.memory[0x80] = 0x95;
    b.memory[0x81] = 0x11;
    run(&mut r, &mut b);
    assert_eq!(b.read(0x12), 0xFF)
  }

  #[test]
  fn test_run_stx_zpg_y() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_X(0xFF);
    r.set_Y(0x01);
    b.memory[0x80] = 0x96;
    b.memory[0x81] = 0x11;
    run(&mut r, &mut b);
    assert_eq!(b.read(0x12),0xFF)
  }

  #[test]
  fn test_run_sty_zpg_x() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_Y(0xFF);
    r.set_X(0x01);
    b.memory[0x80] = 0x94;
    b.memory[0x81] = 0x11;
    run(&mut r, &mut b);
    assert_eq!(b.read(0x12), 0xFF)
  }

  #[test]
  fn test_run_tax() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0xFF);
    b.memory[0x80] = 0xAA;
    run(&mut r, &mut b);
    assert_eq!(r.get_X(),0xFF)
  }
}