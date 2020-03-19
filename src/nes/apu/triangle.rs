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
  length_counter: usize,
  counter_reload: bool,

  linear_counter: usize,
  frequency: usize,
  enabled: bool,
  playing: bool,
}

extern "C" {
  fn start_oscillator(index: usize);
  fn stop_oscillator(index: usize);
  fn set_oscillator_frequency(index: usize, freq: usize);
  fn change_oscillator_frequency(index: usize, freq: usize);
  fn set_oscillator_volume(index: usize, volume: f32);
}

impl Triangle {
  pub fn new(index: usize) -> Self {
    Triangle {
      index,
      is_length_enabled: false,
      counter_period: 0,
      timer_period: 0,
      length_counter: 0,
      counter_reload: false,

      linear_counter: 0,
      frequency: 0,
      enabled: false,
      playing: false,
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
        self.update_frequency();
        self.change_frequency();
      }
      0x03 => {
        self.timer_period &= 0xFF;
        self.timer_period |= (data as usize & 0x7) << 8;
        self.length_counter = COUNTER_TABLE[(data & 0xF8) as usize >> 3] as usize / 2;
        self.update_frequency();
        if self.enabled {
          self.start();
        }
        self.counter_reload = true
      }
      _ => (),
    }
  }
  // length coutner
  pub fn update_counter(&mut self) {
    self.step_length();
    self.step_linear_counter();
    if self.length_counter == 0  || self.linear_counter == 0 {
      self.stop();
    }
  }

  fn step_length(&mut self) {
    if self.is_length_enabled && self.length_counter > 0 {
      self.length_counter -= 1;
    }
  }

  fn step_linear_counter(&mut self) {
    if self.counter_reload {
      self.linear_counter = self.counter_period;
    } else if self.linear_counter > 0 {
      self.linear_counter -= 1;
    }

    if self.is_length_enabled {
      self.counter_reload = false;
    }
  }

  pub fn has_count_end(&self) -> bool {
    self.length_counter == 0
  }

  fn update_frequency(&mut self) {
    self.frequency = CPU_CLOCK / ((self.timer_period + 1) * 32) as usize;
  }

  fn change_frequency(&self) {
    unsafe {
      change_oscillator_frequency(self.index, self.frequency);
    }
  }

  fn set_volume(&mut self){
    unsafe { set_oscillator_volume(self.index, self.get_volume()) }
  }

  // current volume is set manually
  // actually set automatically
  // TODO: therefore, call stop on update_counter()
  fn get_volume(&self) -> f32 {
    let vol = if !self.enabled || self.length_counter == 0 || self.linear_counter == 0 {
      0 as f32
    } else {
      32.0 / (16.0 / GROBAL_GAIN) as f32
    };
    vol
  }

  pub fn enable(&mut self) {
    self.enabled = true;
    self.start();
  }

  pub fn disable(&mut self) {
    self.enabled = false;
    self.stop();
  }

  pub fn start(&mut self) {
    if !self.playing {
      self.playing = true;
      unsafe {
        start_oscillator(self.index);
        set_oscillator_frequency(self.index, self.frequency);
      };
    } else {
      self.change_frequency();
    }
    self.set_volume();
  }

  pub fn stop(&mut self) {
    if self.playing {
      unsafe {
        stop_oscillator(self.index);
        set_oscillator_volume(self.index, 0.0);
      }
      self.playing = false
    }
  }
}