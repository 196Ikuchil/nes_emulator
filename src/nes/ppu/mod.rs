pub mod background;
pub mod tile;
mod register;
mod palette;
mod sprite;
mod sprite_utils;

use super::types::{Addr, Data};
use super::mapper::Mapper;
use self::super::ram::Ram;
use self::register::*;
pub use self::palette::*;
pub use self::sprite::*;
pub use self::sprite_utils::*;
pub use self::background::*;


const CYCLES_PER_LINE: usize = 341;

#[derive(Debug)]
pub struct PpuCtx<P: PaletteRam> {
  pub palette: P,
  pub vram: Box<Ram>,
  pub cram: Box<Ram>,
  pub oam_ram: Box<Ram>,
}

#[derive(Debug)]
pub struct PpuConfig {
  pub is_horizontal_mirror: bool,
}

#[derive(Debug)]
pub struct Ppu {
  pub cycle: usize,
  pub line: usize,
  pub register: Register,
  pub ctx: PpuCtx<Palette>,
  pub sprites: SpritesWithCtx,
  pub background: Background,
  pub config: PpuConfig,
}

impl Ppu {
  pub fn new(character_ram: Vec<Data>, config: PpuConfig) -> Ppu {
    Ppu {
      cycle: 0,
      line: 0,
      register: Register::new(),
      ctx: PpuCtx {
        palette: Palette::new(),
        vram: Box::new(Ram::new(vec![0;0x2000])),
        cram: Box::new(Ram::new(character_ram)),
        oam_ram: Box::new(Ram::new(vec![0;0x0100])),
      },
      sprites: Vec::new(),
      background: Background::new(),
      config,
    }
  }

  pub fn read(&mut self, addr: Addr, mapper: &dyn Mapper) -> Data {
    self.register.read(addr, &mut self.ctx, mapper)
  }

  pub fn write(&mut self, addr: Addr, data: Data, mapper: &mut dyn Mapper){
    self.register.write(addr, data, &mut self.ctx, mapper)
  }

  pub fn run(&mut self, cycle: usize, nmi: &mut bool, mapper: &dyn Mapper) -> bool {
    let cycle = self.cycle + cycle;
    if cycle < CYCLES_PER_LINE {
      self.cycle = cycle;
      return false;
    }

    if self.line == 0 {
      self.background.clear();
    }

    if self.has_sprite_hit(cycle) {
      self.register.set_sprite_hit();
    }

    self.cycle = cycle - CYCLES_PER_LINE;
    self.line = self.line + 1;

    let scroll_x = self.register.get_scroll_x();
    let scroll_y = self.register.get_scroll_y();
    if self.line <= 240 && self.line % 8 == 0 && scroll_y <= 240 {
      let mut config = SpriteConfig{
        offset_addr_by_name_table: None,
        offset_addr_by_background_table: self.register.get_background_table_offset(),
        offset_addr_by_sprite_table: self.register.get_sprite_table_offset(),
        is_horizontal_mirror: self.config.is_horizontal_mirror,
        is_background_enable: self.register.is_background_enable(),
      };
      // target line edge on display area
      let tile_x = ((scroll_x as usize + (self.register.get_name_table_id() % 2) as usize * 256) / 8) as Data;
      let tile_y = self.get_scroll_tile_y();
      self.background.build_line(
        &self.ctx.vram,
        &self.ctx.cram,
        &self.ctx.palette,
        (tile_x, tile_y),
        (scroll_x, scroll_y),
        &mut config,
        mapper
      );
    }

    // VBLANK
    if self.line == 241 {
      self.register.set_vblank();
      self.register.clear_sprite_hit();
      if self.register.is_irq_enable() {
        *nmi = true;
      }
    }

    // page end
    if self.line >= 262 {
      self.register.clear_vblank();
      self.register.clear_sprite_hit();
      *nmi = false;
      self.line = 0;
      self.sprites = build_sprites(
        &self.ctx.cram,
        &self.ctx.oam_ram,
        &self.ctx.palette,
        self.register.get_sprite_table_offset(),
        self.register.is_sprite_8x8(),
        mapper,
      );
      return true
    }

    false
  }

  pub fn transfer_sprite(&mut self, addr: Addr, data: Data) {
    let addr = addr + self.register.oam.get_addr();
    self.ctx.oam_ram.write(addr % 0x100, data);
  }

  fn get_scroll_tile_y(&self) -> Data {
    ((self.register.get_scroll_y() as usize
        + self.line
        + ((self.register.get_name_table_id() / 2) as usize * 240))
        / 8) as Data
  }

  pub fn is_background_enabled(&self) -> bool {
    self.register.is_background_enable()
  }

  pub fn is_sprite_enabled(&self) -> bool {
    self.register.is_sprite_enable()
  }

  // sprite 0 hit
  fn has_sprite_hit(&self, cycle: usize) -> bool {
    let y = self.ctx.oam_ram.read(0) as usize;
    let x = self.ctx.oam_ram.read(3) as usize;
    self.register.is_sprite_enable() && (y == self.line) && x <= cycle
  }
}