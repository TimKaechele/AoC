use std::fs;

fn main() {
  let file_contents = match fs::read_to_string("./input.txt") {
    Ok(val) => val,
    Err(_) => String::from(""),
  };

  let lines = file_contents.lines();
  let integers = lines.map(|val : &str| val.to_string().parse::<i32>().unwrap());

  let mut count = 0;
  {
    let mut prev : i32 = i32::MAX;

    for i in integers {
      if i > prev {
        count += 1
      }
      prev = i;
    }
  }
  println!("{}", count);
}
