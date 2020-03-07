use super::super::super::types::{Data, Addr};
use super::super::super::ram::Ram;

#[derive(Debug)]
pub struct Oam {
  addr: Addr,
}

impl Oam {
  pub fn new() -> Self {
    Oam { addr: 0}
  }

  pub fn get_addr(&mut self) -> Addr {
    self.addr
  }

   pub fn write_addr(&mut self, data: Data) {
    self.addr = data as Addr
   }

  pub fn write_data(&mut self, oam_ram: &mut Ram, data: Data) {
    oam_ram.write(self.addr, data);
    self.addr += 1
  }

  pub fn read_data(&self, oam_ram: &Ram) -> Data {
    oam_ram.read(self.addr)
  }
}

#[test]
fn test_set_addr() {
  let mut reg = Oam::new();
  reg.write_addr(0xFF);
  assert_eq!(reg.get_addr(), 0xFF)
}