use super::types::{Data, Addr};

#[derive(Debug)]
pub struct Ram {
  pub field: Vec<Data>,
}

impl Ram {
  pub fn new(buf: Vec<Data>) -> Ram {
    Ram { field: buf }
  }

  pub fn read(&self, addr: Addr) -> Data {
    self.field[addr as usize]
  }

  pub fn write(&mut self, addr: Addr, data: Data) {
    self.field[addr as usize] = data;
  }
}