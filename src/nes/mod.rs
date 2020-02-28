mod bus;
mod cpu;
mod cpu_register;
mod types;
mod helper;

#[derive(Debug)]
pub struct Context {

}

pub fn run(){

}

pub fn debug_run(){
  let mut register = cpu_register::Register::new();
  cpu::run(&mut register);
}