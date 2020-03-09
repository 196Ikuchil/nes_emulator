#[cfg_attr(test, macro_use)]
extern crate lazy_static;

mod nes;
mod externs;

use nes::Context;
use std::string::String;

fn main() {
}

#[no_mangle]
pub fn run(len: usize, ptr: *mut u8) {
  let buf: &mut [u8] = unsafe{ std::slice::from_raw_parts_mut(ptr, len + 1) };
  let mut ctx = Context::new(buf);
  nes::reset(&mut ctx);
  externs::cancel_main_loop();
  let main_loop = || {
    // let key_state = buf[len -1];
    nes::run(&mut ctx);
  };
  externs::set_main_loop_callback(main_loop);
}
