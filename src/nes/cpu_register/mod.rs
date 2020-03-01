use super::helper::*;
use super::types::{Data, Addr, Word};

struct Status {
  negative: bool,
  overflow: bool,
  reserved: bool, // non usable, always true
  break_mode: bool,
  decimal_mode: bool, // non usable on nes
  interrupt: bool, // Is interrupt occured
  zero: bool,
  carry: bool,
}

#[allow(non_snake_case)]
pub struct Register {
  A: Data, // accumelator
  X: Data, // index register
  Y: Data, // index register
  S: Data, // stack pointer
  P: Status, // status register
  PC: u16,
}

impl Register {
  pub fn new() -> Self {
    Register{
      A: 0,
      X: 0,
      Y: 0,
      S: 0xFD, // 0x0100 ~ 0x01FF
      PC: 0x8000,
      P: Status{
        negative: false,
        overflow: false,
        reserved: true,
        break_mode: true,
        decimal_mode: false,
        interrupt: true,
        zero: false,
        carry: false,
      },
    }
  }
}

#[allow(non_snake_case)]
pub trait CpuRegister {
  fn get_A(&self) -> Data;
  fn get_X(&self) -> Data;
  fn get_Y(&self) -> Data;
  fn get_S(&self) -> Data;
  fn get_PC(&self) -> Addr;
  fn set_A(&mut self, v: Data) -> &mut Self;
  fn set_X(&mut self, v: Data) -> &mut Self;
  fn set_Y(&mut self, v: Data) -> &mut Self;
  fn set_S(&mut self, v: Data) -> &mut Self;
  fn set_PC(&mut self, v: Addr) -> &mut Self;
  fn get_status_negative(&self) -> bool;
  fn get_status_overflow(&self) -> bool;
  fn get_status_reserved(&self) -> bool;
  fn get_status_break_mode(&self) -> bool;
  fn get_status_decimal_mode(&self) -> bool;
  fn get_status_interrupt(&self) -> bool;
  fn get_status_zero(&self) -> bool;
  fn get_status_carry(&self) -> bool;
  fn set_status_negative(&mut self, b: bool) -> &mut Self;
  fn set_status_overflow(&mut self, b: bool) -> &mut Self;
  fn set_status_reserved(&mut self, b: bool) -> &mut Self;
  fn set_status_break_mode(&mut self, b: bool) -> &mut Self;
  fn set_status_decimal_mode(&mut self, b: bool) -> &mut Self;
  fn set_status_interrupt(&mut self, b: bool) -> &mut Self;
  fn set_status_zero(&mut self, b: bool) -> &mut Self;
  fn set_status_carry(&mut self, b: bool) -> &mut Self;

  fn update_status_negative_by(&mut self, v: Data) -> &mut Self;
  fn update_status_zero_by(&mut self, v: Data) -> &mut Self;
  fn increment_PC(&mut self) -> &mut Self;
  fn inc_S(&mut self) -> &mut Self;
  fn dec_S(&mut self) -> &mut Self;
}

impl CpuRegister for Register {
  fn get_A(&self) -> Data{
    self.A
  }

  fn get_X(&self) -> Data{
    self.X
  }

  fn get_Y(&self) -> Data{
    self.Y
  }

  fn get_S(&self) -> Data{
    self.S
  }

  fn get_PC(&self) -> Addr{
    self.PC
  }

  fn set_A(&mut self, v: Data) -> &mut Self {
    self.A = v;
    self
  }

  fn set_X(&mut self, v: Data) -> &mut Self {
    self.X = v;
    self
  }

  fn set_Y(&mut self, v: Data) -> &mut Self {
    self.Y = v;
    self
  }

  fn set_S(&mut self, v: Data) -> &mut Self {
    self.S = v;
    self
  }

  fn set_PC(&mut self, v: Addr) -> &mut Self {
    self.PC = v;
    self
  }

  fn get_status_negative(&self) -> bool {
    self.P.negative
  }

  fn get_status_overflow(&self) -> bool {
    self.P.overflow
  }

  fn get_status_reserved(&self) -> bool {
    self.P.reserved
  }

  fn get_status_break_mode(&self) -> bool {
    self.P.break_mode
  }

  fn get_status_decimal_mode(&self) -> bool {
    self.P.decimal_mode
  }

  fn get_status_interrupt(&self) -> bool {
    self.P.interrupt
  }

  fn get_status_zero(&self) -> bool {
    self.P.zero
  }

  fn get_status_carry(&self) -> bool {
    self.P.carry
  }

  fn set_status_negative(&mut self, b: bool) -> &mut Self {
    self.P.negative = b;
    self
  }

  fn set_status_overflow(&mut self, b: bool) -> &mut Self {
    self.P.overflow = b;
    self
  }

  fn set_status_reserved(&mut self, b: bool) -> &mut Self {
    self.P.reserved = b;
    self
  }

  fn set_status_break_mode(&mut self, b: bool) -> &mut Self {
    self.P.break_mode = b;
    self
  }

  fn set_status_decimal_mode(&mut self, b: bool) -> &mut Self {
    self.P.decimal_mode = b;
    self
  }

  fn set_status_interrupt(&mut self, b: bool) -> &mut Self {
    self.P.interrupt = b;
    self
  }

  fn set_status_zero(&mut self, b: bool) -> &mut Self {
    self.P.zero = b;
    self
  }

  fn set_status_carry(&mut self, b: bool) -> &mut Self {
    self.P.carry = b;
    self
  }


  fn update_status_negative_by(&mut self, v: Data) -> &mut Self {
    self.set_status_negative((v & 0x80) == 0x80) // set bit 7 of culc result
  }

  fn update_status_zero_by(&mut self, v: Data) -> &mut Self {
    self.set_status_zero(v == 0)
  }

  fn increment_PC(&mut self) -> &mut Self {
    self.PC += 1;
    self
  }

  fn inc_S(&mut self) -> &mut Self {
    self.S += 1;
    self
  }

  fn dec_S(&mut self) -> &mut Self {
    self.S -= 1;
    self
  }
}

#[test]
fn get_s() {
  let r = Register::new();
  let s = r.get_S();
  assert_eq!(s, 0xFD);
}

#[test]
fn set_a() {
  let mut r = Register::new();
  let a = r.set_A(0x01);
  assert_eq!(a.get_A(), 0x01)
}