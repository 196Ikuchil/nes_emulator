use super::opecodes::*;
use super::super::cpu_register::CpuRegister;
use super::CpuBus;
use super::super::types::{Data, Addr, Word};

// fetch value from current pc addr
// Increment PC
pub fn fetch<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Data {
  let code = bus.read(register.get_PC());
  register.increment_PC();
  code
}

pub fn fetch_operand<T: CpuRegister, U: CpuBus>(code: &Opecode, register: &mut T, bus: &mut U) -> Word {
  match code.mode {
    Addressing::Implied => 0x0000,
    Addressing::Immediate => fetch(register, bus) as Word,
    Addressing::Zeropage => fetch(register, bus) as Addr,
    Addressing::ZeropageX => fetch_zeropage_x(register, bus) as Addr,
    Addressing::ZeropageY => fetch_zeropage_y(register, bus) as Addr,
    Addressing::Absolute => fetch_absolute(register, bus),
    Addressing::AbsoluteX => fetch_absolute_x(register, bus),
    Addressing::AbsoluteY => fetch_absolute_y(register, bus),
    Addressing::IndirectX => fetch_indirect_x(register, bus),
    Addressing::IndirectY => fetch_indirect_y(register, bus),
    _ => panic!("Invalid code"),
  }
}

fn fetch_zeropage_x<T: CpuRegister,U: CpuBus>(register: &mut T, bus: &mut U) -> Data {
  let addr = fetch(register, bus);
  addr + register.get_X()
}

fn fetch_zeropage_y<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Data {
  let addr = fetch(register, bus);
  addr + register.get_Y()
}

fn fetch_absolute<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Addr {
  let top = fetch(register, bus) as Word;
  (fetch(register,bus) as Word) << 8 | top
}

fn fetch_absolute_x<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Addr {
  fetch_absolute(register, bus) + (register.get_X() as Word)
}

fn fetch_absolute_y<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Addr {
  fetch_absolute(register, bus) + (register.get_Y() as Word)
}

fn fetch_indirect_x<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Addr {
  let addr = (fetch(register, bus) + register.get_X()) as Addr;
  (bus.read(addr) as Addr) + ((bus.read(addr+1) as Addr) << 8)
}

fn fetch_indirect_y<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Addr {
  let addr = fetch(register, bus) as Addr;
  let top = bus.read(addr) as Word;
  let low = bus.read(addr + 1) as Word;
  ((top << 8) | low) + (register.get_Y() as Word)
}

#[cfg(test)]
mod test {
  use super::super::super::cpu_register::*;
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
  fn test_fetch() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0xFF;
    let v = fetch(&mut r ,&mut b);
    assert_eq!(r.get_PC(), 0x81);
    assert_eq!(v, 0xFF)
  }

  #[test]
  fn test_fetch_zeropage_x() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_X(0x01);
    b.memory[0x80] = 0x90;
    let addr = fetch_zeropage_x(&mut r, &mut b);
    assert_eq!(addr, 0x91);
  }

  #[test]
  fn test_fetch_zeropage_y() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_Y(0x01);
    b.memory[0x80] = 0x90;
    let addr = fetch_zeropage_y(&mut r, &mut b);
    assert_eq!(addr,0x91)
  }

  #[test]
  fn test_fetch_absolute() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    b.memory[0x80] = 0x11;
    b.memory[0x81] = 0x22;
    let addr = fetch_absolute(&mut r, &mut b);
    assert_eq!(addr, 0x2211)
  }

  #[test]
  fn test_fetch_absolute_x() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_X(0x01);
    b.memory[0x80] = 0x11;
    b.memory[0x81] = 0x22;
    let addr = fetch_absolute_x(&mut r, &mut b);
    assert_eq!(addr, 0x2212)
  }

  #[test]
  fn test_fetch_absolute_y() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_Y(0x01);
    b.memory[0x80] = 0x11;
    b.memory[0x81] = 0x22;
    let addr = fetch_absolute_y(&mut r, &mut b);
    assert_eq!(addr, 0x2212)
  }

  #[test]
  fn test_fetch_indirect_x() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_X(0x01);
    b.memory[0x80] = 0x11;
    b.memory[0x12] = 0xFF;
    b.memory[0x13] = 0xEE;
    let addr = fetch_indirect_x(&mut r, &mut b);
    assert_eq!(addr,0xEEFF)
  }

  #[test]
  fn test_fetch_indirect_y() {
    let mut b = MockBus::new();
    let mut r = Register::new();
    r.set_PC(0x80);
    r.set_Y(0x01);
    b.memory[0x80] = 0x10;
    b.memory[0x10] = 0xFF;
    b.memory[0x11] = 0xEE;
    let addr = fetch_indirect_y(&mut r, &mut b);
    assert_eq!(addr, 0xFFEF);
  }
}