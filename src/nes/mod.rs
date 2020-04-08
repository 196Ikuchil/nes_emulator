mod apu;
mod bus;
mod cassette_paser;
mod cpu;
mod cpu_register;
mod dma;
mod types;
mod helper;
mod keypad;
mod ram;
mod rom;
mod ppu;
mod renderer;
mod mapper;

pub use self::apu::*;
pub use self::renderer::*;
pub use self::keypad::*;
use self::mapper::*;
use self::bus::cpu_bus;
use self::ram::Ram;
use self::rom::Rom;
use self::ppu::*;
use self::dma::*;
pub use self::types::{Data, Addr, Word};

const DMA_CYCLES: u16 = 514;

#[derive(Debug)]
pub struct Context {
  apu: Apu,
  work_ram: Ram,
  ppu: Ppu,
  program_rom: Rom,
  sram: Ram,
  cpu_register: cpu_register::Register,
  dma: Dma,
  nmi: bool,
  renderer: Renderer,
  keypad: Keypad,
  mapper: Box<dyn Mapper>,
}

pub fn reset(ctx: &mut Context) {
  let mut cpu_bus = cpu_bus::Bus::new(
    &mut ctx.apu,
    &ctx.program_rom,
    &mut ctx.work_ram,
    &mut ctx.sram,
    &mut ctx.ppu,
    &mut ctx.dma,
    &mut ctx.keypad,
    &mut *ctx.mapper,
  );
  cpu::reset(&mut ctx.cpu_register, &mut cpu_bus);
}

pub fn run(ctx: &mut Context, key_state: Data, debug_input: Data){
  ctx.keypad.update(key_state);

  // debug
  if debug_input & 0x01 == 0x01 {
    ctx.sram.save();
  }


  let mut stall: u8 = 0;
  loop {
    let cycle: Word = if ctx.dma.is_should_run() {
      ctx.dma.run(&ctx.work_ram, &mut ctx.ppu);
      DMA_CYCLES
    } else if stall > 0 {
      stall -= 1;
      1
    } else {
      let mut cpu_bus = cpu_bus::Bus::new(
        &mut ctx.apu,
        &ctx.program_rom,
        &mut ctx.work_ram,
        &mut ctx.sram,
        &mut ctx.ppu,
        &mut ctx.dma,
        &mut ctx.keypad,
        &mut *ctx.mapper,
      );
      cpu::run(&mut ctx.cpu_register, &mut cpu_bus, &mut ctx.nmi) as Word
    };
    // want to pass the cpu_bus
    ctx.apu.run(cycle, &mut ctx.cpu_register, &mut *ctx.mapper, &ctx.sram, &ctx.program_rom, &mut stall);
    let mut is_ready = false;
    for _ in 0..cycle*3 { // refactor: step for mapper
      is_ready |= ctx.ppu.run(1 as usize, &mut ctx.nmi, &*ctx.mapper);
      ctx.mapper.step(&ctx.ppu,&mut ctx.cpu_register);
    }

    if is_ready {
      if ctx.ppu.background.0.len() != 0 {
        ctx.renderer.render(&ctx.ppu.background.0, &ctx.ppu.sprites, ctx.ppu.register.ppu_ctrl2);
      }
      break;
    }
  }
}

impl Context {
  pub fn new(buf: &mut [Data], sram: &mut [Data]) -> Self {
    let cassette = cassette_paser::parse(buf);
    let mapper = Mapper::new(&cassette);
    Context {
      apu: Apu::new(),
      cpu_register: cpu_register::Register::new(),
      program_rom: Rom::new(cassette.program_rom),
      ppu: Ppu::new(
        cassette.character_ram,
        PpuConfig {
          is_horizontal_mirror: cassette.is_horizontal_mirror,
        },
      ),
      work_ram: Ram::new(vec![0;0x2000]),
      sram: Ram::new(sram.to_vec()),
      dma: Dma::new(),
      nmi: false,
      renderer: Renderer::new(),
      keypad: Keypad::new(),
      mapper: mapper,
    }
  }
}
