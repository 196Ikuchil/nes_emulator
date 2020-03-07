mod register;
mod palette;
mod sprite_utils;
mod tile;

use self::register::*;
use self::palette::*;
use self::super::ram::Ram;
use super::types::{Addr, Data};

#[derive(Debug)]
pub struct PpuCtx<P: PaletteRam> {
  pub palette: P,
  pub vram: Box<Ram>,
  pub cram: Box<Ram>,
  pub oam_ram: Box<Ram>,
}

#[derive(Debug)]
pub struct Ppu {

}