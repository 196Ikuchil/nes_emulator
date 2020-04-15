mod color;

use super::super::types::{Data};
use super::{BackgroundField, BackgroundCtx};
use super::{Sprite, SpritesWithCtx, SpritePosition};
use super::{PaletteList};
use self::color::COLORS;
extern "C" {
  fn canvas_render(ptr: *const Data, len: usize);
}

#[derive(Debug)]
pub struct Renderer {
  buf: Vec<Data>,
}

impl Renderer {
  pub fn new() -> Self {
    Renderer { buf: vec![0xFF;256*224*4]}
  }

  pub fn render(&mut self, background: &BackgroundField, sprites: &SpritesWithCtx, bg_clip: bool, sprite_clip: bool) {
    self.render_background(background, bg_clip);
    self.render_sprites(sprites,background, sprite_clip);
    unsafe {
      canvas_render(self.buf.as_ptr(), self.buf.len());
    }
  }

  fn render_background(&mut self, background: &BackgroundField, clip: bool) {
    for (i, bg) in background.into_iter().enumerate() {
      if bg.is_enabled {
        let x = (i % 33) * 8;
        let y = (i / 33) * 8;
        self.render_tile(bg, x, y, clip);
      }
    }
  }

  fn render_tile(&mut self, bg: &BackgroundCtx, x: usize, y: usize, clip: bool) {
    let offset_x = (bg.scroll_x % 8) as i32;
    let offset_y = (bg.scroll_y % 8) as i32;
    for i in 0..8 {
        for j in 0..8 {
            let x = (x + j) as i32 - offset_x;
            let y = (y + i) as i32 - offset_y;
            if x >= 0 as i32 && 0xFF >= x && y >= 0 as i32 && y < 224 {
                let color_id = bg.tile.palette[bg.tile.sprite[i][j] as usize];
                let color = COLORS[color_id as usize];
                let index = ((x + (y * 0x100)) * 4) as usize;
                self.buf[index] = color.0;
                self.buf[index + 1] = color.1;
                self.buf[index + 2] = color.2;
                if clip && x < 8 {
                    self.buf[index + 3] = 0;
                }
            }
        }
    }
}

  fn render_sprites(&mut self, sprites: &SpritesWithCtx, background: &BackgroundField, clip: bool) {
    for sprite in sprites {
      self.render_sprite(&sprite.sprite, &sprite.position,&sprite.palette ,sprite.attr, &background, clip);
    }
  }

  fn render_sprite(&mut self, sprite: &Sprite, position: &SpritePosition, palette: &PaletteList, attr: Data, background: &BackgroundField, clip: bool) {
    let is_vertical_reverse = (attr & 0x80) == 0x80;
    let is_horizontal_reverse = (attr & 0x40) == 0x40;
    let is_low_priority = (attr & 0x20) == 0x20;
    let h = sprite.len();
    for i in 0..h {
      let y = position.1 as usize + if is_vertical_reverse { h - 1 - i} else { i };
      if y >= 224 {
        continue
      }
      for j in 0..8 {
        let x = position.0 as usize + if is_horizontal_reverse { 7 - j } else { j };
        if x >= 256 {
          continue
        }
        if is_low_priority && self.should_pixel_hide(x, y, background) {
          continue
        }
        if sprite[i][j] != 0 {
          let color_id = palette[sprite[i][j] as usize];
          let color = COLORS[color_id as usize];
          let index = (x + (y * 0x100)) * 4;
          self.buf[index] = color.0;
          self.buf[index + 1] = color.1;
          self.buf[index + 2] = color.2;
          if clip && x < 8 {
            self.buf[index + 3] = 0;
          }
        }
      }
    }
  }

  fn should_pixel_hide(&self, x: usize, y: usize, background: &BackgroundField) -> bool {
    let tile_x = x / 8;
    let tile_y = y / 8;
    let background_index = tile_y * 33 + tile_x;
    let sprite = &background[background_index];
    // NOTE: If background pixel is not transparent, we need to hide sprite.
    sprite.tile.sprite[y % 8][x % 8] % 4 != 0
  }

  pub fn get_buf(&self) -> &Vec<u8> {
    &self.buf
  }
}