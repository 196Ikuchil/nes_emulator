use super::constants::*;
use super::super::types::{Data, Addr};
use super::Mapper;
use super::Ram;
use super::Rom;

#[derive(Debug)]
pub struct DMC {
  index: usize,
  is_irq_enabled: bool,
  is_loop: bool,
  tick_period: u16,
  volume: Data,
  sample_address: Addr,
  sample_length: Addr,

  tick_value: u16,
  current_address: Addr,
  current_length: Addr,
  bit_count: u8,
  shift_register: Data,
  is_enabled: bool,
  is_playing: bool,
}

// sham emulation
extern "C" {
  fn set_oscillator_frequency(index: usize, freq: usize);
  fn change_oscillator_frequency(index: usize, freq: usize);
  fn set_oscillator_volume(index: usize, volume: f32);
  fn start_oscillator(index: usize);
  fn stop_oscillator(index: usize);
}

impl DMC {
  pub fn new(index: usize) -> Self {
    DMC {
      index,
      is_irq_enabled: false,
      is_loop: false,
      tick_period: 0x0,
      volume: 0x0,
      sample_address: 0x0,
      sample_length: 0x0,
      tick_value: 0x0,
      current_address: 0x0,
      current_length: 0x0,
      bit_count: 0x0,
      shift_register: 0x0,
      is_enabled: false,
      is_playing: false,
    }
  }

  pub fn write(&mut self, addr: Addr, data: Data) {
    match addr {
      0x00 => { // 0x4010
        self.is_irq_enabled = data & 0x80 == 0x80;
        self.is_loop = data & 0x40 == 0x40;
        self.tick_period = DMC_NTSC_TABLE[(data & 0x0F) as usize];
        self.set_frequency();
      }
      0x01 => { // 0x4011
        self.volume = data & 0x7F;
      }
      0x02 => { // 0x4012
        // Sample address = %11AAAAAA.AA000000 = $C000 + (A * 64)
        self.sample_address = 0xC000 | ((data as Addr) << 6)
      }
      0x03 => { // 0x4013
        // Sample length = %LLLL.LLLL0001 = (L * 16) + 1 bytes
        self.sample_length = ((data as Addr) << 4) | 0x0001;
      }
      _ => ()
    }
  }

  pub fn enable(&mut self) {
    self.is_enabled = true;
    unsafe {
      start_oscillator(self.index);
    }
    self.is_playing = true;
  }

  pub fn disable(&mut self) {
    self.is_enabled = false;
    unsafe {
      stop_oscillator(self.index);
    }
    self.is_playing = false;
  }

  pub fn step_timer(&mut self, mapper: &mut dyn Mapper, sram: &Ram, prg_rom: &Rom, stall: &mut u8) {
    if !self.is_enabled {
      return
    }
    self.step_reader(mapper, sram, prg_rom, stall);
    if self.tick_value == 0 {
      self.tick_value = self.tick_period;
      self.step_shifter();
    } else {
      self.tick_value -= 1;
    }
  }

  fn set_frequency(&mut self) {
    let freq = CPU_CLOCK / self.tick_period as usize;
    if !self.is_playing {
      unsafe {
        set_oscillator_frequency(self.index, freq);
      }
      self.is_playing = true;
    } else {
      self.update_frequency(freq);
    }
  }

  fn update_frequency(&self, freq: usize) {
    unsafe {
      change_oscillator_frequency(self.index, freq);
    }
  }

  pub fn step_reader(&mut self, mapper: &mut dyn Mapper, sram: &Ram, prg_rom: &Rom, stall: &mut u8) {
    if self.current_length > 0 && self.bit_count == 0 {
      *stall += 4;
      self.shift_register = mapper.read(self.current_address - 0x8000,prg_rom, sram); // Does here access to only prg rom?
      self.bit_count = 8;
      self.current_address += 1;
      if self.current_address == 0 {
        self.current_address = 0x8000;
      }
      self.current_length -= 1;
      if self.current_length == 0 && self.is_loop {
        self.restart();
      }
    }
  }

  pub fn step_shifter(&mut self) {
    if self.bit_count == 0 {
      return
    }
    if self.shift_register & 0x1 == 0x1 {
      if self.volume <= 125 {
        self.volume += 2;
      }
    } else {
      if self.volume >= 2 {
        self.volume -= 2
      }
    }
    self.shift_register >>= 1;
    self.bit_count -= 1;
    self.set_volume();
  }

  fn restart(&mut self) {
    self.current_address = self.sample_address;
    self.current_length = self.sample_length;
  }


  fn set_volume(&self) {
    unsafe {
      set_oscillator_volume(self.index, self.get_volume());
    }
  }

  fn get_volume(&self) -> f32 {
    self.volume as f32 / (GROBAL_GAIN * 10 as f32)
  }

  pub fn has_count_end(&self) -> bool {
    self.current_length == 0
  }
 }