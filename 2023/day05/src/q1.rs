use std::fs::read_to_string;

struct Mapping {
  dest_start: i64,
  src_start: i64,
  range: i64,
}

fn parse_mapping(data: &str) -> Vec<Mapping> {
  data.lines()
    .filter_map(|line| {
      let parts: Vec<i64> = line.split_whitespace()
                                .filter_map(|s| s.parse().ok())
                                .collect();
      if parts.len() == 3 {
        Some(Mapping {
          dest_start: parts[0],
          src_start: parts[1],
          range: parts[2],
        })
      } else {
        None
      }
    })
    .collect()
}

fn convert_number(number: i64, mappings: &[Mapping]) -> i64 {
  for mapping in mappings {
    if number >= mapping.src_start && number < mapping.src_start + mapping.range {
      return match mapping.dest_start.checked_add(number - mapping.src_start) {
        Some(n) => n,
        None => panic!("Overflow occurred during calculation"),
      };
    }
  }
  number
}

fn parse_seeds(data: &str) -> Vec<i64> {
  data.split_whitespace()
      .filter_map(|s| s.parse().ok())
      .collect()
}

pub fn run() {
  let input = read_to_string("./input.txt").unwrap();
  let sections: Vec<&str> = input.split("\n\n").collect();

  let initial_seeds = parse_seeds(sections[0].trim().strip_prefix("seeds: ").unwrap());

  let mut all_maps = Vec::new();
  for section in &sections[1..] {
    all_maps.push(parse_mapping(section));
  }

  let mut lowest_location = i64::MAX;

  for seed in initial_seeds {
    let mut current_number = seed;
    for map in &all_maps {
      current_number = convert_number(current_number, map);
    }
    lowest_location = lowest_location.min(current_number);
  }

  println!("Lowest location number: {}", lowest_location);
}

fn main() {
  run();
}
