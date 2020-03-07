mod bus;
mod cpu;
mod cpu_register;
mod dma;
mod types;
mod helper;
mod ram;
mod ppu;

use self::bus::*;
use self::ram::Ram;

#[derive(Debug)]
pub struct Context {

}

pub fn run(){

}

pub fn debug_run(){
  let mut register = cpu_register::Register::new();
  let mut x = 0;
  let mut cpu_bus = bus::bus::Bus::new(&x);
  cpu::run(&mut register, &mut cpu_bus, &mut false);
}