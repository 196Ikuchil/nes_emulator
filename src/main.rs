#[cfg_attr(test, macro_use)]
extern crate lazy_static;

mod nes;
mod externs;

use nes::Context;
use std::string::String;

fn main() {
}

#[no_mangle]
pub fn run(len: usize, ptr: *mut u8, sram: *mut u8) {
  let buf: &mut [u8] = unsafe{ std::slice::from_raw_parts_mut(ptr, len) };
  let s: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(sram, 0x2000)};
  let mut ctx = Context::new(buf, s);
  nes::reset(&mut ctx);
  externs::cancel_main_loop();
  let main_loop = || {
    let key_state = buf[len -1];
    let debug_input = buf[len -2];
    nes::run(&mut ctx, key_state, debug_input);
  };
  externs::set_main_loop_callback(main_loop);
}
