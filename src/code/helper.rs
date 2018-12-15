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

pub fn difference_between_two_strings(s1: &str, s2: &str) -> i32 {
  let mut diff = 0;
  for (ch1, ch2) in s1.chars().zip(s2.chars()) {
    if ch1 != ch2 {
      diff += 1; 
    }
  }
  diff
}

pub fn similar_chars(s1: &str, s2: &str) -> String {
  let mut out = "".to_string();
  for (ch1, ch2) in s1.chars().zip(s2.chars()) {
    if ch1 == ch2 {
      out.push_str(&ch1.to_string());
    }
  }
  out
}

pub fn extract_nums(v: Vec<String>) -> Vec<Vec<i32>> {
  let mut out: Vec<Vec<i32>> = vec!();
  for line in &v {
    let mut lineout: Vec<i32> = vec!();
    let split_by_whitespace: Vec<&str> = line.split_whitespace().collect();

    let mut id: i32 = split_by_whitespace[0]
      .get(1..).unwrap()
      .parse::<i32>().unwrap();
    lineout.push(id);

    let mut lhs: Vec<i32> = split_by_whitespace[2]
      .get(0..split_by_whitespace[2].len()-1).unwrap() // curse the :
      .split(",")
      .map(|s| s.parse::<i32>().unwrap())
      .collect();
    lineout.append(&mut lhs);

    let mut rhs: Vec<i32> = split_by_whitespace[3]
      .split("x")
      .map(|s| s.parse::<i32>().unwrap())
      .collect();
    lineout.append(&mut rhs);

    out.push(lineout);
  }

  out
}
