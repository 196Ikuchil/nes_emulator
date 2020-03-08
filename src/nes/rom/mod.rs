use super::types::{Data, Addr};

#[derive(Debug)]
pub struct Rom {
  vec: Vec<Data>,
}

impl Rom {
  pub fn new(buf: Vec<Data>) -> Rom {
    Rom { vec: buf.clone() }
  }

  pub fn read(&self, addr: Addr) -> Data {
    self.vec[addr as usize]
  }

  pub fn size(&self) -> usize {
    self.vec.len()
  }
}