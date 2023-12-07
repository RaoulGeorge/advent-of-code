use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn calc() -> usize {
    let input: Vec<String> = read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    input.iter()
        .map(|line| {
            line.replace("A", "Z")
                .replace("K", "Y")
                .replace("Q", "X")
                .replace("J", "W")
                .replace("T", "V")
        })
        .map(|line| (line[0..5].to_string(), line[6..].parse::<usize>().unwrap()))
        .map(|(hand, bid)| {
            (
                hand.chars()
                    .map(|c| (hand.chars().filter(|c1| c == *c1).count(), c))
                    .max()
                    .unwrap(),
                hand,
                bid,
            )
        })
        .map(|(max1, hand, bid)| {
            (
                (max1.0 * 10
                    + hand
                        .chars()
                        .map(|c| {
                            hand.chars()
                                .filter(|c1| ((c == *c1) && (c != max1.1)))
                                .count()
                        })
                        .max()
                        .unwrap()),
                hand,
                bid,
            )
        })
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
        .iter()
        .enumerate()
        .map(|(i, (_score, _hand, bid))| (i + 1) * bid)
        .sum()
}

pub fn run() {
    println!("Result: {}", calc())
}