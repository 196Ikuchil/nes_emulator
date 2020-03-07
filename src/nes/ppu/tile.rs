use self::super::palette::*;
use self::super::sprite_utils::*;
use self::super::Ram;

#[derive(Debug)]
pub struct Tile {
  pub sprite: Sprite,
  pub palette: PaletteList,
}

impl Tile {
  pub fn new<P: PaletteRam>(
    vram: &Ram,
    cram: &Ram,
    palette: &P,
    pos: &SpritePosition,
    config: &SpriteConfig
  ) -> Self {
    let block_id = get_block_id(pos);
    let tile_id = get_tile_id(&vram, pos, config);
    let attr = get_attribute(&vram, pos, config);
    let palette_id = (attr >> (block_id * 2)) & 0x03;
    let sprite = build(&cram, tile_id, config.offset_addr_by_background_table, true);
    Tile {
      sprite,
      palette: palette.get(palette_id, PaletteType::Background),
    }
  }
}
