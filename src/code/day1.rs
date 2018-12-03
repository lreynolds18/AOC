use code::helper::file_to_veci32;

use std::str::FromStr;
use std::collections::HashMap;

// TODO: proposal 
//   - error handling over unwrap?
//   - error handler helper func to remove unwrap in file_to_*
//   - error handler in func here to remove unwrap (use match w/ panic)

pub fn problem1(path: &str) -> i32 {
  let v = file_to_veci32(path).unwrap();
  v.iter().sum()
}

pub fn problem2(path: &str) -> i32 {
  let v = file_to_veci32(path).unwrap();

  let mut t = 0;
  let mut d = HashMap::new();
  
  // while true / loop. poss to remove? 
  loop {
    for l in v.iter() {
      t += l;
      if d.contains_key(&t) {
        return t;
      }
      d.insert(t, 0);
    } 
  }
}
