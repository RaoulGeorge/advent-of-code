use std::collections::HashMap;
use std::fs::read_to_string;

fn create_digit_map() -> HashMap<String, u32> {
  let mut map = HashMap::new();
  map.insert("one".to_string(), 1);
  map.insert("two".to_string(), 2);
  map.insert("three".to_string(), 3);
  map.insert("four".to_string(), 4);
  map.insert("five".to_string(), 5);
  map.insert("six".to_string(), 6);
  map.insert("seven".to_string(), 7);
  map.insert("eight".to_string(), 8);
  map.insert("nine".to_string(), 9);
  map
}

fn parse_line(line: &str, digit_map: &HashMap<String, u32>) -> Option<u32> {

  let mut first_digit: Option<u32> = None;
  let mut last_digit: Option<u32> = None;
  for (index, char) in line.chars().enumerate() {
    // numbers in words
    for (word, &digit) in digit_map {
      if line[index..].starts_with(word) {
        if first_digit.is_none() {
          first_digit = Some(digit);
        }
        last_digit = Some(digit);
        break;
      }
    }
    // actual numbers
    if char.is_digit(10) {
      let digit = char.to_digit(10).unwrap();
      if first_digit.is_none() {
        first_digit = Some(digit);
      }
      last_digit = Some(digit);
    }
  }

  match (first_digit, last_digit) {
    (Some(first), Some(last)) => Some(first * 10 + last),
    _ => None,
  }
}

fn main() {
  println!("Hello, world!");

  let mut input = Vec::new();
  let mut result = 0;

  let lines = read_to_string("input.txt").unwrap();

  let digit_map = create_digit_map();

  for l in lines.lines() {
    println!("original line {:?}", l);
    input.push(l.to_string());
    let mut line = parse_line(&l, &digit_map);
    println!("parsed line {:?}", line);

    if let Some(value) = line {
      result += value;
    }

  }
  println!("final result {:?}", result);

  let test = parse_line(&"twoone11", &digit_map);
  println!("test {:?}", test);

}
