use super::constants::*;

#[derive(Debug)]
pub struct Filter {
  b0: f32,
  b1: f32,
	a1: f32,
	prevX: f32,
	prevY: f32,
}

impl Filter {
  pub fn new_as_low_pass_filter(cutoff_freq: f32) -> Self {
    let c = BROWSER_SAMPLE_RATE as f32 / std::f32::consts::PI / cutoff_freq;
    let a0i = 1.0 / (1.0 + c);
    Filter {
      b0: a0i,
      b1: a0i,
      a1: ( 1.0 - c) * a0i,
      prevX: 0.0,
      prevY: 0.0,
    }
  }

  pub fn new_as_high_pass_filter(cutoff_freq: f32) -> Self {
    let c = BROWSER_SAMPLE_RATE as f32 / std::f32::consts::PI / cutoff_freq;
    let a0i = 1.0 / (1.0 + c);
    Filter {
      b0: c * a0i,
      b1: -c * a0i,
      a1: (1.0 - c) * a0i,
      prevX: 0.0,
      prevY: 0.0,
    }
  }

  pub fn Step(&mut self, x: f32) -> f32 {
    let y = self.b0 * x + self.b1 * self.prevX - self.a1 * self.prevY;
    self.prevY = y;
    self.prevX = x;
    y
  }
}