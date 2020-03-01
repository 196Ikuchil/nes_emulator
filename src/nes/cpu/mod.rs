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
    Instruction::TAY => tay(register),
    Instruction::TSX => tsx(register),
    Instruction::TXA => txa(register),
    Instruction::TXS => txs(register),
    Instruction::TYA => tya(register),
    Instruction::ADC if code.mode == Addressing::Immediate => adc_imm(operand, register),
    Instruction::ADC => adc(operand, register, cpu_bus),
    Instruction::AND if code.mode == Addressing::Immediate => and_imm(operand, register),
    Instruction::AND => and(operand, register, cpu_bus),
    Instruction::ASL if code.mode == Addressing::Accumulator => asl_acc(register),
    Instruction::ASL => asl(operand, register, cpu_bus),
    Instruction::BIT => bit(operand, register, cpu_bus),
    Instruction::CMP if code.mode == Addressing::Immediate => cmp_imm(operand, register),
    Instruction::CMP => cmp(operand, register, cpu_bus),
    Instruction::CPX if code.mode == Addressing::Immediate => cpx_imm(operand, register),
    Instruction::CPX => cpx(operand, register, cpu_bus),
    Instruction::CPY if code.mode == Addressing::Immediate => cpy_imm(operand, register),
    Instruction::CPY => cpy(operand, register, cpu_bus),
    Instruction::DEC => dec(operand, register, cpu_bus),
    Instruction::DEX => dex(register),
    Instruction::DEY => dey(register),
    Instruction::EOR if code.mode == Addressing::Immediate => eor_imm(operand, register),
    Instruction::EOR => eor(operand, register, cpu_bus),
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

  #[test]
  fn test_run_tay() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0xFF);
    b.memory[0x80] = 0xA8;
    run(&mut r, &mut b);
    assert_eq!(r.get_Y(),0xFF)
  }

  #[test]
  fn test_run_tsx() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_S(0xFF);
    b.memory[0x80] = 0xBA;
    run(&mut r, &mut b);
    assert_eq!(r.get_X(), 0xFF)
  }

  #[test]
  fn test_run_txa() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0xFF);
    b.memory[0x80] = 0x8A;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_txs() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0xFF);
    b.memory[0x80] = 0x9A;
    run(&mut r, &mut b);
    assert_eq!(r.get_S(), 0xFF)
  }

  #[test]
  fn test_run_tya() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Y(0xFF);
    b.memory[0x80] = 0x98;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_and_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x01);
    b.memory[0x80] = 0x29;
    b.memory[0x81] = 0x11;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0x01)
  }

  #[test]
  fn test_run_and_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x01);
    b.memory[0x80] = 0x25;
    b.memory[0x81] = 0x22;
    b.memory[0x22] = 0x11;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0x01)
  }

  #[test]
  fn test_run_asl_acc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x01);
    b.memory[0x80] = 0x0A;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0x02)
  }

  #[test]
  fn test_run_asl_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0x06;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x01;
    run(&mut r, &mut b);
    assert_eq!(b.memory[0x10], 0x02)
  }

  #[test]
  fn test_run_bit_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0x24;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x40;
    r.set_A(0x40);
    run(&mut r, &mut b);
    assert_eq!(r.get_status_zero(), false);
    assert_eq!(r.get_status_negative(), false);
    assert_eq!(r.get_status_overflow(), true)
  }

  #[test]
  fn test_run_cmp_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x40);
    b.memory[0x80] = 0xC9;
    b.memory[0x81] = 0x50;
    run(&mut r, &mut b);
    assert_eq!(r.get_status_negative(), true)
  }

  #[test]
  fn test_run_cmp_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x40);
    b.memory[0x80] = 0xC5;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x50;
    run(&mut r, &mut b);
    assert_eq!(r.get_status_negative(), true)
  }

  #[test]
  fn test_run_cpx_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0x40);
    b.memory[0x80] = 0xC9;
    b.memory[0x81] = 0x50;
    run(&mut r, &mut b);
    assert_eq!(r.get_status_negative(), true)
  }

  #[test]
  fn test_run_cpx_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0x40);
    b.memory[0x80] = 0xC5;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x50;
    run(&mut r, &mut b);
    assert_eq!(r.get_status_negative(), true)
  }

  #[test]
  fn test_run_cpy_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Y(0x40);
    b.memory[0x80] = 0xC9;
    b.memory[0x81] = 0x50;
    run(&mut r, &mut b);
    assert_eq!(r.get_status_negative(), true)
  }

  #[test]
  fn test_run_cpy_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Y(0x40);
    b.memory[0x80] = 0xC5;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x50;
    run(&mut r, &mut b);
    assert_eq!(r.get_status_negative(), true)
  }

  #[test]
  fn test_run_dec_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xC6;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x02;
    run(&mut r, &mut b);
    assert_eq!(b.memory[0x10], 0x01)
  }

  #[test]
  fn test_run_dex() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0x02);
    b.memory[0x80] = 0xCA;
    run(&mut r, &mut b);
    assert_eq!(r.get_X(), 0x01)
  }

  #[test]
  fn test_run_dey() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Y(0x02);
    b.memory[0x80] = 0x88;
    run(&mut r, &mut b);
    assert_eq!(r.get_Y(), 0x01)
  }

  #[test]
  fn test_run_eor_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x0F);
    b.memory[0x80] = 0x49;
    b.memory[0x81] = 0xF0;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_eor_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x0F);
    b.memory[0x80] = 0x45;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0xF0;
    run(&mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF)
  }
}