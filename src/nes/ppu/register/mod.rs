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
use super::palette::*;
use super::PpuCtx;

#[derive(Debug)]
pub struct Register {
  pub ppu_ctrl1: Data,
  pub ppu_ctrl2: Data,
  pub ppu_status: Data,
  pub oam: Oam,
  pub ppu_addr: PpuAddr,
  pub ppu_data: PpuData,
  pub ppu_scroll: PpuScroll,
}

//from https://github.com/bokuweb/rustynes/blob/master/src/nes/ppu/registers/mod.rs
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
    fn read<P: PaletteRam>(&mut self, addr: Addr, ctx: &mut PpuCtx<P>) -> Data;
    fn write<P: PaletteRam>(&mut self, addr: Addr, ddata: Data, ctx: &mut PpuCtx<P>);
    fn clear_vblank(&mut self);
    fn clear_sprite_hit(&mut self);
    fn set_vblank(&mut self);
    fn set_sprite_hit(&mut self);
    fn is_sprite_enable(&self) -> bool;
    fn is_background_enable(&self) -> bool;
    fn is_irq_enable(&self) -> bool;
    fn is_sprite_8x8(&self) -> bool;
    fn get_ppu_addr_increment_value(&self) -> usize;
    fn get_background_table_offset(&self) -> Addr;
    fn get_name_table_id(&self) -> Data;
    fn get_sprite_table_offset(&self) -> Addr;
    fn get_scroll_x(&self) -> Data;
    fn get_scroll_y(&self) -> Data;
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

  fn read_status(&mut self) -> Data {
    let data = self.ppu_status;
    self.ppu_scroll.enable_x();
    self.clear_vblank();
    self.clear_sprite_hit();
    self.ppu_addr.reset_latch();
    data
  }

  fn write_oam_addr(&mut self, data: Data) {
    self.oam.write_addr(data)
  }

  fn write_oam_data(&mut self, data: Data, oam_ram: &mut Ram) {
    self.oam.write_data(oam_ram, data)
  }

  fn write_ppu_addr(&mut self, data: Data) {
    self.ppu_addr.write(data)
  }

  fn read_ppu_data<P: PaletteRam>(&mut self, vram: &Ram, cram: &Ram, palette: &P) -> Data {
    let addr = self.ppu_addr.get();
    let data = self.ppu_data.read(vram, cram,addr, palette);
    let v = self.get_ppu_addr_increment_value() as u16;
    self.ppu_addr.update(v);
    data
  }

  fn write_ppu_data<P: PaletteRam>(&mut self, data: Data, vram: &mut Ram, cram: &mut Ram, palette: &mut P){
    let addr = self.ppu_addr.get();
    self.ppu_data.write(vram, cram, addr ,data, palette);
    let v = self.get_ppu_addr_increment_value() as u16;
    self.ppu_addr.update(v);
  }
}

impl PpuRegister for Register {

  fn read<P: PaletteRam>(&mut self, addr: Addr, ctx: &mut PpuCtx<P>) -> Data{
    match addr {
      0x0002 => self.read_status(),
      0x0004 => self.oam.read_data(&ctx.oam_ram),
      0x0007 => self.read_ppu_data(&ctx.vram, &ctx.cram, &ctx.palette),
      _ => 0,
    }
  }

  fn write<P: PaletteRam>(&mut self, addr: Addr, data: Data, ctx: &mut PpuCtx<P>) {
    match addr {
      0x0000 => self.ppu_ctrl1 = data,
      0x0001 => self.ppu_ctrl2 = data,
      0x0003 => self.write_oam_addr(data),
      0x0004 => self.write_oam_data(data, &mut ctx.oam_ram),
      0x0005 => self.ppu_scroll.write(data),
      0x0006 => self.write_ppu_addr(data),
      0x0007 => self.write_ppu_data(data, &mut ctx.vram, &mut ctx.cram, &mut ctx.palette),
      _ => (),
    }
  }

  fn clear_vblank(&mut self) {
    self.ppu_status &= 0x7F
  }

  fn clear_sprite_hit(&mut self) {
    self.ppu_status &= 0x40
  }

  fn set_vblank(&mut self) {
    self.ppu_status |= 0x80;
  }

  fn set_sprite_hit(&mut self) {
    self.ppu_status |= 0x40
  }

  fn is_background_enable(&self) -> bool {
    self.ppu_ctrl2 & 0x08 == 0x08
  }

  fn is_sprite_enable(&self) -> bool {
    self.ppu_ctrl2 & 0x10 == 0x10
  }

  fn is_irq_enable(&self) -> bool {
    self.ppu_ctrl1 & 0x80 == 0x80
  }

  fn is_sprite_8x8(&self) -> bool {
    self.ppu_ctrl1 & 0x20 != 0x20
  }

  fn get_ppu_addr_increment_value(&self) -> usize {
    if self.ppu_ctrl1 & 0x04 == 0x04 {
      32
    } else {
      1
    }
  }

  fn get_background_table_offset(&self) -> Addr{
    if self.ppu_ctrl1 & 0x10 == 0x10 {
      0x1000
    } else {
      0x0000
    }
  }

  fn get_name_table_id(&self) -> Data {
    self.ppu_ctrl1 & 0x03
  }

  fn get_sprite_table_offset(&self) -> Addr {
    if self.ppu_ctrl1 & 0x08 == 0x08 {
      0x1000
    } else {
      0x0000
    }
  }

  fn get_scroll_x(&self) -> Data {
    self.ppu_scroll.get_x()
  }

  fn get_scroll_y(&self) -> Data {
    self.ppu_scroll.get_y()
  }
}