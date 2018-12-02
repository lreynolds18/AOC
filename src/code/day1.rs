use code::helper::file_to_string;

use std::str::FromStr;
use std::collections::HashMap;

// TODO: proposal 
//   - error handler helper func to remove unwrap in file_to_string
//   - error handler in func here to remove unwrap (use match w/ panic)
//   - prob not a way to write like s.lines().iter().map(...).collect()
//     for loop ok
//   - while true / loop. poss to remove? 

pub fn problem1(path: &str) -> i32 {
  // something other than unwrap?
  let s = file_to_string(path).unwrap();

  // possible to use s.lines().iter().map(...).collect()
  // or something?
  let mut t = 0;
  for l in s.lines() {
    t += i32::from_str(l).unwrap();
  } 
  t
}

pub fn problem2(path: &str) -> i32 {
  // something other than unwrap?
  let s = file_to_string(path).unwrap();

  // possible to use s.lines().iter().map(...).collect()
  // or something?
  let mut t = 0;
  let mut d = HashMap::new();
  
  loop {
    for l in s.lines() {
      t += i32::from_str(l).unwrap();
      if d.contains_key(&t) {
        return t;
      }
      d.insert(t, 0);
    } 
  }
}
