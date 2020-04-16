mod sdl2;

pub use self::sdl2::*;
use std::fmt;


pub const BUFFER_CAPACITY: usize = 4096 * 2;


pub trait Audio {
	fn resume(&self);
	fn push(&mut self, value: f32);
	fn copy_sample_buffer(&mut self, sample_buffer: &mut [f32; BUFFER_CAPACITY]);
}

impl fmt::Debug for Audio {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "Hi")
	}
}
