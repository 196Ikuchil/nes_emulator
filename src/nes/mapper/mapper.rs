use super::Data;
use super::Addr;
use super::Ram;
use super::Rom;
use super::PpuConfig;
use super::Ppu;
use super::Register;

pub trait Mapper {
  fn get_cram_index(&self, addr: Addr) -> Addr; // for ppu
  fn read(&mut self, addr: Addr, prg_rom: &Rom, sram: &Ram) -> Data;
  fn write(&mut self, addr: Addr, data: Data, sram: &mut Ram, ppu_cfg: &mut PpuConfig);
  fn step(&mut self, ppu: &Ppu, cpu_register: &mut Register);
}

impl std::fmt::Debug for dyn Mapper {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", "derp")
  }
}