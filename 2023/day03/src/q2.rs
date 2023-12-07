use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("./input.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();
    
    let is_symbol = |c: char| !c.is_digit(10) && c != '.';
    let is_valid_coordinate = |row: isize, col: isize| 0 <= row && row < h as isize && 0 <= col && col < w as isize;

    let mut adjacent_numbers = vec![vec![vec![]; w]; h];

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

                for k in (start_col as isize - 1)..=(end_col as isize + 1) {
                    for ii in (row as isize - 1)..=(row as isize + 1) {
                        if is_valid_coordinate(ii, k) && is_symbol(grid[ii as usize][k as usize]) {
                            adjacent_numbers[ii as usize][k as usize].push(number);
                        }
                    }
                }
            } else {
                col += 1;
            }
        }
    }

    for row in 0..h {
        for col in 0..w {
            if grid[row][col] == '*' && adjacent_numbers[row][col].len() == 2 {
                res += adjacent_numbers[row][col][0] * adjacent_numbers[row][col][1];
            }
        }
    }

    println!("{}", res);
}