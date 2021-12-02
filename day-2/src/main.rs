use std::fs;

enum Direction {
  UP(i32),
  DOWN(i32),
  FORWARD(i32),
}

fn read_lines(file_name : &str) -> Vec<Direction> {
  fs::read_to_string(file_name).unwrap().lines().map(|line| decode_string(line)).collect()
}

fn decode_string(input : &str) -> Direction {
  let parts : Vec<&str> = input.split(" ").collect();
  let change_value : i32 = parts[1].parse().unwrap();

  match parts[0] {
    "forward" => Direction::FORWARD(change_value),
    "down" => Direction::DOWN(change_value),
    "up" => Direction::UP(change_value),
    _ => Direction::UP(0)
  }
}


fn main() {
  let mut aim = 0;
  let mut depth = 0;
  let mut horizontal = 0;

  let stream : Vec<Direction> = read_lines("./input.txt");

  for direction in stream {
    match direction {
      Direction::UP(change) => aim = aim - change,
      Direction::DOWN(change) => aim = aim + change,
      Direction::FORWARD(change) => {
        horizontal = horizontal + change;
        depth = depth + (aim * change);
      }
    }
  }

  println!("depth: {} * horizontal: {} = {}", depth, horizontal, depth * horizontal);
}
