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

pub fn run<T: CpuRegister, U: CpuBus>(register: &mut T, cpu_bus: &mut U, _nmi: &mut bool) {

  if *_nmi {
    nmi(register, cpu_bus);
    *_nmi = false;
  }

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
    Instruction::INC => inc(operand, register, cpu_bus),
    Instruction::INX => inx(register),
    Instruction::INY => iny(register),
    Instruction::LSR if code.mode == Addressing::Accumulator => lsr_acc(register),
    Instruction::LSR => lsr(operand, register, cpu_bus),
    Instruction::ORA if code.mode == Addressing::Immediate => ora_imm(operand, register),
    Instruction::ORA => ora(operand, register, cpu_bus),
    Instruction::ROL if code.mode == Addressing::Accumulator => rol_acc(register),
    Instruction::ROL => rol(operand, register, cpu_bus),
    Instruction::ROR if code.mode == Addressing::Accumulator => ror_acc(register),
    Instruction::ROR => ror(operand, register, cpu_bus),
    Instruction::SBC if code.mode == Addressing::Immediate => sbc_imm(operand, register),
    Instruction::SBC => sbc(operand, register, cpu_bus),

    Instruction::PHA => pha(register, cpu_bus),
    Instruction::PHP => php(register, cpu_bus),
    Instruction::PLA => pla(register, cpu_bus),
    Instruction::PLP => plp(register, cpu_bus),

    Instruction::JMP => jmp(operand, register),
    Instruction::JSR => jsr(operand, register, cpu_bus),
    Instruction::RTS => rts(register, cpu_bus),
    Instruction::RTI => rti(register, cpu_bus),
    Instruction::BCC => bcc(operand, register),
    Instruction::BCS => bcs(operand, register),
    Instruction::BEQ => beq(operand, register),
    Instruction::BMI => bmi(operand, register),
    Instruction::BNE => bne(operand, register),
    Instruction::BPL => bpl(operand, register),
    Instruction::BVC => bvc(operand, register),
    Instruction::BVS => bvs(operand, register),

    Instruction::CLC => clc(register),
    Instruction::CLD => cld(register),
    Instruction::CLI => cli(register),
    Instruction::CLV => clv(register),
    Instruction::SEC => sec(register),
    Instruction::SED => sed(register),
    Instruction::SEI => sei(register),

    Instruction::BRK => brk(register, cpu_bus),
    Instruction::NOP => (),
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
        memory: vec!(0; 65536)
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
  fn test_run_nmi_lda_imm() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    b.memory[0xFFFA] = 0x00;
    b.memory[0xFFFB] = 0x80;
    b.memory[0x80] = 0xA9;
    b.memory[0x81] = 0xFF;
    run(&mut r ,&mut b, &mut true);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_lda_imm() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xA9;
    b.memory[0x81] = 0xFF;
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_ldx_imm() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xA2;
    b.memory[0x81] = 0xFF;
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_X(), 0xFF)
  }

  #[test]
  fn test_run_ldy_imm() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xA0;
    b.memory[0x81] = 0xFF;
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.read(0x12), 0xFF)
  }

  #[test]
  fn test_run_tax() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0xFF);
    b.memory[0x80] = 0xAA;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_X(),0xFF)
  }

  #[test]
  fn test_run_tay() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0xFF);
    b.memory[0x80] = 0xA8;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_Y(),0xFF)
  }

  #[test]
  fn test_run_tsx() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_S(0xFF);
    b.memory[0x80] = 0xBA;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_X(), 0xFF)
  }

  #[test]
  fn test_run_txa() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0xFF);
    b.memory[0x80] = 0x8A;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_txs() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0xFF);
    b.memory[0x80] = 0x9A;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_S(), 0xFF)
  }

  #[test]
  fn test_run_tya() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Y(0xFF);
    b.memory[0x80] = 0x98;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_adc_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xA9;
    b.memory[0x81] = 0x01;
    b.memory[0x82] = 0x69;
    b.memory[0x83] = 0x01;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0x01);
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0x02);
  }

  #[test]
  fn test_run_and_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x01);
    b.memory[0x80] = 0x29;
    b.memory[0x81] = 0x11;
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0x01)
  }

  #[test]
  fn test_run_asl_acc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x01);
    b.memory[0x80] = 0x0A;
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.memory[0x10], 0x01)
  }

  #[test]
  fn test_run_dex() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0x02);
    b.memory[0x80] = 0xCA;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_X(), 0x01)
  }

  #[test]
  fn test_run_dey() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Y(0x02);
    b.memory[0x80] = 0x88;
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
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
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_inc_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xE6;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x02;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.memory[0x10], 0x03)
  }

  #[test]
  fn test_run_inx() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_X(0x02);
    b.memory[0x80] = 0xE8;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_X(), 0x03)
  }

  #[test]
  fn test_run_iny() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Y(0x02);
    b.memory[0x80] = 0xC8;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_Y(), 0x03)
  }

  #[test]
  fn test_run_lsr_acc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x02);
    b.memory[0x80] = 0x4A;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0x01)
  }

  #[test]
  fn test_run_lsr_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0x46;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x02;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.memory[0x10], 0x01)
  }

  #[test]
  fn test_run_ora_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0xF0);
    b.memory[0x80] = 0x09;
    b.memory[0x81] = 0x0F;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_ora_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0xF0);
    b.memory[0x80] = 0x05;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x0F;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_run_rol_acc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x01);
    r.set_PC(0x80);
    b.memory[0x80] = 0x2A;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0x02);
  }

  #[test]
  fn test_run_rol_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0x26;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x01;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.memory[0x10], 0x02)
  }



  #[test]
  fn test_run_ror_acc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x02);
    r.set_PC(0x80);
    b.memory[0x80] = 0x6A;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0x01);
  }

  #[test]
  fn test_run_ror_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0x66;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x02;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.memory[0x10], 0x01)
  }

  #[test]
  fn test_run_sbc_imm() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x03);
    r.set_status_carry(true);
    b.memory[0x80] = 0xE9;
    b.memory[0x81] = 0x02;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(),0x01);
    assert_eq!(r.get_status_overflow(), false);

    r.set_PC(0x80);
    r.set_A(0x04);
    r.set_status_carry(false);
    b.memory[0x80] = 0xE9;
    b.memory[0x81] = 0x03;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(),0x00);
    assert_eq!(r.get_status_overflow(), false);

    r.set_PC(0x80);
    r.set_A(0x01);
    r.set_status_carry(true);
    b.memory[0x80] = 0xE9;
    b.memory[0x81] = 0x80;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_overflow(), true);
  }

  #[test]
  fn test_run_sbc_zpg() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0x03);
    r.set_status_carry(true);
    b.memory[0x80] = 0xE5;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x02;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(),0x01);
    assert_eq!(r.get_status_overflow(), false);

    r.set_PC(0x80);
    r.set_A(0x04);
    r.set_status_carry(false);
    b.memory[0x80] = 0xE5;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x03;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(),0x00);
    assert_eq!(r.get_status_overflow(), false);

    r.set_PC(0x80);
    r.set_A(0x01);
    r.set_status_carry(true);
    b.memory[0x80] = 0xE5;
    b.memory[0x81] = 0x10;
    b.memory[0x10] = 0x80;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_overflow(), true);
  }

  #[test]
  fn test_run_pha() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_A(0xFF);
    r.set_S(0x10);
    b.memory[0x80] = 0x48;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.memory[0x0110], 0xFF);
  }

  #[test]
  fn test_run_php() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_Status(0xFF);
    r.set_S(0x10);
    b.memory[0x80] = 0x08;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(b.memory[0x0110],0xFF)
  }

  #[test]
  fn test_run_pla() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_S(0x10);
    b.memory[0x80] = 0x68;
    b.memory[0x0111] = 0xFF;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_A(), 0xFF);
  }

  #[test]
  fn test_run_plp() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_S(0x10);
    b.memory[0x80] = 0x28;
    b.memory[0x0111] = 0xFF;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_Status(), 0xFF);
  }

  #[test]
  fn test_run_jmp_abs () {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0x4C;
    b.memory[0x81] = 0x01;
    b.memory[0x82] = 0x02;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x0201);
  }

  #[test]
  fn test_run_jsr() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_S(0x30);
    b.memory[0x80] = 0x20;
    b.memory[0x81] = 0x11;
    b.memory[0x82] = 0x22;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x2211);
    assert_eq!(b.memory[0x0130], 0x00);
    assert_eq!(b.memory[0x012F], 0x82);
  }

  #[test]
  fn test_run_rts() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_S(0x30);
    b.memory[0x80] = 0x20;
    b.memory[0x81] = 0x11;
    b.memory[0x82] = 0x22;
    b.memory[0x2211] = 0x60;
    run(&mut r ,&mut b, &mut false);
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x0083);
  }

  #[test]
  fn test_run_rti() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_Status(0xFF);
    r.set_PC(0x80);
    r.set_S(0x30);
    b.memory[0x80] = 0x20;
    b.memory[0x81] = 0x11;
    b.memory[0x82] = 0x22;
    b.memory[0x2211] = 0x08;
    b.memory[0x2212] = 0x40;
    run(&mut r ,&mut b, &mut false);
    run(&mut r ,&mut b, &mut false);
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x83);
    assert_eq!(r.get_Status(),0xFF);
  }

  #[test]
  fn test_run_bcc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_carry(false);
    b.memory[0x80] = 0x90;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }

  #[test]
  fn test_run_bcs() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_carry(true);
    b.memory[0x80] = 0xB0;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }

  #[test]
  fn test_run_beq() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_zero(true);
    b.memory[0x80] = 0xF0;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }

  #[test]
  fn test_run_bmi() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_negative(true);
    b.memory[0x80] = 0x30;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }

  #[test]
  fn test_run_bne() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_zero(false);
    b.memory[0x80] = 0xD0;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }

  #[test]
  fn test_run_bpl() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_negative(false);
    b.memory[0x80] = 0x10;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }

  #[test]
  fn test_run_bvc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_overflow(false);
    b.memory[0x80] = 0x50;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }
  #[test]
  fn test_run_bvs() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x80);
    r.set_status_overflow(true);
    b.memory[0x80] = 0x70;
    b.memory[0x81] = 0x10;
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x92);
  }

  #[test]
  fn test_flag_ope() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_status_carry(true);
    r.set_PC(0x80);
    b.memory[0x80] = 0x18;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_carry(), false);
    r.set_PC(0x80);
    b.memory[0x80] = 0x38;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_carry(), true);

    r.set_status_decimal_mode(true);
    r.set_PC(0x80);
    b.memory[0x80] = 0xD8;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_decimal_mode(), false);
    r.set_PC(0x80);
    b.memory[0x80] = 0xF8;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_decimal_mode(), true);

    r.set_status_interrupt(true);
    r.set_PC(0x80);
    b.memory[0x80] = 0x58;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_interrupt(), false);
    r.set_PC(0x80);
    b.memory[0x80] = 0x78;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_interrupt(), true);

    r.set_status_overflow(true);
    r.set_PC(0x80);
    b.memory[0x80] = 0xB8;
run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_status_overflow(), false);
  }

  #[test]
  fn test_run_brk() {
    let mut r = Register::new();
    let mut b = MockBus::new();

    r.set_PC(0x80);
    r.set_S(0x10);
    r.set_Status(0xFF);
    b.memory[0x80] = 0x00;
    b.memory[0xFFFE] = 0x22;
    b.memory[0xFFFF] = 0x11;

    r.set_status_interrupt(false);
    run(&mut r ,&mut b, &mut false);
    assert_eq!(r.get_PC(), 0x2211);
    assert_eq!(b.memory[0x010F], 0x82);
    assert_eq!(b.memory[0x010E], 0xFB);
  }
}