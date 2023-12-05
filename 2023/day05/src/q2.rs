use std::fs::read_to_string;
use std::vec;

#[derive(Clone)]
struct Range(i128, i128);

pub fn run() {
    let input = read_to_string("./input.txt").unwrap();
    let lines: Vec<_> = input.split("\n\n").collect();
    let seeds_ranges: Vec<Range> = lines
        .first()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()
        .iter()
        .last()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .map(|f| f.parse::<i128>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|f| Range(f[0], f[0] + f[1]))
        .collect::<Vec<_>>();

    let sections = lines
        .iter()
        .skip(1)
        .map(|section| {
            let mut a = section
                .split(":\n")
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .flat_map(|f| f.split('\n').collect::<Vec<_>>())
                .map(|x| {
                    x.split(' ')
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|f| f.parse::<i128>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            a.sort_by(|a, b| a[1].cmp(&b[1]));
            a
        })
        .collect::<Vec<_>>();

    let mut ranges = seeds_ranges.clone();
    for section in sections.iter() {
        let mut new_ranges: Vec<Range> = vec![];

        for range in ranges.iter() {
            let mut curr = range.clone();
            for x in section.iter() {
                let offset = x[0] - x[1];
                if curr.0 <= curr.1 && curr.0 < x[1] + x[2] && x[1] <= curr.1 {
                    if curr.0 < x[1] {
                        new_ranges.push(Range(curr.0, x[1] - 1));
                        curr.0 = x[1];
                        if curr.1 < x[1] + x[2] {
                            new_ranges.push(Range(curr.0 + offset, curr.1 + offset));
                            curr.0 = curr.1 + 1;
                        } else {
                            new_ranges.push(Range(curr.0 + offset, x[1] + x[2] - 1 + offset));
                            curr.0 = x[1] + x[2];
                        }
                    } else if curr.1 < x[1] + x[2] {
                        new_ranges.push(Range(curr.0 + offset, curr.1 + offset));
                        curr.0 = curr.1 + 1;
                    } else {
                        new_ranges.push(Range(curr.0 + offset, x[1] + x[2] - 1 + offset));
                        curr.0 = x[1] + x[2];
                    }
                }
            }
            if curr.0 <= curr.1 {
                new_ranges.push(curr);
            }
        }
        ranges = new_ranges;
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let lowest_location = ranges.first().unwrap().0.to_string();
    println!("Lowest location number: {}", lowest_location);
}

fn main() {
    run();
}
