use super::super::types::{Data, Addr, Word};
use super::super::Ram;

pub type Sprite = Vec<Vec<Data>>;

pub type SpritePosition = (Data, Data);

#[derive(Debug)]
pub struct SpriteConfig {
  pub offset_addr_by_name_table: Option<Word>,
  pub offset_addr_by_background_table: Word,
  pub offset_addr_by_sprite_table: Word,
  pub is_horizontal_mirror: bool,
  pub is_background_enable: bool,
}

pub fn get_block_id(pos: &SpritePosition) -> Data { // for BG
  ((pos.0 % 4) / 2) + (((pos.1 % 4) / 2)* 2)
}

// get tile id from name table for BG
pub fn get_tile_id(vram: &Ram, pos: &SpritePosition, config: &SpriteConfig) -> Data {
  let tile_number = pos.1 as Addr * 32 + pos.0 as Addr;
  let addr = tile_number + config.offset_addr_by_name_table.unwrap();
  let addr = mirror_down_sprite_addr(addr, config.is_horizontal_mirror);
  vram.read(addr)
}

pub fn get_attribute(vram: &Ram, pos: &SpritePosition, config: &SpriteConfig) -> Data {
  let addr = config.offset_addr_by_name_table.unwrap() + 0x03C0 + ((pos.0 / 4) + ((pos.1 / 4) * 8)) as Addr;
  vram.read(mirror_down_sprite_addr(addr, config.is_horizontal_mirror))
}

pub fn mirror_down_sprite_addr(addr: Addr, is_horizontal_mirror: bool) -> Addr {
  if !is_horizontal_mirror {
    if 0x0800 <= addr && addr < 0x1000 { // vertical mirror
      return addr - 0x0800 as Addr
    }
    return addr;
  } else {
    if (0x0400 <= addr && addr < 0x0800) || addr >= 0x0C00 { // horizontal mirror
      return addr - 0x400 as Addr;
    }
     return addr
  }
}

pub fn build(cram: &Ram, tile_id: Data, offset: Addr, is_8x8: bool) -> Sprite {
  let h = if is_8x8 {1} else {2};
  let mut sprite: Sprite = (0..8 * h).into_iter().map(|_| vec![0; 8 * h]).collect();
  for k in 0..h {
    for i in 0..16 {
      for j in 0..8 {
        let addr = ((tile_id + (k as Data)) as Addr) * 16 + i + offset; // pattern table
        let data = cram.read(addr);
        if data & (0x80 >> j) as Data != 0 {
          sprite[((k as u16) * 8 + i % 8) as usize][j] += (0x01 << (i / 8)) as u8;
        }
      }
    }
  }
  sprite
}

#[test]
fn test_get_block_id() {
    let position = (2, 3);
    let id = get_block_id(&position);
    assert_eq!(id, 3);
}

#[test]
fn test_get_tile_id() {
  let mut v = Ram::new(vec!(0;10240));
  v.field[0x462] = 0xFF;
  let c = SpriteConfig {
    offset_addr_by_name_table: Some(0x400),
    offset_addr_by_background_table: 0,
    offset_addr_by_sprite_table: 0,
    is_horizontal_mirror: false,
    is_background_enable: true,
  };
  let pos = (2, 3);
  let id = get_tile_id(&mut v, &pos, &c);
  assert_eq!(id, 0xFF)
}

#[test]
fn test_get_attribute() {
  let mut v = Ram::new(vec!(0;10240));
  v.field[0x7C1] = 0xFF;
  let c = SpriteConfig {
    offset_addr_by_name_table: Some(0x400),
    offset_addr_by_background_table: 0,
    offset_addr_by_sprite_table: 0,
    is_horizontal_mirror: false,
    is_background_enable: true,
  };
  let pos = (4, 3);
  let attr = get_attribute(&mut v, &pos, &c);
  assert_eq!(attr, 0xFF)
}