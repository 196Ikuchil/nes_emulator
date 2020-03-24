mod mapper0;
mod mapper;

pub use super::types::*;
pub use super::ram::Ram;
pub use super::rom::Rom;
pub use self::mapper::Mapper;
pub use self::mapper0::Mapper0;

impl dyn Mapper {
  pub fn new(mapper_num: Data) -> Box<dyn Mapper> {
    match mapper_num {
      0 => Box::new(Mapper0::new()),
      _ => Box::new(Mapper0::new()),
    }
  }
}
