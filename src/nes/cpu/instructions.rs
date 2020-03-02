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

pub fn cmp_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  let computed = register.get_A() as i16 - operand as i16;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0);
}

pub fn cmp<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let computed = register.get_A() as i16 - bus.read(operand) as i16;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0);
}

pub fn cpx_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  let computed = register.get_X() as i16 - operand as i16;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0);
}

pub fn cpx<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let computed = register.get_X() as i16 - bus.read(operand) as i16;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0);
}

pub fn cpy_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  let computed = register.get_Y() as i16 - operand as i16;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0);
}

pub fn cpy<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let computed = register.get_Y() as i16 - bus.read(operand) as i16;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0);
}

pub fn dec<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let computed = bus.read(operand) as i8 -1;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data);
  bus.write(operand, computed as Data)
}

pub fn dex<T: CpuRegister>(register: &mut T) {
  let x = register.get_X() as i8 -1;
  register
    .update_status_negative_by(x as Data)
    .update_status_zero_by(x as Data)
    .set_X(x as Data);
}

pub fn dey<T: CpuRegister>(register: &mut T) {
  let y = register.get_Y() as i8 -1;
  register
    .update_status_negative_by(y as Data)
    .update_status_zero_by(y as Data)
    .set_Y(y as Data);
}

pub fn eor_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  let computed = operand as Data ^ register.get_A();
  register
    .update_status_negative_by(computed)
    .update_status_zero_by(computed)
    .set_A(computed);
}

pub fn eor<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let computed = bus.read(operand) ^ register.get_A();
  register
    .update_status_negative_by(computed)
    .update_status_zero_by(computed)
    .set_A(computed);
}

pub fn inc<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let computed = bus.read(operand) as i8 +1;
  register
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data);
  bus.write(operand, computed as Data)
}

pub fn inx<T: CpuRegister>(register: &mut T) {
  let x = register.get_X() as i8 +1;
  register
    .update_status_negative_by(x as Data)
    .update_status_zero_by(x as Data)
    .set_X(x as Data);
}

pub fn iny<T: CpuRegister>(register: &mut T) {
  let y = register.get_Y() as i8 +1;
  register
    .update_status_negative_by(y as Data)
    .update_status_zero_by(y as Data)
    .set_Y(y as Data);
}

pub fn lsr_acc<T: CpuRegister>(register: &mut T) {
  let a = register.get_A();
  let shifted = (a >> 1) as u8;
  register
    .set_status_carry((a & 0x01) == 0x01)
    .update_status_negative_by(shifted)
    .update_status_zero_by(shifted)
    .set_A(shifted);
}

pub fn lsr<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let fetched = bus.read(operand);
  let shifted = (fetched >> 1) as u8;
  register
    .set_status_carry((fetched & 0x01) == 0x01)
    .update_status_negative_by(shifted)
    .update_status_zero_by(shifted);
  bus.write(operand, shifted)
}

pub fn ora_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  let or = register.get_A() | operand as Data;
  register
    .update_status_negative_by(or)
    .update_status_zero_by(or)
    .set_A(or);
}
pub fn ora<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let or = register.get_A() | bus.read(operand);
  register
    .update_status_negative_by(or)
    .update_status_zero_by(or)
    .set_A(or);
}

pub fn rol_acc<T: CpuRegister>(register: &mut T) {
  let a = register.get_A();
  let computed = rotate_to_left(register, a);
  register
    .set_status_carry((a & 0x80) == 0x80)
    .update_status_negative_by(computed)
    .update_status_zero_by(computed)
    .set_A(computed);
}

pub fn rol<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let fetched = bus.read(operand);
  let computed = rotate_to_left(register, fetched);
  register
    .set_status_carry((fetched & 0x80) == 0x80)
    .update_status_negative_by(computed)
    .update_status_zero_by(computed);
  bus.write(operand, computed);
}

pub fn ror_acc<T: CpuRegister>(register: &mut T) {
  let a = register.get_A();
  let computed = rotate_to_right(register, a);
  register
    .set_status_carry((a & 0x01) == 0x01)
    .update_status_negative_by(computed)
    .update_status_zero_by(computed)
    .set_A(computed);
}

pub fn ror<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let fetched = bus.read(operand);
  let computed = rotate_to_right(register, fetched);
  register
    .set_status_carry((fetched & 0x01) == 0x01)
    .update_status_negative_by(computed)
    .update_status_zero_by(computed);
  bus.write(operand, computed);
}

