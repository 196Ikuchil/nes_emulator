use std::str;
use super::types::{Data};

const NES_HEADER_SIZE: usize = 0x0010;
const PROGRAM_ROM_SIZE: usize = 0x4000;
const CHARACTER_ROM_SIZE: usize = 0x2000;

#[derive(Debug)]
pub struct Cassette {
  pub is_horizontal_mirror: bool,
  pub character_ram: Vec<Data>,
  pub program_rom: Vec<Data>,
  pub mapper: Data,
}

pub fn parse(buf: &mut [Data]) -> Cassette {
  let name = buf[0..3].to_vec();
  let ines = str::from_utf8(&name).unwrap();
  if ines != "NES" {
    panic!("Invalid *.nes file.")
  };
  let program_rom_pages = buf[4] as usize;
  println!("program rom size is {}", program_rom_pages);
  let character_rom_pages = buf[5] as usize;
  println!("character rom size is {}", character_rom_pages);
  let is_horizontal_mirror = (buf[6] & 0x01) != 0x01;
  let mapper = ((buf[6] & 0xF0) >> 4) | buf[7] & 0xF0;
  println!("mapper type is {}", mapper);
  let character_rom_start = NES_HEADER_SIZE + program_rom_pages * PROGRAM_ROM_SIZE;
  let character_rom_end = character_rom_start + character_rom_pages * CHARACTER_ROM_SIZE;
  let c_ram = if character_rom_start != character_rom_end {
    buf[character_rom_start..character_rom_end].to_vec()
  } else {
    vec!(0;0x2000)
  };
  Cassette {
    is_horizontal_mirror,
    program_rom: buf[NES_HEADER_SIZE..character_rom_start].to_vec(),
    character_ram: c_ram,
    mapper,
  }
}