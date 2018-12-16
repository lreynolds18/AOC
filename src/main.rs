pub mod code;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  // check valid resource_path & valid int
  let mut resource_path = args[1].to_owned();
  let problem_number = args[2].parse::<i32>().unwrap();

  match problem_number {
    1 => {
           resource_path.push_str("/day1.txt");
           println!("{}", code::day1::problem1(&resource_path));
           println!("{}\n", code::day1::problem2(&resource_path));
         },
    2 => {
           resource_path.push_str("/day2.txt");
           println!("{}", code::day2::problem1(&resource_path));
           println!("{}\n", code::day2::problem2(&resource_path));
         },
    3 => {
           resource_path.push_str("/day3.txt");
           println!("{}", code::day3::problem1(&resource_path));
           println!("{}", code::day3::problem2(&resource_path));
         },
    4 => {
           resource_path.push_str("/day4.txt");
           println!("{}", code::day4::problem1(&resource_path));
           println!("{}", code::day4::problem2(&resource_path));
         },
    _ => {
           println!("Error: not found");
         }
  };
}
