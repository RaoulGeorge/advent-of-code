use std::env;
use std::fs::read_to_string;

struct Card {
  id: i32,
  winning_numbers: Vec<i32>,
  actual_numbers: Vec<i32>,
  score: i32,
}

fn parse_card(data: &str) -> Option<Card> {
  let parts: Vec<&str> = data.split(": ").collect();
  if parts.len() != 2 {
      return None;
  }

  let id_parts: Vec<&str> = parts[0].split_whitespace().filter(|s| !s.is_empty()).collect();
  if id_parts.len() != 2 {
    return None;
  }

  let id = id_parts[1].parse::<i32>().ok()?;

  let card_data = parts[1].split(" | ");

  let mut winning_numbers = Vec::new();
  let mut actual_numbers = Vec::new();
  for (index, data) in card_data.enumerate() {
    // let mut num = 0;

    for datum in data.split_whitespace() {
      let num = datum.parse::<i32>().ok()?;
      if index == 0 {
        winning_numbers.push(num);
      } else {
        actual_numbers.push(num);
      }
    }
    
  }

  Some(Card { 
    id: id, 
    winning_numbers: winning_numbers, 
    actual_numbers: actual_numbers,
    score: 0,
  })
}

fn score_card(card: &Card) -> i32 {
  let mut score: i32 = 0;
  for num in &card.winning_numbers {
    if card.actual_numbers.contains(num) {
      score = calculate_score(score);
    }
  }
  score
}

fn calculate_score(score: i32) -> i32 {
  match score {
    0 => 1,
    _ => score*2,
  }
}

pub fn run() {
  env::set_var("RUST_BACKTRACE", "1");

  let lines = read_to_string("./input.txt").unwrap();

  let mut cards = Vec::new();

  for l in lines.lines() {
    if let Some(l) = parse_card(&l) {
      cards.push(l);
    }
  }

  let mut res = 0;

  for card in &cards {
    let score = score_card(&card);
    println!("Card ID: {}, Score: {}", card.id, score);
    res += score;
  }

  println!("Total Score: {}", res);
  

  // for card in &cards {
  //   println!("Card ID: {}", card.id);
  //   for num in &card.winning_numbers {
  //     println!("Winning Number: {}", num);
  //   }
  //   for num in &card.actual_numbers {
  //     println!("Actual Number: {}", num);
  //   }
  // }
}