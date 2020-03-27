use super::mapper::*;
use super::Data;
use super::Addr;
use super::Rom;
use super::Ram;

#[derive(Debug)]
pub struct Mapper3 {
  chrBank: u8,
  prgBank1: u16,
  prgBank2: u16,
}

impl Mapper3 {
  pub fn new(rpg_rom_len: u16) -> Self {
    Mapper3 {
      chrBank: 0,
      prgBank1: 0,
      prgBank2: (rpg_rom_len / 0x4000) -1 ,
    }
  }
}

impl Mapper for Mapper3 {
  fn get_cram_index(&self, addr: Addr)-> Addr {
    self.chrBank as u16 * 0x2000 + addr
  }

  fn read(&mut self, addr: Addr, prg_rom: &Rom, sram: &Ram) -> Data {
    match addr {
      0x6000..=0x7FFF => sram.read(addr - 0x6000),
      0x8000..=0xBFFF => prg_rom.read((self.prgBank1 * 0x4000) + (addr - 0x8000)),
      0xC000..=0xFFFF => prg_rom.read((self.prgBank2 * 0x4000) + (addr - 0xC000)),
      _ => panic!("[READ] There is an illegal address (0x{:x}) access on Mapper.", addr),
    }
  }

  fn write(&mut self, addr: Addr, data: Data, prg_rom: &Rom, sram: &mut Ram) {
    match addr {
      0x6000..=0x7FFF => sram.write(addr - 0x6000, data),
      0x8000..=0xFFFF => self.chrBank = data & 0x3 as Data,
      _ => panic!("[READ] There is an illegal address (0x{:x}) access on Mapper.", addr),
    }
  }
}