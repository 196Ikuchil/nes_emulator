use super::helper::*;
use super::types::{Data, Addr, Word};

#[derive(Debug)]
struct Status {
  negative: bool,
  overflow: bool,
  reserved: bool, // non usable, always true
  break_mode: bool,
  decimal_mode: bool, // non usable on nes
  interrupt: bool, // interrupt disable flag
  zero: bool,
  carry: bool,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Register {
  A: Data, // accumelator
  X: Data, // index register
  Y: Data, // index register
  S: Data, // stack pointer
  P: Status, // status register
  PC: u16,
  interrupt_type: InterruptType,
  is_page_crossed: bool,
  additional_cycle: Data,
}

pub enum InterruptType {
  NONE,
  INTERRUPT_IRQ,
  INTERRUPT_NMI, // TODO
}

impl std::fmt::Debug for InterruptType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", "derp")
  }
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
      interrupt_type: InterruptType::NONE,
      is_page_crossed: false,
      additional_cycle: 0,
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
  fn get_interrupt_type(&self) -> InterruptType;
  fn is_interrupt_irq_enabled(&self) -> bool;
  fn set_status_negative(&mut self, b: bool) -> &mut Self;
  fn set_status_overflow(&mut self, b: bool) -> &mut Self;
  fn set_status_reserved(&mut self, b: bool) -> &mut Self;
  fn set_status_break_mode(&mut self, b: bool) -> &mut Self;
  fn set_status_decimal_mode(&mut self, b: bool) -> &mut Self;
  fn set_status_interrupt(&mut self, b: bool) -> &mut Self;
  fn set_status_zero(&mut self, b: bool) -> &mut Self;
  fn set_status_carry(&mut self, b: bool) -> &mut Self;
  fn set_interrupt_type(&mut self, t: InterruptType) -> &mut Self;
  fn set_interrupt_irq(&mut self) -> &mut Self;
  fn set_interrupt_none(&mut self) -> &mut Self;
  fn update_status_negative_by(&mut self, v: Data) -> &mut Self;
  fn update_status_zero_by(&mut self, v: Data) -> &mut Self;
  fn increment_PC(&mut self) -> &mut Self;
  fn decrement_PC(&mut self) -> &mut Self;
  fn inc_S(&mut self) -> &mut Self;
  fn dec_S(&mut self) -> &mut Self;
  fn get_Status(&mut self) -> Data;
  fn set_Status(&mut self, v: Data) -> &mut Self;
  fn get_page_crossed(&self) -> bool;
  fn set_page_crossed(&mut self, b: bool);

  fn page_differ(&mut self, addr1: Addr, addr2: Addr) -> bool;
  fn add_interrupt_cycle(&mut self);
  fn add_branch_cycle(&mut self, addr1: Addr, addr2: Addr);
  fn pop_additional_cycle(&mut self) -> Data;
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

  fn get_interrupt_type(&self) -> InterruptType {
    match self.interrupt_type {
      InterruptType::INTERRUPT_IRQ => InterruptType::INTERRUPT_IRQ,
      InterruptType::INTERRUPT_NMI => InterruptType::INTERRUPT_NMI, // current not used
      _ => InterruptType::NONE,
    }
  }

  fn is_interrupt_irq_enabled(&self) -> bool {
     match self.interrupt_type {
      InterruptType::INTERRUPT_IRQ => true,
      _ => false,
     }
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

  fn set_interrupt_type(&mut self, t: InterruptType) -> &mut Self {
    self.interrupt_type = t;
    self
  }

  fn set_interrupt_irq(&mut self) -> &mut Self {
    if !self.get_status_interrupt(){
      self.set_interrupt_type(InterruptType::INTERRUPT_IRQ);
    }
    self
  }

  fn set_interrupt_none(&mut self) -> &mut Self {
    self.set_interrupt_type(InterruptType::NONE)
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

  fn decrement_PC(&mut self) -> &mut Self {
    self.PC -= 1;
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

  fn get_Status(&mut self) -> Data {
    bool2u8(self.P.negative) << 7 | bool2u8(self.P.overflow) << 6 |
    bool2u8(self.P.reserved) << 5 | bool2u8(self.P.break_mode) << 4 |
    bool2u8(self.P.decimal_mode) << 3 | bool2u8(self.P.interrupt) << 2 |
    bool2u8(self.P.zero) << 1 | bool2u8(self.P.carry) as Data
  }

  fn set_Status(&mut self, v: Data) -> &mut Self {
    self.P.negative = v & 0x80 == 0x80;
    self.P.overflow = v & 0x40 == 0x40;
    self.P.reserved = v & 0x20 == 0x20;
    self.P.break_mode = v & 0x10 == 0x10;
    self.P.decimal_mode = v & 0x08 == 0x08;
    self.P.interrupt = v & 0x04 == 0x04;
    self.P.zero = v & 0x02 == 0x02;
    self.P.carry = v & 0x01 == 0x01;
    self
  }

  fn get_page_crossed(&self) -> bool {
    self.is_page_crossed
  }

  fn set_page_crossed(&mut self, b: bool) {
    self.is_page_crossed = b;
  }

  fn page_differ(&mut self, addr1: Addr, addr2: Addr) -> bool {
    (addr1 & 0xFF00) != (addr2 & 0xFF00)
  }

  fn add_interrupt_cycle(&mut self) {
    self.additional_cycle += 7;
  }

  fn add_branch_cycle(&mut self, addr1: Addr, addr2: Addr) {
    self.additional_cycle += 1;
    if self.page_differ(addr1, addr2) {
      self.additional_cycle +=1;
    }
  }

  fn pop_additional_cycle(&mut self) -> Data {
    let c = self.additional_cycle;
    self.additional_cycle = 0;
    c
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

#[test]
fn test_set_Status() {
  let mut r = Register::new();
  r.set_Status(0xFF);
  assert_eq!(r.get_status_negative(),true);
  assert_eq!(r.get_status_overflow(),true);
  assert_eq!(r.get_status_reserved(),true);
  assert_eq!(r.get_status_break_mode(), true);
  assert_eq!(r.get_status_decimal_mode(), true);
  assert_eq!(r.get_status_interrupt(), true);
  assert_eq!(r.get_status_zero(), true);
  assert_eq!(r.get_status_carry(),true);
}

#[test]
fn test_get_Status() {
  let mut r = Register::new();
  r.set_Status(0xFF);
  assert_eq!(r.get_Status(),0xFF);
}