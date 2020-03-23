use super::Data;
use super::Addr;
use super::Rom;

pub trait Mapper {
  fn get_cram_index(&self, addr: Addr) -> Addr; // for ppu
  fn read(&mut self, addr: Addr, prg_rom: &Rom) -> Data;
  fn write(&mut self, addr: Addr, data: Data, prg_rom: &Rom);
}

impl std::fmt::Debug for dyn Mapper {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", "derp")
  }
}