use super::super::super::types::{Data, Addr};
use super::super::super::Ram;
use super::super::palette::*;

#[derive(Debug)]
pub struct PpuData {
  buf: Data,
}

// Data ($2007)
// VRAM read/write data register. After access, the video memory address will increment by an amount determined by $2000:2.
impl PpuData {
  pub fn new() -> Self {
    PpuData { buf: 0 }
  }

  pub fn read<P: PaletteRam>(&mut self, vram: &Ram, cram: &Ram, addr: Addr, palette: &P) -> Data {
    let buf = self.buf;
    // vram
    if addr >= 0x2000 {
      // let addr = self.calc_addr(addr); ?
      // palette
      // Reading palette data from $3F00-$3FFF works differently.
      // The palette data is placed immediately on the data bus, and hence no dummy read is required.
      if addr >= 0x3F00 { // TODO: need debug.?
        self.buf = vram.read(addr - 0x3000); // ?
        return palette.read(addr - 0x3F00);
      }
      let addr = self.calc_addr(addr);
      self.buf = vram.read(addr);
    } else {
      self.buf = cram.read(addr);
    }
    buf // late 1 cycle
  }

  pub fn write<P: PaletteRam>(&mut self, vram: &mut Ram, cram: &mut Ram, addr: Addr, data: Data, palette: &mut P){
    if addr >= 0x2000 {
      if addr >= 0x3f00 && addr < 0x4000 { // palette
        palette.write(addr - 0x3f00, data);
      } else { // vram
        let addr = self.calc_addr(addr);
        vram.write(addr, data);
      }
    } else { // cram
      cram.write(addr, data);
    }
  }

  fn calc_addr(&self, addr: Addr) -> Addr {
    if addr >= 0x3000 && addr < 0x3F00 {
       addr - 0x3000
    } else {
      addr - 0x2000
    }
  }
}

#[test]
fn test_ppu_data() {
  let mut d = PpuData::new();
  let mut cram = Ram::new(vec!(0;20000));
  let mut vram = Ram::new(vec!(0;20000));
  let mut p = Palette::new();
  d.write(&mut vram, &mut cram, 0x1100, 0xFF, &mut p);
  d.read(&vram, &cram, 0x1100,  &p);
  assert_eq!(d.read(&vram, &cram, 0x1100,  &p), 0xFF);

  d.write(&mut vram, &mut cram, 0x2100, 0xFF, &mut p);
  d.read(&vram, &cram, 0x2100,  &p);
  assert_eq!(d.read(&vram, &cram, 0x2100,  &p), 0xFF);

  d.write(&mut vram, &mut cram, 0x3100, 0xFF, &mut p);
  d.read(&vram, &cram, 0x3100,  &p);
  assert_eq!(d.read(&vram, &cram, 0x3100,  &p), 0xFF);

  d.write(&mut vram, &mut cram, 0x3F00, 0xFF, &mut p);
  d.read(&vram, &cram, 0x3F00,  &p);
  assert_eq!(d.read(&vram, &cram, 0x3F00,  &p), 0xFF);
}