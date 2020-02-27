#[cfg_attr(test, macro_use)]
extern crate lazy_static;

mod nes;


fn main() {
    println!("Hello, world!");
    nes::debug_run();
}
