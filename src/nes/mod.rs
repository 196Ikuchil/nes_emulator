mod bus;
mod cassette_paser;
mod cpu;
mod cpu_register;
mod dma;
mod types;
mod helper;
mod ram;
mod rom;
mod ppu;

use self::bus::cpu_bus;
use self::ram::Ram;
use self::rom::Rom;
use self::ppu::*;
use self::dma::*;
use self::types::{Data, Addr, Word};

const DMA_CYCLES: u16 = 514;

#[derive(Debug)]
pub struct Context {
  work_ram: Ram,
  ppu: Ppu,
  program_rom: Rom,
  cpu_register: cpu_register::Register,
  dma: Dma,
  nmi: bool,
}

pub fn reset(ctx: &mut Context) {
  let mut cpu_bus = cpu_bus::Bus::new(
    &ctx.program_rom,
    &mut ctx.work_ram,
    &mut ctx.ppu,
    &mut ctx.dma,
  );
  cpu::reset(&mut ctx.cpu_register, &mut cpu_bus);
}

pub fn run(ctx: &mut Context){
  loop {
    let cycle: Word = if ctx.dma.is_should_run() {
      ctx.dma.run(&ctx.work_ram, &mut ctx.ppu);
      DMA_CYCLES
    } else {
      let mut cpu_bus = cpu_bus::Bus::new(
        &ctx.program_rom,
        &mut ctx.work_ram,
        &mut ctx.ppu,
        &mut ctx.dma,
      );
      cpu::run(&mut ctx.cpu_register, &mut cpu_bus, &mut ctx.nmi) as Word
    };

    let is_ready = ctx.ppu.run((cycle * 3) as usize, &mut ctx.nmi);
    if is_ready {
      if ctx.ppu.background.0.len() != 0 {
        // ctx.renderer.render(&ctx.ppu.background.0, &ctx.ppu.sprites);
      }
      break;
    }
  }
}

impl Context {
  pub fn new(buf: &mut [Data]) -> Self {
    let cassette = cassette_paser::parse(buf);
    Context {
      cpu_register: cpu_register::Register::new(),
      program_rom: Rom::new(cassette.program_rom),
      ppu: Ppu::new(
        cassette.character_ram,
        PpuConfig {
          is_horizontal_mirror: cassette.is_horizontal_mirror,
        },
      ),
      work_ram: Ram::new(vec![0;0x0800]),
      dma: Dma::new(),
      nmi: false,
    }
  }
}
