pub struct Bus<'a> {
  hoge: &'a i8
}

impl<'a> Bus<'a> {
  pub fn new(hoge: &'a i8) -> Bus<'a> {
    Self {
      hoge,
    }
  }
}