use std::fs;
use std::collections::HashMap;

const INPUT_LENGTH : usize = 12;

fn read_int_from_binary(bit_string : &str) -> Vec<u8> {
  bit_string.chars().map(|char| String::from(char).parse().unwrap()).collect()
}

fn read_integers(file_name: &str) -> Vec<Vec<u8>> {
  let contents = match fs::read_to_string(file_name) {
    Ok(val) => val,
    Err(_) => String::from("")
  };

  let lines = contents.lines();

  lines.map(|line| read_int_from_binary(line)).collect()
}

fn most_common_bit(input : &Vec<Vec<u8>>, position : usize) -> u8 {
  let mut map : HashMap<u8, usize> = HashMap::new();

  for int_buffer in input {
    let val = int_buffer[position];
    let current_count : usize = *map.get(&val).unwrap_or(&(0 as usize));
    map.insert(val, current_count + 1);
  }

  *map.iter().max_by(|a,b| a.1.cmp(&b.1)).unwrap().0
}

fn build_integer_from_bits(bits : &Vec<u8>) -> u32 {
  let mut int = 0 as u32;
  for (i, val) in bits.iter().enumerate() {
    let bit_position = bits.len() - 1 - i;
    let bit_to_push = (*val as u32) << bit_position;
    int =  bit_to_push | int;
  }
  int
}

fn invert_bits(bits : &Vec<u8>) -> Vec<u8> {
  let mut out = Vec::new();

  for bit in bits {
    if *bit == (0 as u8) {
      out.push(1)
    } else {
      out.push(0)
    }
  }

  out
}

fn first_problem() {
  let integers = read_integers("./input.txt");
  let mut most_common_bits : Vec<u8> = vec![];
  for i in 0..INPUT_LENGTH {
    most_common_bits.push(most_common_bit(&integers, i));
  }
  let gamma = build_integer_from_bits(&most_common_bits);
  let inverted_bits = invert_bits(&most_common_bits);
  let epsilon = build_integer_from_bits(&inverted_bits);

  println!("gamma: {} * epsilon: {} = {}", gamma, epsilon, gamma * epsilon);

}

fn remove_not_matching(integers : &Vec<Vec<u8>>, position : usize, val : u8) -> Vec<Vec<u8>> {
  let mut out = Vec::new();
  for int in integers {
    if int[position] == val {
      out.push(int.clone());
    }
  }
  out
}

fn second_problem() {
  let oxygen =
  {
    let mut integers = read_integers("./input.txt");
    let mut position = 0;
    while integers.len() > 1 {
      let mcb = most_common_bit(&integers, position);

      integers = remove_not_matching(&integers, position, mcb);
      position += 1;
    }

    build_integer_from_bits(&integers[0])
  };

  let co2 =
  {
    let mut integers = read_integers("./input.txt");
    let mut position = 0;
    while integers.len() > 1 {
      let mcb = most_common_bit(&integers, position);
      let inverted_mcb = if mcb == 0 {
        1
       } else {
         0
       };

      integers = remove_not_matching(&integers, position, inverted_mcb);
      position += 1;
    }

    build_integer_from_bits(&integers[0])
  };

  println!("o2: {} * co2: {} = {}", oxygen, co2, oxygen * co2)
}

fn main() {
  first_problem();
  second_problem();
}


