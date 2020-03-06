use super::super::super::types::{Data, Addr};
use super::super::super::Ram;

#[derive(Debug)]
pub struct PpuData {
  buf: Data,
}

// Data ($2007)
impl PpuData {
  pub fn new() -> Self {
    PpuData { buf: 0 }
  }


  pub fn read(&mut self, addr: Addr) -> Data {
    // vram
    if addr >= 0x2000 {
      let addr = self.calc_addr(addr);
      // palette
      // Reading palette data from $3F00-$3FFF works differently.
      // The palette data is placed immediately on the data bus, and hence no dummy read is required.
      // Reading the palettes still updates the internal buffer though, but the data placed in it is the mirrored nametable data
      // that would appear "underneath" the palette. (Checking the PPU memory map should make this clearer.)
      if addr >= 0x3F00 { // ?

      }
    }
    // cram
    else {

    }
    self.buf
  }

  pub fn write(&mut self, addr: Addr, data: Data){
    if addr >= 0x2000 {
      if addr >= 0x3f00 && addr < 0x4000 { // palette
      } else { // vram

      }
    } else { // cram

    }
  }

  fn calc_addr(&self, addr: Addr) -> Addr {
    if addr >= 0x3000 && addr < 0x3F00 {
       addr - 0x3000
    } else {
      addr - 0x2000
    }
  }
}