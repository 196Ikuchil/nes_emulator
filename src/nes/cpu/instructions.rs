use super::super::types::{Data, Addr, Word};
use super::CpuRegister;
use super::CpuBus;

use std::fmt::Debug;

pub fn lda_imm<T: CpuRegister>(operand: Word, register: &mut T) {
  println!("lda imm called")
}