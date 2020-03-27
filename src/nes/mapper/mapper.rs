use super::Data;
use super::Addr;
use super::Ram;
use super::Rom;

pub trait Mapper {
  fn get_cram_index(&self, addr: Addr) -> Addr; // for ppu
  fn read(&mut self, addr: Addr, prg_rom: &Rom, sram: &Ram) -> Data;
  fn write(&mut self, addr: Addr, data: Data, prg_rom: &Rom, sram: &mut Ram);
}

impl std::fmt::Debug for dyn Mapper {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", "derp")
  }
}