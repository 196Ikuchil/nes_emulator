use super::bus::{Bus};
use super::super::types::{Data, Addr, Word};

pub trait CpuBus {
  fn read(&mut self, addr: Addr) -> Data;
  fn read_word(&mut self, addr: Addr) -> Word;
  fn write(&mut self, addr: Addr);
}

impl<'a> CpuBus for Bus<'a> {
  fn read(&mut self, addr: Addr) -> Data {
    0x00 as Data
  }

  fn read_word(&mut self, addr: Addr) -> Word {
    0x00 as Word
  }

  fn write(&mut self, addr: Addr) {

  }
}