use self::super::palette::*;
use self::super::sprite_utils::*;
use self::super::Ram;
use super::super::types::{Data, Addr};

// 256 bytes
const OAM_RAM_CAPACITY: u16 = 0x100;

pub type SpritesWithCtx = Vec<SpriteWithCtx>;

#[derive(Debug)]
pub struct SpriteWithCtx {
  pub sprite: Sprite,
  pub position: SpritePosition,
  pub attr: Data,
  pub palette: PaletteList,
}

// make all registered from oam
pub fn build_sprites<P: PaletteRam>(cram: &Ram, oam_ram: &Ram, palette: &P, offset: Addr, is_8x8: bool) ->SpritesWithCtx {
  let mut buf: SpritesWithCtx = vec![];
  for i in 0..(OAM_RAM_CAPACITY / 4){
     // INFO: Offset sprite Y position, because First and last 8line is not rendered.
    let base = i * 4;
    let y = oam_ram.read(base);
    if 8 <= y && y < 244 {
      let sprite_id = oam_ram.read(base + 1);
      let attr = oam_ram.read(base + 2);
      let (offset, sprite_id) = if is_8x8 {
        (offset, sprite_id)
      } else {
        // 76543210
        // ||||||||
        // |||||||+- Bank ($0000 or $1000) of tiles
        // +++++++-- Tile number of top of sprite (0 to 254; bottom half gets the next tile)
        let offset = 0x1000u16 * (sprite_id & 0x01) as u16;
        let sprite_id = sprite_id & 0xFE;
        (offset, sprite_id)
      };
      let x = oam_ram.read(base + 3);
      let sprite = build(&cram, sprite_id as Data, offset, is_8x8);
      let position: SpritePosition = (x, y - 8);
      let palette_id = attr & 0x03;
      buf.push(SpriteWithCtx {
        sprite,
        position,
        attr,
        palette: palette.get(palette_id, PaletteType::Sprite),
      });
    }
  }
  buf
}