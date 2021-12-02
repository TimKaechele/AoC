use std::fs;

fn main() {
  let file_contents = match fs::read_to_string("./input.txt") {
    Ok(val) => val,
    Err(_) => String::from(""),
  };

  let lines = file_contents.lines();
  let integers = lines.map(|val : &str| val.to_string().parse::<i32>().unwrap());

  let integer_vec : Vec<i32> = integers.collect();

  let folded_windows = integer_vec.windows(3).map(|window| window.iter().fold(0, |collector, value| collector + value ));

  let mut prev = i32::MAX;
  let mut counter = 0;

  for i in folded_windows {
    if i > prev {
      counter += 1
    }
    prev = i;
  }

  println!("{}", counter)
}
