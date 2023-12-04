use std::env;
use std::fs::read_to_string;

struct Round {
  green: i32,
  blue: i32,
  red: i32,
}

struct Game {
  id: i32,
  rounds: Vec<Round>,
}

fn parse_game(data: &str) -> Option<Game> {
  let parts: Vec<&str> = data.split(": ").collect();
  if parts.len() != 2 {
      return None;
  }

  let game_id = parts[0].replace("Game ", "").parse::<i32>().ok()?;
  let rounds_data = parts[1].split("; ");

  let mut rounds = Vec::new();
  for round_data in rounds_data {
    let mut green = 0;
    let mut blue = 0;
    let mut red = 0;

    for color_data in round_data.split(", ") {
      let color_parts: Vec<&str> = color_data.split_whitespace().collect();
      if color_parts.len() != 2 {
          return None;
      }

      let count = color_parts[0].parse::<i32>().ok()?;
      match color_parts[1] {
        "green" => green = count,
        "blue" => blue = count,
        "red" => red = count,
        _ => return None,
      }
    }

    rounds.push(Round { green, blue, red });
  }

  Some(Game { id: game_id, rounds })
}

fn calculate_score(games: &Vec<Game>) {

  let r = 12;
  let g = 13;
  let b = 14;

  let mut res = 0;

  'outer: for game in games {
    for round in &game.rounds {
      if(round.green > g || round.blue > b || round.red > r) {
        println!("Game ID: {}", game.id);
        println!("Green: {}, Blue: {}, Red: {}", round.green, round.blue, round.red);
        continue 'outer;
      }
    }
    res += game.id;
  }

  println!("res {}", res);
}

pub fn run() {
  env::set_var("RUST_BACKTRACE", "1");

  let lines = read_to_string("./input.txt").unwrap();


  let mut games = Vec::new();

  for l in lines.lines() {
    if let Some(l) = parse_game(&l) {
      games.push(l);
    }
  }

  for game in &games {
    println!("Game ID: {}", game.id);
    for round in &game.rounds {
      println!("Green: {}, Blue: {}, Red: {}", round.green, round.blue, round.red);
    }
  }

  calculate_score(&games);
}