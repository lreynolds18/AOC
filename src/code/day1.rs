use code::helper::file_to_veci32;

use std::str::FromStr;
use std::collections::HashMap;

pub fn problem1(path: &str) -> i32 {
  let v = file_to_veci32(path).unwrap();
  v.iter().sum()
}

pub fn problem2(path: &str) -> i32 {
  let v = file_to_veci32(path).unwrap();

  let mut t = 0;
  let mut d = HashMap::new();
  
  // TODO: while true / loop. poss to remove? 
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
