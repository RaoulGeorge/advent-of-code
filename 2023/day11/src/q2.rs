use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(u64, u64);

fn calc(input: &str) -> u64 {
    let mut total: u64 = 0;
    let mut galaxies: Vec<Position> = vec![];
    let num_rows: u64 = input.lines().count() as u64;
    let num_cols: u64 = input.lines().next().unwrap().chars().count() as u64;
    let mut empty_rows: Vec<u64> = vec![];
    let mut empty_cols: Vec<u64> = vec![];
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Position(row as u64, col as u64));
            }
        }
    }
    for i in 0..num_rows {
        if !galaxies.iter().any(|p| p.0 == i) {
            empty_rows.push(i);
        }
    }
    for i in 0..num_cols {
        if !galaxies.iter().any(|p| p.1 == i) {
            empty_cols.push(i);
        }
    }
    for galaxy in galaxies.iter_mut() {
        let mut num_empty_rows = 0;
        for r in empty_rows.iter() {
            if galaxy.0 > *r {
                num_empty_rows += 999999;
            }
        }
        galaxy.0 += num_empty_rows;
        let mut num_empty_cols = 0;
        for c in empty_cols.iter() {
            if galaxy.1 > *c {
                num_empty_cols += 999999;
            }
        }
        galaxy.1 += num_empty_cols;
    }
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            total += (i64::abs(galaxies[i].0 as i64 - galaxies[j].0 as i64)
                + i64::abs(galaxies[i].1 as i64 - galaxies[j].1 as i64))
                as u64;
        }
    }
    total
}

pub fn run() {
    let input = fs::read_to_string("./input.txt").expect("read failed");
    let res = calc(&input);
    println!("{}", res);
}
