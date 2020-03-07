mod bus;
mod cpu;
mod cpu_register;
mod dma;
mod types;
mod helper;
mod ram;
mod ppu;

use self::bus::cpu_bus;
use self::ram::Ram;
use self::ppu::*;
use self::dma::*;
use self::types::{Data, Addr, Word};

const DMA_CYCLES: u16 = 514;

#[derive(Debug)]
pub struct Context {
  ppu: Ppu,
  work_ram: Ram,
  cpu_register: cpu_register::Register,
  dma: Dma,
  nmi: bool,
}

pub fn reset(ctx: &mut Context) {
  let mut cpu_bus = cpu_bus::Bus::new(
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

pub fn debug_run(){

}