pub fn sbc_imm<T:CpuRegister>(operand: Word, register: &mut T) {
  let computed = (register.get_A() as i16) - (operand as i16) - (bool2u8(!register.get_status_carry()) as i16);
  let a = register.get_A();
  register
    .set_status_overflow( (((a ^ operand as Data) & 0x80) != 0) && (((a ^ computed as Data) & 0x80)!= 0))
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0)
    .set_A(computed as Data);
}

pub fn sbc<T: CpuRegister, U: CpuBus>(operand: Word, register: &mut T, bus: &mut U) {
  let fetched = bus.read(operand);
  let computed = (register.get_A() as i16) - (fetched as i16) - (bool2u8(!register.get_status_carry()) as i16);
  let a = register.get_A();
  register
    .set_status_overflow( (((a ^ fetched as Data) & 0x80) != 0) && (((a ^ computed as Data) & 0x80)!= 0))
    .update_status_negative_by(computed as Data)
    .update_status_zero_by(computed as Data)
    .set_status_carry(computed >= 0)
    .set_A(computed as Data);
}

pub fn pha<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) {
  push(register.get_A(), register, bus);
}

pub fn php<T:CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U){
  push(register.get_Status(), register, bus); // registers.set_break(true);?
}


pub fn pla<T:CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) {
  let v = pop(register, bus);
  register
    .update_status_negative_by(v)
    .update_status_zero_by(v)
    .set_A(v);
}

pub fn plp<T:CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) {
  let v = pop(register, bus); //  registers.set_reserved(true);?
  register.set_Status(v);
}

pub fn jmp<T:CpuRegister>(operand: Addr, register: &mut T) {
  register.set_PC(operand);
}

pub fn jsr<T:CpuRegister, U: CpuBus>(operand: Addr, register: &mut T, bus: &mut U) {
  let addr = register.get_PC() - 1; // auto incremented in fetch
  push((addr >> 8) as u8, register, bus);
  push(addr as u8, register, bus);
  register.set_PC(operand);
}

pub fn rts<T:CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) {
  pop_pc(register, bus);
  register.increment_PC();
}

pub fn rti<T:CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) {
  pop_status(register, bus);
  pop_pc(register, bus);
  register.increment_PC();
}

pub fn bcc<T:CpuRegister>(operand: Addr, register: &mut T) {
  if !register.get_status_carry() {
    branch(operand, register);
  }
}

pub fn bcs<T:CpuRegister>(operand: Addr, register: &mut T) {
  if register.get_status_carry() {
    branch(operand, register);
  }
}



fn push<T:CpuRegister, U: CpuBus>(data: Data, register: &mut T, bus: &mut U) {
  let addr = register.get_S() as Addr;
  bus.write(0x0100 | addr, data);
  register.dec_S();
}

fn pop<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) -> Data {
  register.inc_S();
  let stack = register.get_S() as Addr;
  bus.read(0x0100 | stack)

}

fn pop_pc<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) {
  let lower = pop(register, bus) as u16;
  let upper = pop(register, bus) as u16;
  register.set_PC(upper << 8 | lower);
}

fn pop_status<T: CpuRegister, U: CpuBus>(register: &mut T, bus: &mut U) {
  let p = pop(register, bus);
  register.set_Status(p);
}

fn rotate_to_left<T: CpuRegister>(register: &mut T, v: Data) -> Data {
  ((v << 1) as Data | if register.get_status_carry() { 0x01 } else { 0x00 }) as Data
}

fn rotate_to_right<T: CpuRegister>(register: &mut T, v: Data) -> Data {
  ((v >> 1) as Data | if register.get_status_carry() { 0x80 } else { 0x00 }) as Data
}

