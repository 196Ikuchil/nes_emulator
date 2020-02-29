use super::super::types::{Data, Addr, Word};
use super::CpuRegister;
use super::CpuBus;
use super::super::helper::*;

use std::fmt::Debug;

pub fn lda_imm<T: CpuRegister>(operand: Word, register: &mut T) {
    register
      .set_A(operand as Data)
      .update_status_negative_by(operand as Data)
      .update_status_zero_by(operand as Data);
}

pub fn lda<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let v = bus.read(operand as Addr);
  register
    .set_A(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn ldx_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  register
    .set_X(operand as Data)
    .update_status_negative_by(operand as Data)
    .update_status_zero_by(operand as Data);
}

pub fn ldx<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let v = bus.read(operand as Addr);
  register
    .set_X(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn ldy_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  register
    .set_Y(operand as Data)
    .update_status_negative_by(operand as Data)
    .update_status_zero_by(operand as Data);
}

pub fn ldy<T: CpuRegister, U: CpuBus>(operand: Addr, register: &mut T, bus: &mut U) {
  let v = bus.read(operand);
  register
    .set_Y(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn sta<T: CpuRegister, U: CpuBus>(operand: Addr, register: &mut T, bus: &mut U) {
  bus.write(operand, register.get_A())
}

pub fn stx<T: CpuRegister, U: CpuBus>(operand: Addr, register: &mut T, bus: &mut U) {
  bus.write(operand, register.get_X())
}

pub fn sty<T: CpuRegister, U: CpuBus>(operand: Addr, register: &mut T, bus: &mut U) {
  bus.write(operand, register.get_Y())
}

pub fn tax<T: CpuRegister>(register: &mut T) {
  let v = register.get_A();
  register
    .set_X(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn tay<T: CpuRegister>(register: &mut T) {
  let v = register.get_A();
  register
    .set_Y(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn tsx<T: CpuRegister>(register: &mut T) {
  let v = register.get_S();
  register
    .set_X(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn txa<T: CpuRegister>(register: &mut T) {
  let v = register.get_X();
  register
    .set_A(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn txs<T: CpuRegister>(register: &mut T) {
  let v = register.get_X();
  register
    .set_S(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn tya<T: CpuRegister>(register: &mut T) {
  let v = register.get_Y();
  register
    .set_A(v)
    .update_status_negative_by(v)
    .update_status_zero_by(v);
}

pub fn adc_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  let a = register.get_A();
  let computed = operand + a as u16 +  bool2u8(register.get_status_carry()) as u16;
  let result = (computed & 0xff) as u8;

  register
    .set_status_overflow(((a ^ result) & ((operand as u8) ^ result) & 0x80) == 0x80)
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed > 0x00ff)
    .set_A(result);
}

pub fn adc<T: CpuRegister, U: CpuBus>(operand: Addr, register: &mut T, bus: &mut U) {
  let a = register.get_A();
  let fetched = bus.read(operand);
  let computed = fetched as u16 + a as u16 + bool2u8(register.get_status_carry()) as u16;
  let result = (computed & 0xff) as u8;

  register
    .set_status_overflow(((a ^ result) & (fetched ^ result) & 0x80) == 0x80)
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed > 0x00ff)
    .set_A(result);
}

pub fn and_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  let c = register.get_A() & (operand as Data);
  register
    .update_status_negative_by(c)
    .update_status_zero_by(c)
    .set_A(c);
}

pub fn and<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let c = register.get_A() & (bus.read(operand));
  register
    .update_status_negative_by(c)
    .update_status_zero_by(c)
    .set_A(c);
}

pub fn asl_acc<T: CpuRegister>(register: &mut T) {
  let a = register.get_A() << 1;
  register
    .set_status_carry((a & 0x80) == 0x80)
    .update_status_negative_by(a)
    .update_status_zero_by(a)
    .set_A(a);
}

pub fn asl<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let fetched = bus.read(operand) << 1;
  register
    .set_status_carry((fetched & 0x80) == 0x80)
    .update_status_negative_by(fetched)
    .update_status_zero_by(fetched);
  bus.write(operand, fetched);
}

pub fn bit<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let fetched = bus.read(operand);
  let a = register.get_A();
  register
    .update_status_negative_by(fetched)
    .update_status_zero_by(a & fetched)
    .set_status_overflow((fetched & 0x40) == 0x40);
}

#[cfg(test)]
mod test {
  use super::super::super::cpu_register::Register;
  use super::*;

  struct MockBus {
    pub memory: Vec<Data>,
  }

  impl MockBus {
    fn new() -> Self {
      MockBus {
        memory: vec!(0; 256)
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
  fn test_lda_imm() {
    let mut r = Register::new();
    lda_imm(0xA9, &mut r);
    assert_eq!(r.get_A(), 0xA9);
  }

  #[test]
  fn test_lda() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0xA5] = 0xFF;
    lda(0xA5, &mut r, &mut b);
    assert_eq!(r.get_A(),0xFF);
  }

  #[test]
  fn test_ldx_imm() {
    let mut r = Register::new();
    ldx_imm(0xFF, &mut r);
    assert_eq!(r.get_X(), 0xFF)
  }

  #[test]
  fn test_ldx() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0xA5] = 0xFF;
    ldx(0xA5, &mut r, &mut b);
    assert_eq!(r.get_X(),0xFF)
  }

  #[test]
  fn test_ldy_imm() {
    let mut r = Register::new();
    ldy_imm(0xFF, &mut r);
    assert_eq!(r.get_Y(), 0xFF)
  }

  #[test]
  fn test_ldy() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0xA5] = 0xFF;
    ldy(0xA5, &mut r, &mut b);
    assert_eq!(r.get_Y(),0xFF)
  }

  #[test]
  fn test_sta() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0xFF);
    sta(0x11, &mut r, &mut b);
    assert_eq!(b.read(0x11), 0xFF)
  }

  #[test]
  fn test_stx() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_X(0xFF);
    stx(0x11, &mut r, &mut b);
    assert_eq!(b.read(0x11), 0xFF)
  }

  #[test]
  fn test_sty() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_Y(0xFF);
    sty(0x11, &mut r, &mut b);
    assert_eq!(b.read(0x11), 0xFF)
  }

  #[test]
  fn test_tax() {
    let mut r = Register::new();
    r.set_A(0xFF);
    tax(&mut r);
    assert_eq!(r.get_X(),0xFF)
  }

  #[test]
  fn test_tay() {
    let mut r = Register::new();
    r.set_A(0xFF);
    tay(&mut r);
    assert_eq!(r.get_Y(),0xFF)
  }

  #[test]
  fn test_tsx() {
    let mut r = Register::new();
    r.set_S(0xFF);
    tsx(&mut r);
    assert_eq!(r.get_X(),0xFF)
  }


  #[test]
  fn test_txa() {
    let mut r = Register::new();
    r.set_X(0xFF);
    txa(&mut r);
    assert_eq!(r.get_A(),0xFF)
  }

  #[test]
  fn test_txs() {
    let mut r = Register::new();
    r.set_X(0xFF);
    txs(&mut r);
    assert_eq!(r.get_S(),0xFF)
  }

  #[test]
  fn test_tya() {
    let mut r = Register::new();
    r.set_Y(0xFF);
    tya(&mut r);
    assert_eq!(r.get_A(),0xFF)
  }

  #[test]
  fn test_adc_imm() {
    let mut r = Register::new();
    r.set_A(0x01);
    adc_imm(0x03, &mut r);
    assert_eq!(r.get_A(),0x04);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x01);
    r.set_status_carry(true);
    adc_imm(0x03, &mut r);
    assert_eq!(r.get_A(),0x05);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x01);
    adc_imm(0x7F, &mut r);
    assert_eq!(r.get_A(),0x80);
    assert_eq!(r.get_status_overflow(), true);

    // Unconfirmed?
    r.set_A(0x01);
    adc_imm(0x80, &mut r);
    assert_eq!(r.get_status_overflow(), false);
    // Unconfirmed?
    r.set_A(0x80);
    adc_imm(0x01, &mut r);
    assert_eq!(r.get_status_overflow(), false);
  }

  #[test]
  fn test_adc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x01);
    b.memory[0x11] = 0x03;
    adc(0x11, &mut r, &mut b);
    assert_eq!(r.get_A(),0x04);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x01);
    r.set_status_carry(true);
    b.memory[0x11] = 0x03;
    adc(0x11, &mut r, &mut b);
    assert_eq!(r.get_A(),0x05);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x01);
    b.memory[0x11] = 0x7F;
    adc(0x11, &mut r, &mut b);
    assert_eq!(r.get_A(),0x80);
    assert_eq!(r.get_status_overflow(), true);

    // Unconfirmed?
    r.set_A(0x01);
    b.memory[0x11] = 0x80;
    adc(0x11, &mut r, &mut b);
    assert_eq!(r.get_status_overflow(), false);
    // Unconfirmed?
    r.set_A(0x80);
    b.memory[0x11] = 0x01;
    adc(0x11, &mut r, &mut b);
    assert_eq!(r.get_status_overflow(), false)
  }

  #[test]
  fn test_and_imm() {
    let mut r = Register::new();
    r.set_A(0x11);
    and_imm(0x01, &mut r);
    assert_eq!(r.get_A(), 0x01)
  }

  #[test]
  fn test_and() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x11);
    b.memory[0x80] = 0x01;
    and(0x80, &mut r, &mut b);
    assert_eq!(r.get_A(), 0x01)
  }

  #[test]
  fn test_asl_acc() {
    let mut r = Register::new();
    r.set_A(0x01);
    asl_acc(&mut r);
    assert_eq!(r.get_A(), 0x02)
  }

  #[test]
  fn test_asl() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0x80] = 0x01;
    asl(0x80, &mut r, &mut b);
    assert_eq!(b.read(0x80), 0x02)
  }

  #[test]
  fn test_bit() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x40);
    b.memory[0x80] = 0x40;
    bit(0x80, &mut r, &mut b);
    assert_eq!(r.get_status_zero(), false);
    assert_eq!(r.get_status_negative(), false);
    assert_eq!(r.get_status_overflow(), true)
  }
}