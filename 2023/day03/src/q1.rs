use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("./input.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();

    let is_symbol = |c: char| !c.is_digit(10) && c != '.';
    let is_valid_coordinate = |row: isize, col: isize| 0 <= row && row < h as isize && 0 <= col && col < w as isize;

    let mut res = 0;
    for row in 0..h {
        let mut col = 0;
        while col < w {
            if grid[row][col].is_digit(10) {
                let mut number = 0;
                let start_col = col;
                
                while col < w && grid[row][col].is_digit(10) {
                    number = number * 10 + (grid[row][col] as u8 - b'0') as i32;
                    col += 1;
                }
                let end_col = col - 1;

                let mut is_adjacent_to_symbol = false;
                for k in (start_col as isize - 1)..=(end_col as isize + 1) {
                    for ii in (row as isize - 1)..=(row as isize + 1) {
                        if is_valid_coordinate(ii, k) && is_symbol(grid[ii as usize][k as usize]) {
                            is_adjacent_to_symbol = true;
                            break;
                        }
                    }
                    if is_adjacent_to_symbol {
                        break;
                    }
                }
                
                if is_adjacent_to_symbol {
                    res += number;
                }
            } else {
                col += 1;
            }
        }
    }

    println!("{}", res);
}