fn branch<T: CpuRegister>(addr: Addr, register: &mut T) {
  register.set_PC(addr);
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

  #[test]
  fn test_cmp_imm() {
    let mut r = Register::new();
    r.set_A(0x40);
    cmp_imm(0x50, &mut r);
    assert_eq!(r.get_status_negative(), true);
    assert_eq!(r.get_status_carry(), false)
  }

  #[test]
  fn test_cmp() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x40);
    b.memory[0x80] = 0x50;
    cmp(0x80, &mut r, &mut b);
    assert_eq!(r.get_status_negative(), true);
    assert_eq!(r.get_status_carry(), false)
  }

  #[test]
  fn test_cpx_imm() {
    let mut r = Register::new();
    r.set_X(0x40);
    cpx_imm(0x50, &mut r);
    assert_eq!(r.get_status_negative(), true);
    assert_eq!(r.get_status_carry(), false)
  }

  #[test]
  fn test_cpx() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_X(0x40);
    b.memory[0x80] = 0x50;
    cpx(0x80, &mut r, &mut b);
    assert_eq!(r.get_status_negative(), true);
    assert_eq!(r.get_status_carry(), false)
  }


  #[test]
  fn test_cpy_imm() {
    let mut r = Register::new();
    r.set_Y(0x40);
    cpx_imm(0x50, &mut r);
    assert_eq!(r.get_status_negative(), true);
    assert_eq!(r.get_status_carry(), false)
  }

  #[test]
  fn test_cpy() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_Y(0x40);
    b.memory[0x80] = 0x50;
    cpx(0x80, &mut r, &mut b);
    assert_eq!(r.get_status_negative(), true);
    assert_eq!(r.get_status_carry(), false)
  }

  #[test]
  fn test_dec() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0x80] = 0x02;
    dec(0x80, &mut r, &mut b);
    assert_eq!(b.memory[0x80], 0x01)
  }

  #[test]
  fn test_dex() {
    let mut r = Register::new();
    r.set_X(0x02);
    dex(&mut r);
    assert_eq!(r.get_X(), 0x01)
  }

  #[test]
  fn test_dey() {
    let mut r = Register::new();
    r.set_Y(0x02);
    dey(&mut r);
    assert_eq!(r.get_Y(), 0x01)
  }

  #[test]
  fn test_eor_imm() {
    let mut r = Register::new();
    r.set_A(0xF0);
    eor_imm(0x0F, &mut r);
    assert_eq!(r.get_A(),0xFF)
  }

  #[test]
  fn test_eor() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x0F);
    b.memory[0x80] = 0xF0;
    eor(0x80, &mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_inc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0x80] = 0x02;
    inc(0x80, &mut r, &mut b);
    assert_eq!(b.memory[0x80], 0x03)
  }

  #[test]
  fn test_inx() {
    let mut r = Register::new();
    r.set_X(0x02);
    inx(&mut r);
    assert_eq!(r.get_X(), 0x03)
  }

  #[test]
  fn test_iny() {
    let mut r = Register::new();
    r.set_Y(0x02);
    iny(&mut r);
    assert_eq!(r.get_Y(), 0x03)
  }

  #[test]
  fn test_lsr_acc() {
    let mut r = Register::new();
    r.set_A(0x01);
    lsr_acc(&mut r);
    assert_eq!(r.get_A(), 0x00);
    assert_eq!(r.get_status_carry(), true);

    r.set_A(0x02);
    lsr_acc(&mut r);
    assert_eq!(r.get_A(),0x01);
    assert_eq!(r.get_status_carry(), false)
  }

  #[test]
  fn test_lsr() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0x80] = 0x01;
    lsr(0x80, &mut r, &mut b);
    assert_eq!(b.read(0x80), 0x00);
    assert_eq!(r.get_status_carry(), true);

    b.memory[0x80] = 0x02;
    lsr(0x80, &mut r, &mut b);
    assert_eq!(b.read(0x80), 0x01);
    assert_eq!(r.get_status_carry(), false);
  }


  #[test]
  fn test_ora_imm() {
    let mut r = Register::new();
    r.set_A(0xF0);
    ora_imm(0x0F, &mut r);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_ora() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0xF0);
    b.memory[0x80] = 0xF;
    ora(0x80, &mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF)
  }

  #[test]
  fn test_rol_acc() {
    let mut r = Register::new();
    r.set_A(0x01);
    rol_acc(&mut r);
    assert_eq!(r.get_A(), 0x02);

    r.set_A(0x01);
    r.set_status_carry(true);
    rol_acc(&mut r);
    assert_eq!(r.get_A(), 0x03);
  }

  #[test]
  fn test_rol() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0x80] = 0x01;
    rol(0x80, &mut r, &mut b);
    assert_eq!(b.memory[0x80], 0x02);

    b.memory[0x80] = 0x01;
    r.set_status_carry(true);
    rol(0x80, &mut r, &mut b);
    assert_eq!(b.memory[0x80], 0x03);
  }

  #[test]
  fn test_ror_acc() {
    let mut r = Register::new();
    r.set_A(0x02);
    ror_acc(&mut r);
    assert_eq!(r.get_A(), 0x01);

    r.set_A(0x00);
    r.set_status_carry(true);
    ror_acc(&mut r);
    assert_eq!(r.get_A(), 0x80);
  }

  #[test]
  fn test_ror() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    b.memory[0x80] = 0x02;
    ror(0x80, &mut r, &mut b);
    assert_eq!(b.memory[0x80], 0x01);

    b.memory[0x80] = 0x00;
    r.set_status_carry(true);
    ror(0x80, &mut r, &mut b);
    assert_eq!(b.memory[0x80], 0x80);
  }

  #[test]
  fn test_sbc_imm() {
    let mut r = Register::new();
    r.set_A(0x03);
    r.set_status_carry(true);
    sbc_imm(0x02, &mut r);
    assert_eq!(r.get_A(),0x01);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x04);
    r.set_status_carry(false);
    sbc_imm(0x03, &mut r);
    assert_eq!(r.get_A(),0x00);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x01);
    r.set_status_carry(true);
    sbc_imm(0x80, &mut r);
    assert_eq!(r.get_status_overflow(), true);
  }

  #[test]
  fn test_sbc() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0x03);
    r.set_status_carry(true);
    b.memory[0x10] = 0x02;
    sbc(0x10, &mut r, &mut b);
    assert_eq!(r.get_A(),0x01);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x04);
    r.set_status_carry(false);
    b.memory[0x10] = 0x03;
    sbc(0x10, &mut r, &mut b);
    assert_eq!(r.get_A(),0x00);
    assert_eq!(r.get_status_overflow(), false);

    r.set_A(0x01);
    r.set_status_carry(true);
    b.memory[0x10] = 0x80;
    sbc(0x10, &mut r, &mut b);
    assert_eq!(r.get_status_overflow(), true);
  }

  #[test]
  fn test_pha() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_A(0xFF);
    r.set_S(0x10);
    pha(&mut r, &mut b);
    assert_eq!(b.memory[0x0110], 0xFF)
  }

  #[test]
  fn test_php() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_Status(0xFF);
    r.set_S(0x10);
    php(&mut r, &mut b);
    assert_eq!(b.memory[0x0110],0xFF)
  }

  #[test]
  fn test_pla() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_S(0x10);
    b.memory[0x0111] = 0xFF;
    pla(&mut r, &mut b);
    assert_eq!(r.get_A(), 0xFF);
  }

  #[test]
  fn test_plp() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_S(0x10);
    b.memory[0x0111] = 0xFF;
    plp(&mut r, &mut b);
    assert_eq!(r.get_Status(), 0xFF);
  }

  #[test]
  fn test_jmp() {
    let mut r = Register::new();
    jmp(0x10, &mut r);
    assert_eq!(r.get_PC(), 0x10);
  }

  #[test]
  fn test_jsr() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x0204);
    r.set_S(0x30);
    jsr(0x10, &mut r, &mut b);
    assert_eq!(r.get_PC(), 0x10);
    assert_eq!(b.memory[0x0130], 0x02);
    assert_eq!(b.memory[0x012F], 0x03);
  }

  #[test]
  fn test_rts() {
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_PC(0x0204);
    r.set_S(0x30);
    jsr(0x10, &mut r, &mut b);
    rts(&mut r, &mut b);
    assert_eq!(r.get_PC(), 0x0204);
  }

  #[test]
  fn test_rti(){
    let mut r = Register::new();
    let mut b = MockBus::new();
    r.set_S(0x30);
    r.set_Status(0xFF);
    r.set_PC(0x0204);
    jsr(0x10, &mut r, &mut b);
    php(&mut r, &mut b);
    rti(&mut r, &mut b);
    assert_eq!(r.get_PC(), 0x0204);
    assert_eq!(r.get_Status(),0xFF);
  }

  #[test]
  fn test_bcc() {
    let mut r = Register::new();
    r.set_status_carry(false);
    bcc(0x10, &mut r);
    assert_eq!(r.get_PC(), 0x10);

    r.set_status_carry(true);
    bcc(0x20, &mut r);
    assert_ne!(r.get_PC(), 0x20);
  }

  #[test]
  fn test_bcs() {
    let mut r = Register::new();
    r.set_status_carry(true);
    bcs(0x10, &mut r);
    assert_eq!(r.get_PC(), 0x10);

    r.set_status_carry(false);
    bcs(0x20, &mut r);
    assert_ne!(r.get_PC(), 0x20);
  }
}