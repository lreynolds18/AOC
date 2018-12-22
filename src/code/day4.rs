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

  let mut minutes_asleep: i32;
  let mut day_count: i32;
  let mut time_start: i32 = 0;
  let mut max_guard_id: i32 = -1;
  let mut max_minutes_asleep: f64 = -1.0;

  // get avg time sleeping
  for (k, v) in d.iter() {
    minutes_asleep = 0;
    day_count = 0;

    for line in v {
      if &line.description[..5] == "Guard" {
        day_count += 1;        
      } else if &line.description == "falls asleep" {
        time_start = line.minute;
      } else if &line.description == "wakes up" {
        minutes_asleep += if time_start > line.minute {
          60 - time_start + line.minute
        } else {
          line.minute - time_start
        }
      }
    }

    if minutes_asleep as f64 / day_count as f64 > max_minutes_asleep {
      max_guard_id = *k;
      max_minutes_asleep = minutes_asleep as f64 / day_count as f64;
    }
  }

  let mut c: HashMap<i32, i32> = HashMap::new();
  // get most frequent time a specific guard is sleeping
  for line in d.get(&max_guard_id).unwrap() {
    if &line.description == "falls asleep" {
      time_start = line.minute;
    } else if &line.description == "wakes up" {
      for i in time_start..line.minute {
        *c.entry(i).or_insert(0) += 1;
      }
    }
  }

  let mut max_minute = 0;
  let mut max_count = 0;
  for (k, v) in c.iter() {
    if v > &max_count {
      max_minute = *k;
      max_count = *v;
    }
  }

  max_guard_id * max_minute
}

pub fn problem2(path: &str) -> i32 {
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


  let mut max_guard_id: i32 = -1;
  let mut max_minute_asleep: i32 = -1;
  let mut max_minute_freq: i32 = -1;
  let mut time_start: i32 = 0;

  for (k, v) in d.iter() {
    let mut c: HashMap<i32, i32> = HashMap::new();
    for line in v {
      if &line.description == "falls asleep" {
        time_start = line.minute;
      } else if &line.description == "wakes up" {
        for i in time_start..line.minute {
          *c.entry(i).or_insert(0) += 1;
        }
      }
    }

    for (ck, cv) in c.iter() {
      if *cv > max_minute_freq {
        max_guard_id = *k;
        max_minute_asleep = *ck;
        max_minute_freq = *cv;
      }
    }
  }

  max_guard_id * max_minute_asleep
}
