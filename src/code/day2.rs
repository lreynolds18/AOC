use code::helper::file_to_string;
use code::helper::file_to_vecstring;
use code::helper::difference_between_two_strings;
use code::helper::similar_chars;

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

pub fn problem2(path: &str) -> String {
  let v = file_to_vecstring(path).unwrap();
  let mut l1 = "";
  let mut l2 = "";

  'outer: for line1 in &v {
    for line2 in &v {
      if line1 != line2 {
        let mut diff = difference_between_two_strings(&line1, &line2);
        if diff == 1 {
          l1 = line1;
          l2 = line2;
          break 'outer;
        }
      }
    }
  }

  similar_chars(&l1, &l2)
}
