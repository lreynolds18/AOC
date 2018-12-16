use code::helper::file_to_vecstring;
use code::helper::extract_guard_input;
use code::helper::GuardInput;

use std::collections::HashMap;

pub fn problem1(path: &str) -> i32 {
  let v = file_to_vecstring(path).unwrap();
  let mut input = extract_guard_input(v); 

  input.sort();

  let mut d: HashMap<i32, Vec<GuardInput>> = HashMap::new();
  let mut id = 0;

  for mut line in input {
    if line.guard_id != 0 {
      id = line.guard_id;
    } else {
      line.guard_id = id;
    }

    d.entry(id).or_insert(Vec::new()).push(line);
  }

  for v in d.values() {
    for line in v {
      println!("{} {} {} {} {} {}", line.month, line.day, line.hour, line.minute, line.description, line.guard_id);
    }
  }

  // get avg time sleeping
  // get most frequent time a specific guard is sleeping
  0  
}

pub fn problem2(path: &str) -> i32 {
  0
}
