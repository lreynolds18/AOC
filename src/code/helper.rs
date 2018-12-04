use std::io;
use std::io::Read;
use std::fs::File;
use std::str::FromStr;

pub fn file_to_string(path: &str) -> Result<String, io::Error> {
  let mut s = String::new();
  File::open(path)?.read_to_string(&mut s)?;
  Ok(s)
}

pub fn file_to_veci32(path: &str) -> Result<Vec<i32>, io::Error> {
  let mut s = String::new();
  File::open(path)?.read_to_string(&mut s)?;
  Ok(s.lines()
      .map(|line| i32::from_str(line).unwrap())
      .collect())
}

pub fn file_to_vecstring(path: &str) -> Result<Vec<String>, io::Error> {
  let mut s = String::new();
  File::open(path)?.read_to_string(&mut s)?;
  Ok(s.lines().map(|l| l.to_string()).collect())
}
