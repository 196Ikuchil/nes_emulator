use super::super::types::{Data, Addr, Word};
use super::super::dma::Dma;
use super::super::ppu::Ppu;
use super::super::ram::Ram;

pub struct Bus<'a> {
  work_ram: &'a mut Ram,
  ppu: &'a mut Ppu,
  dma: &'a mut Dma,
}

pub trait CpuBus {
  fn read(&mut self, addr: Addr) -> Data;
  fn read_word(&mut self, addr: Addr) -> Word;
  fn write(&mut self, addr: Addr, data: Data);
}

impl<'a> Bus<'a> {
  pub fn new(
    work_ram: &'a mut Ram,
    ppu: &'a mut Ppu,
    dma: &'a mut Dma,
  ) -> Bus<'a> {
    Self {
      work_ram,
      ppu,
      dma,
    }
  }
}

impl<'a> CpuBus for Bus<'a> {
  fn read_word(&mut self, addr: Addr) -> Word {
    let lower = self.read(addr) as Word;
    let upper = self.read(addr + 1) as Word;
    (upper << 8 | lower) as Word
  }

  fn read(&mut self, addr: Addr) -> Data {
    match addr {
      0x0000..=0x1FFF => self.work_ram.read(addr & 0x07FF),
      0x2000..=0x3FFF => self.ppu.read(addr - 0x2000),
      0x6000..=0x7FFF => {
        println!("Not implemented. This area is battery backup ram area 0x{:x}", addr );
        0
      }
      _ => panic!("[READ] There is an illegal address (0x{:x}) access.", addr),
    }
  }

  fn write(&mut self, addr: Addr, data: Data) {
    match addr {
      0x0000..=0x1FFF => self.work_ram.write(addr & 0x07FF, data),
      0x2000..=0x3FFF => self.ppu.write(addr - 0x2000, data),
      0x4014 => self.dma.write(data),
      0x6000..=0x7FFF => {
        println!("Not implemented. This area is battery backup ram area 0x{:x}", addr );
      }
      _ => panic!("[WRITE] There is an illegal address (0x{:x}) access.", addr),
    }
  }
}