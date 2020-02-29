use super::super::types::{Data, Addr, Word};
use super::CpuRegister;
use super::CpuBus;

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
}