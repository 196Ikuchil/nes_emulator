use super::types::{Data, Addr};

#[derive(Debug)]
pub struct Ram {
  pub field: Vec<Data>,
}

extern "C" {
  fn save_sram(ptr: *const Data, len: usize);
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

  pub fn size(&self) -> usize {
    self.field.len()
  }

  pub fn save(&self) {
    unsafe {
      save_sram(self.field.as_ptr(),self.field.len());
    }
  }
}