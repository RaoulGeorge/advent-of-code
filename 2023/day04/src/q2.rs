use std::env;
use std::fs::read_to_string;

struct Card {
  id: i32,
  winning_numbers: Vec<i32>,
  actual_numbers: Vec<i32>,
  instance_count: i32,
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
    instance_count: 1,
  })
}

fn get_num_matches(card: &Card) -> i32 {
  let mut score: i32 = 0;
  for num in &card.winning_numbers {
    if card.actual_numbers.contains(num) {
      score += 1;
    }
  }
  score
}

fn process_cards(cards: &mut Vec<Card>) {
  let mut i = 0;
  while i < cards.len() {
    let num_of_matches = get_num_matches(&cards[i]);
    println!("num_of_matches: {}", num_of_matches);
    let mut j = 1;

    while j <= num_of_matches as usize && i + j < cards.len() {
      cards[i + j].instance_count += cards[i].instance_count;
      j += 1;
    }
    i += 1;
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

  process_cards(&mut cards);

  let mut res = 0;
  for card in &cards {
    res += card.instance_count;
  }

  println!("Total Score: {}", res);
}