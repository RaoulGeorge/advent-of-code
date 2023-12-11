use std::iter::repeat;

pub fn run() {
    let map = include_bytes!("./input.txt");
    let w = map.iter().position(|&b| b == b'\n').unwrap();
    let s = map.iter().position(|&b| b == b'S').unwrap();

    let (mut pos, mut dir) = {
        if matches!(map[s - w - 1], b'|' | b'7' | b'F') {
            (s - w - 1, 0)
        } else if matches!(map[s + w + 1], b'|' | b'L' | b'J') {
            (s + w + 1, 2)
        } else {
            (s - 1, 3)
        }
    };

    let res = (1 + repeat(())
        .position(|_| unsafe {
            match (map.get_unchecked(pos), dir) {
                (b'|', 0) => pos -= w + 1,
                (b'|', 2) => pos += w + 1,
                (b'-', 1) => pos += 1,
                (b'-', 3) => pos -= 1,
                (b'L', 2) | (b'F', 0) => {
                    pos += 1;
                    dir = 1;
                }
                (b'L', 3) | (b'J', 1) => {
                    pos -= w + 1;
                    dir = 0;
                }
                (b'7', 0) | (b'J', 2) => {
                    pos -= 1;
                    dir = 3;
                }
                (b'7', 1) | (b'F', 3) => {
                    pos += w + 1;
                    dir = 2;
                }
                (b'S', _) => return true,
                (_, _) => unreachable!(),
            }
            false
        })
        .unwrap())
        / 2;

    println!("{}", res);
}
