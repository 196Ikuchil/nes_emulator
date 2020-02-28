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




#[cfg(test)]
mod test {
  use super::super::super::cpu_register::Register;
  use super::*;

  #[test]
  fn test_lda_imm() {
    let mut r = Register::new();
    lda_imm(0xA9, &mut r);
    assert_eq!(r.get_A(), 0xA9);
  }
}