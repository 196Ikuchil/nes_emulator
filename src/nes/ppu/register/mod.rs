mod oam;
mod ppu_addr;
mod ppu_data;
mod ppu_scroll;

use self::oam::Oam;
use self::ppu_addr::PpuAddr;
use self::ppu_data::PpuData;
use self::ppu_scroll::PpuScroll;
use super::super::types::{Data, Addr};
use super::super::Ram;


pub struct Register {
  pub ppu_ctrl1: Data,
  pub ppu_ctrl2: Data,
  pub ppu_status: Data,
  pub oam: Oam,
  pub ppu_addr: PpuAddr,
  pub ppu_data: PpuData,
  pub ppu_scroll: PpuScroll,
}

// PPU power up state from https://github.com/bokuweb/rustynes/blob/master/src/nes/ppu/registers/mod.rs
  // see. https://wiki.nesdev.com/w/index.php/PPU_power_up_state
  //
  // Memory map
  /*
  | addr           |  description               |
  +----------------+----------------------------+
  | 0x0000-0x0FFF  |  Pattern table#0           |
  | 0x1000-0x1FFF  |  Pattern table#1           |
  | 0x2000-0x23BF  |  Name table                |
  | 0x23C0-0x23FF  |  Attribute table           |
  | 0x2400-0x27BF  |  Name table                |
  | 0x27C0-0x27FF  |  Attribute table           |
  | 0x2800-0x2BBF  |  Name table                |
  | 0x2BC0-0x2BFF  |  Attribute table           |
  | 0x2C00-0x2FBF  |  Name Table                |
  | 0x2FC0-0x2FFF  |  Attribute Table           |
  | 0x3000-0x3EFF  |  mirror of 0x2000-0x2EFF   |
  | 0x3F00-0x3F0F  |  background Palette        |
  | 0x3F10-0x3F1F  |  sprite Palette            |
  | 0x3F20-0x3FFF  |  mirror of 0x3F00-0x3F1F   |
  */
  pub trait PpuRegister {

  }

impl Register {
  pub fn new()-> Self {
    Register {
      ppu_ctrl1: 0,
      ppu_ctrl2: 0,
      ppu_status: 0,
      oam: Oam::new(),
      ppu_addr: PpuAddr::new(),
      ppu_data: PpuData::new(),
      ppu_scroll: PpuScroll::new(),
    }
  }
}