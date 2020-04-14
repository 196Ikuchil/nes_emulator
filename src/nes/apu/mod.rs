mod square;
mod triangle;
mod noise;
mod dmc;
mod constants;
mod filter;

use self::constants::*;
use self::square::Square;
use self::triangle::Triangle;
use self::noise::Noise;
use self::dmc::DMC;
use self::filter::Filter;
use super::types::{Data, Addr};
use super::mapper::Mapper;
use super::Rom;
use super::Ram;
use super::CpuRegister;


#[derive(Debug)]
pub struct Apu {
  squares: (Square, Square),
  triangle: Triangle,
  noise: Noise,
  dmc: DMC,
  cycle: u16,
  step: usize,
  sequencer_mode: bool, // t => mode 1, f => mode 0
  enable_irq: bool,
  filter_chain: [Filter;3],
  pulse_table: Vec<f32>,
  tnd_table: Vec<f32>,
}

extern "C" {
  fn audio_output(value: f32);
}

impl Apu {
  pub fn new() -> Self {
    let mut pt = Vec::new();
    let mut tt = Vec::new();
    for i in 0..31 {
      pt.push( 95.52 / (8128.0 / i as f32 + 100.0));
    }
    for i in 0..203 {
      tt.push(163.67 / (24329.0 / i as f32 + 100.0));
    }
    Apu {
      squares: (Square::new(0), Square::new(1)),
      triangle: Triangle::new(2),
      noise: Noise::new(),
      dmc: DMC::new(3),
      cycle: 0,
      step: 0,
      sequencer_mode: false,
      enable_irq: false,
      filter_chain: [
        Filter::new_as_high_pass_filter(90 as f32),
        Filter::new_as_high_pass_filter(440 as f32),
        Filter::new_as_low_pass_filter(14000 as f32),
      ],
      pulse_table: pt.clone(),
      tnd_table: tt.clone(),
    }
  }

  // step
  pub fn run<T: CpuRegister>(&mut self, cycle: u16,register: &mut T, mapper: &mut dyn Mapper, sram: &Ram, prg_rom: &Rom, stall: &mut u8) {
    for _ in 0..cycle {
      let cycle1 = self.cycle;
      self.cycle += 1;
      let cycle2 = self.cycle;
      self.step_timers(mapper, sram, prg_rom, stall);

      let f1 = (cycle1 as f64 / DIVIDE_COUNT_FOR_240HZ as f64) as u16;
      let f2 = (cycle2 as f64 / DIVIDE_COUNT_FOR_240HZ as f64) as u16;
      if f1 != f2 {
        self.step_frame_counter(register);
      }
      let s1 = (cycle1 as f64 / (APU_SAMPLE_RATE as f64)) as u16;
      let s2 = (cycle2 as f64 / (APU_SAMPLE_RATE as f64)) as u16;
      if s1 != s2 {
        self.send_sample();
      }
    }
  }

  fn step_frame_counter<T: CpuRegister>(&mut self, register: &mut T) {
    if !self.sequencer_mode { // step4
      self.step = (self.step + 1) % 4;
      match self.step {
        0 | 2 => {
          self.step_envelope();
        }
        1 => {
          self.step_envelope();
          self.step_sweep();
          self.step_length();
        }
        3 => {
          self.step_envelope();
          self.step_sweep();
          self.step_length();
          if self.enable_irq {
            register.set_interrupt_irq();
          }
        }
        _ => panic!("step error step{:?}", self.step),
      }
    } else { // step5
      self.step = (self.step + 1) % 5;
      match self.step {
        0 | 2 => {
          self.step_envelope();
        }
        1 | 3 => {
          self.step_envelope();
          self.step_sweep();
          self.step_length();
        }
        4 => {}
        _ => panic!("step error")
      }
    }
  }

  fn send_sample(&mut self) {
    let output = self.step_filter_chain(self.output());
    unsafe {
      audio_output(output);
    }
  }

  fn output(&self) -> f32 {
    let p1 = self.squares.0.output() as f32;
    let p2 = self.squares.1.output() as f32;
    let t = self.triangle.output() as f32;
    let n = 0.0;//self.noise.output();
    let d = 0.0;//self.dmc.output();
    self.pulse_table[(p1 + p2) as usize] + self.tnd_table[((3.0 * t) + (2.0 * n + d)) as usize]
  }

  fn step_envelope(&mut self) {
    self.squares.0.step_envelope();
    self.squares.1.step_envelope();
    self.triangle.step_counter();
    // self.noise.stepEnvelope();
  }

  fn step_sweep(&mut self) {
    self.squares.0.step_sweep();
    self.squares.1.step_sweep();
  }

  fn step_length(&mut self) {
    self.squares.0.step_length();
    self.squares.1.step_length();
    self.triangle.step_length();
    // self.noise.stepLength()
  }

  fn step_timers(&mut self, mapper: &mut dyn Mapper, sram: &Ram, prg_rom: &Rom, stall: &mut u8) {
    if self.cycle%2 == 0 {
      self.squares.0.step_timer();
      self.squares.1.step_timer();
      // self.noise.step_timer();
      // self.dmc.step_timer(mapper, sram, prg_rom, stall);
    }
    self.triangle.step_timer();
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
        let t = if self.triangle.has_count_end() {
          0x00
        } else {
          0x04
        };
        let n = if self.noise.has_count_end() {
          0x00
        } else {
          0x08
        };
        let d = if self.dmc.has_count_end() {
          0x00
        } else {
          0x10
        };
        d | n | t | s1 | s0
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
        self.triangle.write(addr - 0x08, data);
      }
      0x0c..=0x0f => {
        self.noise.write(addr - 0x0c, data);
      }
      0x10..=0x13 => {
        self.dmc.write(addr - 0x10, data);
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
        if data & 0x04 == 0x04 {
          self.triangle.enable();
        } else {
          self.triangle.disable();
        }
        if data & 0x08 == 0x08 {
          self.noise.enable();
        } else {
          self.noise.disable();
          // self.noise.length_value = 0;
        }
        if data & 0x10 == 0x10 {
          self.dmc.enable();
          // if self.dmc.currentLength == 0 {
            // self.dmc.restart();
        } else {
          self.dmc.disable();
          // self.dmc.length_value = 0;
        }
      }
      0x17 => {
        self.sequencer_mode = data & 0x80 == 0x80;
        self.enable_irq = data & 0x40 != 0x40; // actually it keeps wherer irq is disabled
        if self.sequencer_mode { // step5
          self.step_envelope();
          self.step_sweep();
          self.step_length();
        }
      }
      _ => (),
    }
  }

  pub fn step_filter_chain(&mut self, x: f32) -> f32 {
    let mut v = x;
    for f in &mut self.filter_chain {
      v = f.Step(v);
    }
    v
  }
 }