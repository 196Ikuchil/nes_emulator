use super::super::super::types::{Data, Addr};

#[derive(Debug)]
pub struct PpuAddr {
  addr: Addr,
  is_lower_addr: bool,
}

// Address ($2006)
impl PpuAddr {
  pub fn new() -> Self {
    PpuAddr {
      addr: 0,
      is_lower_addr: false,
    }
  }

  pub fn get(&self) -> Addr {
    self.addr
  }

  pub fn reset_latch(&mut self) {
    self.is_lower_addr = false
  }

  pub fn update(&mut self, offset: Addr) {
    self.addr += offset
  }

  pub fn write(&mut self, data: Data) {
    if self.is_lower_addr {
      self.addr += data as Addr;
      self.is_lower_addr = false;
    } else {
      self.addr = (data as Addr) << 8;
      self.is_lower_addr = true
    }
  }
}


#[test]
fn set_addr() {
    let mut reg = PpuAddr::new();
    reg.write(0xFF);
    reg.write(0x55);
    assert_eq!(reg.get(), 0xFF55);
}

#[test]
fn update_addr() {
    let mut reg = PpuAddr::new();
    reg.write(0xFF);
    reg.write(0x55);
    reg.update(32);
    assert_eq!(reg.get(), 0xFF75);
}