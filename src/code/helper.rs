use std::io;
use std::io::Read;
use std::fs::File;

pub fn file_to_string(path: &str) -> Result<String, io::Error> {
  let mut s = String::new();
  File::open(path)?.read_to_string(&mut s)?;
  Ok(s)
}
