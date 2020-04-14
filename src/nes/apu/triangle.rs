use super::constants::*;
use super::super::types::{Data, Addr};

#[derive(Debug)]
pub struct Triangle {
  index: usize,
  // 0x4008
  is_length_enabled: bool,
  counter_period: usize,
  // 0x400A
  timer_period: usize, // divier for freqency
  // 0x400B
  length_value: usize,
  timer_value: usize,
  counter_reload: bool,

  duty: usize,
  counter_value: usize,
  enabled: bool,
}

impl Triangle {
  pub fn new(index: usize) -> Self {
    Triangle {
      index,
      is_length_enabled: false,
      counter_period: 0,
      timer_period: 0,
      length_value: 0,
      timer_value: 0,
      counter_reload: false,
      duty: 0,
      counter_value: 0,
      enabled: false,
    }
  }

  pub fn write(&mut self, addr: Addr, data: Data) {
    match addr {
      0x00 => {
        self.is_length_enabled = data & 0x80 != 0x80;
        self.counter_period = data as usize & 0x7F;
      }
      0x02 => {
        self.timer_period &= 0x700;
        self.timer_period |= data as usize;
      }
      0x03 => {
        self.timer_period &= 0x00FF;
        self.timer_period |= (data as usize & 0x7) << 8;
        self.length_value = COUNTER_TABLE[(data & 0xF8) as usize >> 3] as usize;
        self.timer_value = self.timer_period;
        self.counter_reload = true
      }
      _ => (),
    }
  }

  pub fn step_timer(&mut self) {
    if self.timer_value == 0 {
      self.timer_value = self.timer_period;
      if self.length_value > 0 && self.counter_value > 0 {
        self.duty = (self.duty + 1) % 32;
      }
    } else {
      self.timer_value -= 1;
    }
  }

  pub fn step_length(&mut self) {
    if self.is_length_enabled && self.length_value > 0 {
      self.length_value -= 1;
    }
  }

  pub fn step_counter(&mut self) {
    if self.counter_reload {
      self.counter_value = self.counter_period;
    } else if self.counter_value > 0 {
      self.counter_value -= 1;
    }
    if self.is_length_enabled {
      self.counter_reload = false;
    }
  }

  pub fn output(&self) -> u8 {
    if !self.enabled || self.length_value == 0 || self.counter_value == 0 {
      0
    } else {
      TRIANGLE_TABLE[self.duty]
    }
  }

  pub fn has_count_end(&self) -> bool {
    if self.length_value == 0 {
      true
    } else {
      false
    }
  }

  pub fn enable(&mut self) {
    self.enabled = true;
  }

  pub fn disable(&mut self) {
    self.enabled = false;
    self.length_value = 0;
  }
}