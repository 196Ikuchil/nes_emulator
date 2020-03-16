mod square;
mod constants;

use self::constants::*;
use self::square::Square;
use super::types::{Data, Addr};

#[derive(Debug)]
pub struct Apu {
  squares: (Square, Square),
  cycle: u16,
  step: usize,
  sequencer_mode: bool, // t => mode 1, f => mode 0
  enable_irq: bool,
}

impl Apu {
  pub fn new() -> Self {
    Apu {
      squares: (Square::new(0), Square::new(1)),
      cycle: 0,
      step: 0,
      sequencer_mode: false,
      enable_irq: false,
    }
  }
  pub fn run(&mut self, cycle: u16) {
    self.cycle += cycle;
    if self.cycle >= DIVIDE_COUNT_FOR_240HZ {
      // TODO: invoked by 240hz
      self.cycle -= DIVIDE_COUNT_FOR_240HZ;
      if self.sequencer_mode {
        self.update_by_sequence_mode1();
      } else {
        self.update_by_sequence_mdoe0();
      }
    }
  }

  // step 4
  fn update_by_sequence_mdoe0(&mut self) {
    if self.step % 2 == 1 {
      self.update_counters();
    }
    self.step += 1;
    if self.step == 4 {
      if self.enable_irq {
        // TODO:
      }
      self.step = 0;
    }
    self.update_envelope();
  }

  // step 5
  fn update_by_sequence_mode1(&mut self) {
    if self.step % 2 == 0 {
      self.update_counters();
    }
    self.step += 1;
    if self.step == 5 {
      self.step = 0;
    } else {
      self.update_envelope();
    }
  }

  // generate envelope & linear clock
  fn update_envelope(&mut self) {
    self.squares.0.update_envelope();
    self.squares.1.update_envelope();
  }

  // generate length counter & sweep ckock
  fn update_counters(&mut self) {
    self.squares.0.update_counters();
    self.squares.1.update_counters();
  }

  pub fn read(&mut self, addr: Addr) -> Data {
    match addr {
      0x15 => {
        let s0 = if self.squares.0.has_count_end() {
          0x00
      } else {
          0x01
      };
      let s1 = if self.squares.1.has_count_end() {
          0x00
      } else {
          0x02
      };
      s1 | s0
      }
    _ => 0,
    }
  }

  pub fn write(&mut self, addr: Addr, data: Data) {
    match addr {
      0x00..=0x03 => {
        self.squares.0.write(addr, data);
      }
      0x04..=0x07 => {
        self.squares.1.write(addr - 0x04, data);
      }
      0x08..=0x0b => {
      }
      0x0c..=0x0f => {
      }
      0x15 => {
        if data & 0x01 == 0x01 {
          self.squares.0.enable();
        } else {
          self.squares.0.disable();
        }
        if data & 0x02 == 0x02 {
          self.squares.1.enable();
        } else {
          self.squares.1.disable();
        }
      }
      0x17 => {
        self.sequencer_mode = data & 0x80 == 0x80;
        self.enable_irq = data & 0x40 != 0x40; // actually it keeps wherer irq is disabled
        self.step = 0;
        self.cycle = 0;
      }
      _ => (),
    }
  }
}