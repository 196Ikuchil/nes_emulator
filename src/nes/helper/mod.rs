use super::types::Data;
use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn bool2u8(b: bool) -> u8 {
  if b { 1 } else { 0 }
}

pub fn save_file(filename: String, buf: &Vec<Data>) {
  let path = &create_sram_file_path(filename);
  let mut file = BufWriter::new(File::create(path).unwrap());
  file.write_all(buf).unwrap();
}

pub fn load_or_init_file(filename: String) -> Vec<Data> {
  let path = &create_sram_file_path(filename);
  if Path::new(path).exists() {
    let mut file = BufReader::new(File::open(path).unwrap());
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    buf
  } else {
    vec!(0;0x2000)
  }
}

fn create_sram_file_path(filename: String) -> String {
  format!("{}{}{}", ".", filename, ".sram")
}
