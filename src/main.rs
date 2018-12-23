pub mod code;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  // check valid resource_path & valid int
  let mut resource_path = args[1].to_owned();
  let problem_number = args[2].parse::<i32>().unwrap();

  match problem_number {
    1 => {
           resource_path.push_str("/day01.txt");
           println!("{}", code::day01::problem1(&resource_path));
           println!("{}\n", code::day01::problem2(&resource_path));
         },
    2 => {
           resource_path.push_str("/day02.txt");
           println!("{}", code::day02::problem1(&resource_path));
           println!("{}\n", code::day02::problem2(&resource_path));
         },
    3 => {
           resource_path.push_str("/day03.txt");
           println!("{}", code::day03::problem1(&resource_path));
           println!("{}", code::day03::problem2(&resource_path));
         },
    4 => {
           resource_path.push_str("/day04.txt");
           println!("{}", code::day04::problem1(&resource_path));
           println!("{}", code::day04::problem2(&resource_path));
         },
    5 => {
           resource_path.push_str("/day05.txt");
           println!("{}", code::day05::problem1(&resource_path));
           println!("{}", code::day05::problem2(&resource_path));
         },
    _ => {
           println!("Error: not found");
         }
  };
}
