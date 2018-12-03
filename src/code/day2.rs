use code::helper::file_to_string;
// use code::helper::file_to_vec;

use std::collections::HashMap;

pub fn problem1(path: &str) -> i32 {
  let s = file_to_string(path).unwrap();

  let mut twos = 0;
  let mut threes = 0;

  for line in s.lines() {
    let mut c: HashMap<char, i32> = HashMap::new();    
    for ch in line.chars() {
      *c.entry(ch).or_insert(0) += 1; 
    }

    let mut found2 = false;
    let mut found3 = false;
    for v in c.values() {
      if !found2 && *v == 2 {
        twos += 1;
        found2 = true;
      } else if !found3 && *v == 3 {
        threes += 1;
        found3 = true;
      }

      if found2 && found3 {
        break;
      }
    }
  }

  twos * threes
}

pub fn problem2(path: &str) -> (&str, &str) {
  /*
  let v = file_to_vec(path);

  for &line1 in v.iter() {
    for &line2 in v.iter() {
      if line1 != line2 {
        let mut diff = 0;
        for (ch1, ch2) in line1.chars().zip(line2.chars()) {
          if ch1 != ch2 {
            diff += 1; 
          }
        }
        if diff == 1 {
          return (&line1, &line2);
        }
      }
    }
  }
  */
  (&"", &"")
}
