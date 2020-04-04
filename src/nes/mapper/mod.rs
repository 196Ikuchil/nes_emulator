mod mapper0;
mod mapper3;
mod mapper4;
mod mapper;

pub use super::types::*;
pub use super::ram::Ram;
pub use super::rom::Rom;
pub use super::ppu::*;
pub use super::cpu_register::*;
pub use super::cassette_paser::Cassette;
pub use self::mapper::Mapper;
pub use self::mapper0::Mapper0;
pub use self::mapper3::Mapper3;
pub use self::mapper4::Mapper4;

impl dyn Mapper {
  pub fn new(cassette: &Cassette) -> Box<dyn Mapper> {
    match cassette.mapper {
      0 => Box::new(Mapper0::new()),
      3 => Box::new(Mapper3::new(cassette.program_rom.len() as u16)),
      4 => Box::new(Mapper4::new(cassette.program_rom.len(), cassette.character_ram.len())),
      _ => Box::new(Mapper0::new()),
    }
  }
}

