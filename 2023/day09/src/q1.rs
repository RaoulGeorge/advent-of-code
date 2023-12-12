use smallvec::{smallvec, SmallVec};

const MAX: usize = 21;

pub fn run() {
    let nums = include_bytes!("./input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            line.split(|b| b == &b' ')
                .map(|b| atoi::atoi(b).unwrap())
                .collect::<SmallVec<[_; 21]>>()
        })
        .collect::<SmallVec<[_; 200]>>();

    let mut t: SmallVec<[SmallVec<[i64; MAX + 1]>; MAX]> = smallvec![smallvec![1]];
    for i in 0..MAX {
        let mut next = smallvec![1];
        next.extend(t[i].windows(2).map(|w| w[0] + w[1]).chain([1]));
        t.push(next);
    }
    (0..=MAX)
        .flat_map(|row| (0..=row).step_by(2).map(move |col| (row, col)))
        .for_each(|(row, col)| t[row][col] *= -1);

    let res = nums
        .into_iter()
        .map(|nums| {
            let row = nums.len();
            nums.iter()
                .enumerate()
                .map(|(col, n)| t[row][col] * n)
                .sum::<i64>()
                * if row % 2 == 0 { 1 } else { -1 }
        })
        .sum::<i64>();

    println!("{}", res);
}
