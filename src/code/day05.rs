use code::helper::file_to_string;
use std::usize::MAX;


pub fn problem1(path: &str) -> i32 {
  let mut v: Vec<char> = file_to_string(path).unwrap().chars().collect();
  let mut keep: Vec<char> = vec!('.');

  for ch in v.iter() {
    let lastch = keep[keep.len()-1];
    if *ch != lastch && ch.to_ascii_lowercase() == lastch.to_ascii_lowercase() {
      let l = keep.len() - 1;
      keep.remove(l);
    } else {
      keep.push(*ch);
    }
  } 
  
  keep.remove(0);

  (keep.len() as i32) - 1
}


pub fn problem2(path: &str) -> i32 {
  let mut v: Vec<char> = file_to_string(path).unwrap().chars().collect();
  let mut min_val: usize = MAX;

  for rch in "abcdefghijklmnopqrstuvwxyz".chars() {
    let mut keep: Vec<char> = vec!('.');
    for ch in v.iter() {
      if *ch != rch && *ch != rch.to_ascii_uppercase() {
        let lastch = keep[keep.len()-1];
        if *ch != lastch && ch.to_ascii_lowercase() == lastch.to_ascii_lowercase() {
          let l = keep.len() - 1;
          keep.remove(l);
        } else {
          keep.push(*ch);
        }
      }
    }

    keep.remove(0);

    if min_val > (keep.len() - 1) {
      min_val = keep.len() - 1;
    }
  }

  min_val as i32
}
