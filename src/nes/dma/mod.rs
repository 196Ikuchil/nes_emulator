use super::types::{Data, Addr};
use super::ram::Ram;
use super::ppu::Ppu;

#[derive(Debug)]
pub struct Dma {
  start_addr_top: Data,
  should_run: bool,
}

impl Dma {
  pub fn new() -> Self {
    Dma {
      start_addr_top: 0,
      should_run: false,
    }
  }

  pub fn write(&mut self, addr_top: Data) {
    self.start_addr_top = addr_top;
    self.should_run = true;
  }

  pub fn is_should_run(&self) -> bool {
    self.should_run
  }

  pub fn run(&mut self, ram: &Ram, ppu: &mut Ppu) {
    let addr = (self.start_addr_top as Addr) << 8;
    for i in 0..0x100 {
      ppu.transfer_sprite(i, ram.read(addr + i));
    }
    self.should_run = false;
  }
}