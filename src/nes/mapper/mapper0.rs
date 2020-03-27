use super::mapper::*;
use super::Data;
use super::Addr;
use super::Ram;
use super::Rom;

#[derive(Debug)]
pub struct Mapper0 {

}

impl Mapper0 {
  pub fn new() -> Self {
    Mapper0 {}
  }
}

impl Mapper for Mapper0 {
  fn get_cram_index(&self, addr: Addr)-> Addr {
    addr
  }

  fn read(&mut self, addr: Addr, prg_rom: &Rom, sram: &Ram) -> Data {
    match addr {
      0x6000..=0x7FFF => {
        println!("Not implemented. This area is battery backup ram area 0x{:x}", addr );
        0
      }
      0x8000..=0xBFFF => prg_rom.read(addr - 0x8000),
      0xC000..=0xFFFF if prg_rom.size() <= 0x4000 => {
        prg_rom.read(addr - 0xC000)
      }
      0xC000..=0xFFFF => prg_rom.read(addr - 0x8000),
      _ => panic!("[READ] There is an illegal address (0x{:x}) access on Mapper.", addr),
    }
  }

  fn write(&mut self, addr: Addr, data: Data, prg_rom: &Rom, sram: &mut Ram) {
    match addr {
      0x6000..=0x7FFF => {
        println!("Not implemented. This area is battery backup ram area 0x{:x}", addr );
      }
      0x8000..=0xFFFF => {
        println!("current not supported")
      }
      _ => panic!("[WRITE] There is an illegal address (0x{:x}) access on Mapper.", addr),
    }
  }
}