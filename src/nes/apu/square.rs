use super::constants::*;
use super::super::types::{Data, Addr};

#[derive(Debug)]
pub struct Square {
  index: usize,
  // $4000
  is_length_counter_enabled: bool,
  is_envelope_enabled: bool,
  is_envelope_loop_enabled: bool,
  envelope_period_and_volume: usize,
  // $4001
  is_sweep_enabled: bool,
  sweep_unit_divider: usize,
  is_sweep_direction_upper: bool,
  sweep_shift_amount: usize,
  // $4003 &0x04 << 8 | $4002
  divider_frequency: usize,
  length_counter: usize,

  frequency: usize,
  sweep_unit_counter: usize,
  envelope_generator_counter: usize,
  envelope_volume: usize,
  is_sweep_overflowed: bool,
  enabled: bool,
  playing: bool,
}

extern "C" {
  fn start_oscillator(index: usize);
  fn stop_oscillator(index: usize);
  fn set_oscillator_frequency(index: usize, freq: usize);
  fn change_oscillator_frequency(index: usize, freq: usize);
  fn set_oscillator_volume(index: usize, volume: f32);
  fn set_oscillator_duty(index: usize, duty: f32);

}

impl Square {
  pub fn new(index: usize) -> Self {
    Square {
      index,
      is_length_counter_enabled: false,
      is_envelope_enabled: false,
      is_envelope_loop_enabled: false,
      envelope_period_and_volume: 0x0F,
      is_sweep_enabled: false,
      sweep_unit_divider: 1,
      is_sweep_direction_upper: true,
      sweep_shift_amount: 0,
      divider_frequency: 1,
      length_counter: 0,

      frequency: 0,
      sweep_unit_counter: 0,
      envelope_generator_counter: 0,
      envelope_volume: 0,
      is_sweep_overflowed: false,
      enabled: false,
      playing: false,
    }
  }

  pub fn write(&mut self, addr: Addr, data: Data) {
    match addr {
      0x00 => {
        let duty = (data >> 6) & 0x3;
        self.is_envelope_loop_enabled = (data & 0x20) == 0x20;
        self.is_length_counter_enabled = !self.is_envelope_loop_enabled; // opposite loop flag
        self.is_envelope_enabled = (data & 0x10) != 0x10; //actually register keep loop is disabled on nes
        self.envelope_period_and_volume = data as usize & 0x0F;
        unsafe {
          set_oscillator_volume(self.index, self.get_volume());
          set_oscillator_duty(self.index, self.get_duty(duty as usize));
        }
      }
      0x01 => {
        self.is_sweep_enabled = data & 0x80 == 0x80;
        self.sweep_unit_divider = ((data as usize >> 4) & 0x07) + 1;
        self.is_sweep_direction_upper = data & 0x08 == 0x08;
        self.sweep_shift_amount = data as usize & 0x07;
      }
      0x02 => {
        self.divider_frequency = (self.divider_frequency & 0x700) | data as usize;
        self.is_sweep_overflowed = false;
        self.update_frequency();
        self.change_frequency();
      }
      0x03 => {
        self.divider_frequency &= 0xFF;
        self.divider_frequency |= (data as usize & 0x7) << 8;
        self.is_sweep_overflowed = false;
        if self.is_length_counter_enabled {
          self.length_counter = COUNTER_TABLE[(data & 0xF8) as usize >> 3] as usize / 2;
        }
        self.update_frequency();
        self.sweep_unit_counter = 0;
        // envelope
        self.envelope_generator_counter = self.envelope_period_and_volume;
        self.envelope_volume = 0x0F;
        if self.enabled {
          self.start();
        }
      }
      _ => ()
    }
  }

  fn change_frequency(&self){
    unsafe {
      change_oscillator_frequency(self.index, self.frequency);
    }
  }

  pub fn update_frequency(&mut self) {
    self.frequency = CPU_CLOCK / ((self.divider_frequency + 1) * 16) as usize;
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
  }

  pub fn stop(&mut self) {
    if self.playing {
      self.playing = false;
      unsafe {
        stop_oscillator(self.index);
      };
    }
  }

  pub fn has_count_end(&self) -> bool {
    self.length_counter == 0
  }

  pub fn update_counters(&mut self ) {
    if self.is_length_counter_enabled && self.length_counter > 0 {
      self.length_counter -= 1;
      if self.length_counter == 0 {
          self.stop();
      }
    }

    if !self.is_sweep_enabled || !self.playing {
      return;
    };

    self.sweep_unit_counter += 1;
    if self.sweep_unit_counter % self.sweep_unit_divider == 0 {
        self.sweep_unit_counter = 0;
        if self.is_sweep_direction_upper {
            self.divider_frequency = self.divider_frequency -
                                         (self.divider_frequency >>
                                          self.sweep_shift_amount);
        } else {
            self.divider_frequency = self.divider_frequency +
                                         (self.divider_frequency >>
                                          self.sweep_shift_amount);

        };
        if self.divider_frequency > 0x7FF || self.divider_frequency < 8 {
          self.is_sweep_overflowed = true;
          self.stop();
        }else {
          self.is_sweep_overflowed = false;
        }
        self.update_frequency();
        self.change_frequency();
    }
  }

  // divider Excitation
  pub fn update_envelope(&mut self) {
    self.envelope_generator_counter -= 1;
    if self.envelope_generator_counter <= 0 {
      self.envelope_generator_counter = self.envelope_period_and_volume;
      if self.envelope_volume > 0 {
        self.envelope_volume -= 1;
      } else {
        self.envelope_volume = if self.is_envelope_loop_enabled {
          0x0F
        } else {
          0x00
        };
      }
    }
    unsafe {
      set_oscillator_volume(self.index, self.get_volume());
    }
  }

  fn get_volume(&self) -> f32 {
    let vol = if !self.enabled || self.is_sweep_overflowed || self.length_counter == 0 { // || duty == 0
      0
    } else if self.is_envelope_enabled {
      self.envelope_volume
    } else {
      self.envelope_period_and_volume
    };
    vol as f32 / (GROBAL_GAIN)
  }

  pub fn get_duty(&self, duty: usize) -> f32 {
    match duty {
      0x00 => 0.125,
      0x01 => 0.25,
      0x02 => 0.5,
      0x03 => 0.75,
      _ => 0.0,
    }
  }
}