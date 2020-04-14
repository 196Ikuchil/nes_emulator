use super::constants::*;
use super::super::types::{Data, Addr};

#[derive(Debug)]
pub struct Square {
  index: usize,
  // $4000
  duty: usize,
  is_length_enabled: bool,
  is_envelope_loop_enabled: bool,
  is_envelope_enabled: bool,
  envelope_period: usize,
  constant_volume: usize,
  envelope_start: bool,
  // $4001
  is_sweep_enabled: bool,
  sweep_period: usize,
  is_sweep_negate: bool,
  sweep_shift_amount: usize,
  is_sweep_reload: bool,
  // $4003 &0x04 << 8 | $4002
  timer_period: usize,
  length_value: usize,
  duty_value: usize,

  timer_value: usize,
  envelope_volume: usize,
  envelope_value: usize,
  sweep_value: usize,
  enabled: bool,
}

impl Square {
  pub fn new(index: usize) -> Self {
    Square {
      index,
      duty: 0,
      is_length_enabled: false,
      is_envelope_loop_enabled: false,
      is_envelope_enabled: false,
      envelope_period: 0,
      constant_volume: 0,
      envelope_start: false,
      is_sweep_enabled: false,
      sweep_period: 0,
      is_sweep_negate: false,
      sweep_shift_amount: 0,
      is_sweep_reload: false,
      timer_period: 0,
      length_value: 0,
      duty_value: 0,

      timer_value: 0,
      envelope_volume: 0,
      envelope_value: 0,
      sweep_value: 0,
      enabled: false,

    }
  }

  pub fn write(&mut self, addr: Addr, data: Data) {
    match addr {
      0x00 => {
        self.duty = (data as usize & 0xC0) >> 6;
        self.is_length_enabled = data & 0x20 != 0x20;
        self.is_envelope_loop_enabled != self.is_length_enabled;
        self.is_envelope_enabled = data & 0x10 != 0x10;
        self.envelope_period = data as usize & 0x0F;
        self.constant_volume = data as usize & 0x0F;
        self.envelope_start = true;
      }
      0x01 => {
        self.is_sweep_enabled = data & 0x80 == 0x80;
        self.sweep_period = ((data as usize & 0x70) >> 4) + 1;
        self.is_sweep_negate = data & 0x08 == 0x08;
        self.sweep_shift_amount = data as usize & 0x07;
        self.is_sweep_reload = true;
      }
      0x02 => {
        self.timer_period = (self.timer_period & 0x700) | data as usize;
      }
      0x03 => {
        self.length_value = COUNTER_TABLE[(data & 0xF8) as usize >> 3] as usize;
        self.timer_period &= 0x00FF;
        self.timer_period |= (data as usize & 0x7) << 8;
        self.envelope_start = true;
        self.duty_value = 0;
      }
      _ => ()
    }
  }

  pub fn enable(&mut self) {
    self.enabled = true;
  }

  pub fn disable(&mut self) {
    self.enabled = false;
    self.length_value = 0;
  }

  pub fn has_count_end(&self) -> bool {
    if self.length_value == 0 {
      true
    } else {
      false
    }
  }

  pub fn step_timer(&mut self) {
    if self.timer_value == 0 {
      self.timer_value = self.timer_period;
      self.duty_value = (self.duty_value + 1) % 8;
    } else  {
      self.timer_value -= 1;
    }
  }

  pub fn step_envelope(&mut self) {
    if self.envelope_start {
      self.envelope_volume = 15;
      self.envelope_value = self.envelope_period;
      self.envelope_start = false;
    } else if self.envelope_value > 0 {
      self.envelope_value -= 1;
    } else {
      if self.envelope_volume > 0 {
        self.envelope_volume -= 1;
      } else if self.is_envelope_loop_enabled {
        self.envelope_volume = 15;
      }
      self.envelope_value = self.envelope_period;
    }
  }

  pub fn step_sweep(&mut self) {
    if self.is_sweep_reload {
      if self.is_sweep_enabled && self.sweep_value == 0 {
        self.sweep();
      }
      self.sweep_value = self.sweep_period;
      self.is_sweep_reload = false;
    } else if self.sweep_value > 0 {
      self.sweep_value -= 1;
    } else {
      if self.is_sweep_enabled {
        self.sweep();
      }
      self.sweep_value = self.sweep_period;
    }
  }

  pub fn sweep(&mut self) {
    let delta = self.timer_period >> self.sweep_shift_amount;
    if self.is_sweep_negate {
      self.timer_period -= delta;
      if self.index == 1 {
        self.timer_period -= 1;
      }
    } else {
      self.timer_period += delta;
    }
  }

  pub fn step_length(&mut self) {
    if self.is_length_enabled && self.length_value > 0 {
      self.length_value -= 1;
    }
  }


  pub fn output(&self) -> u8 {
    if !self.enabled
        || self.length_value == 0
        || DUTY_TABLE[(self.duty * 8 + self.duty_value) as usize] == 0
        || self.timer_period < 8
        || self.timer_period > 0x7FF
    {
      0
    } else if self.is_envelope_enabled {
      self.envelope_volume as u8
    } else {
      self.constant_volume as u8
    }
  }
}