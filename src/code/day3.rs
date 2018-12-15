use code::helper::file_to_vecstring;
use code::helper::extract_nums;

use std::collections::HashMap;

pub fn problem1(path: &str) -> i32 {
  let v = file_to_vecstring(path).unwrap();
  let input = extract_nums(v); 

  let mut c: HashMap<(i32, i32), i32> = HashMap::new();

  for line in &input {
    for i in 0..line[3] {
      for j in 0..line[4] {
        *c.entry((line[1]+i, line[2]+j)).or_insert(0) += 1;
      }
    }
  }

  let mut out = 0;
  for v in c.values() {
    if *v > 1 {
      out += 1;
    }
  }

  out
}

pub fn problem2(path: &str) -> i32 {
  let v = file_to_vecstring(path).unwrap();
  let input = extract_nums(v); 

  let mut c: HashMap<(i32, i32), i32> = HashMap::new();

  for line in &input {
    for i in 0..line[3] {
      for j in 0..line[4] {
        *c.entry((line[1]+i, line[2]+j)).or_insert(0) += 1;
      }
    }
  }

  let mut id = -1;
  'outer: for line in &input {
    let mut unique = true;
    'inner: for i in 0..line[3] {
      for j in 0..line[4] {
        if c.get(&(line[1]+i, line[2]+j)) != Some(&1) {
          unique = false;
          break 'inner;
        }
      }
    }
    if unique == true {
      id = line[0];
      break 'outer;
    }
  }

  id
}
